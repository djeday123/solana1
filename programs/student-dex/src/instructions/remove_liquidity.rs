use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

use crate::{error::DexError, state::PoolState};

/// Remove liquidity from a pool
/// 
/// When you remove liquidity, you burn your LP tokens and receive back
/// a proportional share of the current pool reserves.
/// 
/// This is where "impermanent loss" becomes "permanent" - if the price
/// ratio changed since you added liquidity, you'll get back different
/// amounts than you deposited.
/// 
/// Educational example:
/// Pool: 2000 USDC, 10 SOL, 200 LP tokens (price: 200 USDC/SOL)
/// You have: 20 LP tokens (10% of pool)
/// You remove all liquidity and get: 200 USDC, 1 SOL
/// If SOL price doubled since you deposited, you experience impermanent loss
/// 
/// Accounts expected:
/// 0. Pool state account
/// 1. User authority (signer)
/// 2. User LP token account (source)
/// 3. User token A account (destination)
/// 4. User token B account (destination)
/// 5. Pool token A account (source)
/// 6. Pool token B account (source)
/// 7. LP token mint
/// 8. Token program
pub fn remove_liquidity(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    liquidity_amount: u64,
    min_amount_a: u64,
    min_amount_b: u64,
) -> ProgramResult {
    msg!("Removing liquidity from pool");
    
    // Validate inputs
    if liquidity_amount == 0 {
        msg!("Error: Cannot remove zero liquidity");
        return Err(DexError::ZeroAmount.into());
    }
    
    let account_iter = &mut accounts.iter();
    
    // Parse accounts
    let pool_state_account = next_account_info(account_iter)?;
    let user_authority = next_account_info(account_iter)?;
    let user_lp_token_account = next_account_info(account_iter)?;
    let user_token_a_account = next_account_info(account_iter)?;
    let user_token_b_account = next_account_info(account_iter)?;
    let pool_token_a_account = next_account_info(account_iter)?;
    let pool_token_b_account = next_account_info(account_iter)?;
    let lp_token_mint = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    
    // Verify user authority is signer
    if !user_authority.is_signer {
        msg!("Error: User authority must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Load and validate pool state
    let mut pool_data = pool_state_account.try_borrow_mut_data()?;
    let mut pool_state = PoolState::try_from_slice(&pool_data)?;
    
    if !pool_state.is_initialized {
        msg!("Error: Pool not initialized");
        return Err(DexError::PoolNotInitialized.into());
    }
    
    // Check if user has enough LP tokens (this would be checked by SPL token program)
    if liquidity_amount > pool_state.lp_token_supply {
        msg!("Error: Cannot remove more liquidity than exists");
        return Err(DexError::InsufficientLiquidity.into());
    }
    
    msg!("Current pool state:");
    msg!("  Reserve A: {}, Reserve B: {}", pool_state.reserve_a, pool_state.reserve_b);
    msg!("  LP Token Supply: {}", pool_state.lp_token_supply);
    msg!("  LP tokens to burn: {}", liquidity_amount);
    
    // Calculate share of pool being removed
    let share_percentage = liquidity_amount as f64 / pool_state.lp_token_supply as f64;
    msg!("Your share of pool: {:.2}%", share_percentage * 100.0);
    
    // Calculate amounts to withdraw (proportional to LP token share)
    let amount_a_out = (pool_state.reserve_a as u128 * liquidity_amount as u128) / pool_state.lp_token_supply as u128;
    let amount_b_out = (pool_state.reserve_b as u128 * liquidity_amount as u128) / pool_state.lp_token_supply as u128;
    
    msg!("Tokens you'll receive:");
    msg!("  Token A: {}", amount_a_out);
    msg!("  Token B: {}", amount_b_out);
    
    // Check slippage protection
    if amount_a_out < min_amount_a as u128 {
        msg!("Error: Token A amount below minimum. Expected: {}, Got: {}", 
             min_amount_a, amount_a_out);
        return Err(DexError::SlippageExceeded.into());
    }
    
    if amount_b_out < min_amount_b as u128 {
        msg!("Error: Token B amount below minimum. Expected: {}, Got: {}", 
             min_amount_b, amount_b_out);
        return Err(DexError::SlippageExceeded.into());
    }
    
    // Ensure we don't withdraw more than available
    if amount_a_out > pool_state.reserve_a as u128 || amount_b_out > pool_state.reserve_b as u128 {
        msg!("Error: Cannot withdraw more than pool reserves");
        return Err(DexError::InsufficientLiquidity.into());
    }
    
    // Calculate current price for educational purposes
    let current_price_a_to_b = pool_state.reserve_b as f64 / pool_state.reserve_a as f64;
    msg!("Current price A->B: {:.6}", current_price_a_to_b);
    
    // Update pool state
    pool_state.reserve_a = pool_state.reserve_a
        .checked_sub(amount_a_out as u64)
        .ok_or(DexError::InsufficientLiquidity)?;
    pool_state.reserve_b = pool_state.reserve_b
        .checked_sub(amount_b_out as u64)
        .ok_or(DexError::InsufficientLiquidity)?;
    pool_state.lp_token_supply = pool_state.lp_token_supply
        .checked_sub(liquidity_amount)
        .ok_or(DexError::InsufficientLiquidity)?;
    
    // Check if this is the last withdrawal (pool becomes empty)
    if pool_state.lp_token_supply == 0 {
        msg!("Warning: Pool is now empty after this withdrawal");
        // In a real implementation, you might want to close the pool
        pool_state.reserve_a = 0;
        pool_state.reserve_b = 0;
    } else {
        // Verify the constant product is maintained (should be the same or slightly higher due to accumulated fees)
        let new_k = pool_state.reserve_a as u128 * pool_state.reserve_b as u128;
        msg!("New constant product k: {}", new_k);
        
        // Calculate new price after withdrawal
        if pool_state.reserve_a > 0 && pool_state.reserve_b > 0 {
            let new_price_a_to_b = pool_state.reserve_b as f64 / pool_state.reserve_a as f64;
            msg!("New price A->B: {:.6}", new_price_a_to_b);
            
            // Price should remain the same for proportional withdrawals
            let price_change = ((new_price_a_to_b - current_price_a_to_b) / current_price_a_to_b).abs();
            if price_change > 0.0001 { // 0.01% tolerance
                msg!("Warning: Price changed by {:.4}% after withdrawal", price_change * 100.0);
            }
        }
    }
    
    // Save updated pool state
    pool_state.serialize(&mut &mut pool_data[..])?;
    
    msg!("Liquidity removed successfully!");
    msg!("New pool state:");
    msg!("  Reserve A: {}, Reserve B: {}", pool_state.reserve_a, pool_state.reserve_b);
    msg!("  LP Token Supply: {}", pool_state.lp_token_supply);
    
    // Educational calculation: show what happened to their investment
    if pool_state.lp_token_supply > 0 {
        let remaining_share = 0.0; // They withdrew everything in this example
        msg!("Your remaining pool share: {:.2}%", remaining_share);
    }
    
    // TODO: Execute actual token transfers (would need SPL token transfers)
    // TODO: Burn LP tokens from user (would need SPL token burn)
    // For this educational example, we're focusing on the core AMM logic
    
    Ok(())
}

/*
Educational note for students:

Removing liquidity is the "exit" from providing liquidity. Key concepts:

1. **Proportional Withdrawal**: You get back your exact share of current reserves
   - If you own 10% of LP tokens, you get 10% of each token in the pool
   - Your share percentage stays the same regardless of absolute amounts
   - This ensures fairness - no one can withdraw more than their fair share

2. **Impermanent Loss Realization**: This is where you see if you gained or lost
   - If token prices returned to original ratio: no impermanent loss
   - If one token outperformed: you have less of the winning token
   - Formula: IL = 2*sqrt(price_ratio) / (1 + price_ratio) - 1

3. **Fee Accumulation**: You also get your share of accumulated trading fees
   - Pools generally accumulate fees over time from swaps
   - These fees can offset impermanent loss
   - Longer-term liquidity provision usually more profitable

4. **Slippage Protection**: min_amount parameters protect against MEV attacks
   - Between signing transaction and execution, someone could manipulate pool
   - Minimum amounts ensure you get at least what you expect
   - Similar to swap slippage protection

Mathematical insight:
The withdrawal formula is simple but powerful:
- amount_out = (lp_tokens_burned / total_lp_supply) * current_reserves
- This ensures the constant product formula is maintained
- It's automatically "fair" - no complex calculations needed

Example scenario analysis:
Initial deposit: 1000 USDC + 5 SOL (SOL = $200)
Current pool: 1100 USDC + 4.5 SOL (SOL = $244)
Your withdrawal: 550 USDC + 2.25 SOL = $549 + $549 = $1098
vs holding: 1000 USDC + 5 SOL = $1000 + $1220 = $2220
Impermanent loss: $1098 - $2220 = -$1122 (50.5% loss!)

This shows why impermanent loss can be significant during price changes.
*/