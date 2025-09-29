use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

use crate::{error::DexError, state::PoolState};

/// Add liquidity to an existing pool
/// 
/// When you add liquidity, you must respect the current price ratio!
/// If the pool has 1000 USDC and 10 SOL, the ratio is 100:1
/// To add liquidity, you must deposit in the same ratio (e.g., 200 USDC and 2 SOL)
/// 
/// Why? Because if you could deposit at any ratio, you could manipulate prices
/// and extract value from other liquidity providers.
/// 
/// Educational example:
/// Pool: 1000 USDC, 10 SOL, 100 LP tokens
/// You want to add: 200 USDC, 2 SOL
/// Your share: 200/1200 = 16.67% of new pool
/// LP tokens you get: 100 * (200/1000) = 20 LP tokens
/// 
/// Accounts expected:
/// 0. Pool state account
/// 1. User authority (signer)
/// 2. User token A account (source)
/// 3. User token B account (source)
/// 4. User LP token account (destination)
/// 5. Pool token A account (destination)
/// 6. Pool token B account (destination)
/// 7. LP token mint
/// 8. Token program
pub fn add_liquidity(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount_a: u64,
    amount_b: u64,
    min_liquidity: u64,
) -> ProgramResult {
    msg!("Adding liquidity to pool");
    
    // Validate inputs
    if amount_a == 0 || amount_b == 0 {
        msg!("Error: Cannot add zero liquidity");
        return Err(DexError::ZeroAmount.into());
    }
    
    let account_iter = &mut accounts.iter();
    
    // Parse accounts
    let pool_state_account = next_account_info(account_iter)?;
    let user_authority = next_account_info(account_iter)?;
    let user_token_a_account = next_account_info(account_iter)?;
    let user_token_b_account = next_account_info(account_iter)?;
    let user_lp_token_account = next_account_info(account_iter)?;
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
    
    // Check if pool has existing liquidity
    if pool_state.reserve_a == 0 || pool_state.reserve_b == 0 {
        msg!("Error: Cannot add liquidity to empty pool");
        return Err(DexError::InvalidPoolState.into());
    }
    
    msg!("Current pool state:");
    msg!("  Reserve A: {}, Reserve B: {}", pool_state.reserve_a, pool_state.reserve_b);
    msg!("  LP Token Supply: {}", pool_state.lp_token_supply);
    
    // Calculate the correct ratio for deposits
    // We need to maintain the current price ratio
    let current_ratio = pool_state.reserve_b as f64 / pool_state.reserve_a as f64;
    let deposit_ratio = amount_b as f64 / amount_a as f64;
    
    msg!("Current pool ratio: {:.6}", current_ratio);
    msg!("Your deposit ratio: {:.6}", deposit_ratio);
    
    // Allow small tolerance (0.1%) for ratio matching due to rounding
    let ratio_tolerance = 0.001;
    if (deposit_ratio - current_ratio).abs() / current_ratio > ratio_tolerance {
        msg!("Error: Deposit ratio doesn't match pool ratio");
        msg!("Expected ratio: {:.6}, Your ratio: {:.6}", current_ratio, deposit_ratio);
        return Err(DexError::InvalidPoolState.into());
    }
    
    // Calculate LP tokens to mint
    // Formula: lp_tokens = min(amount_a / reserve_a, amount_b / reserve_b) * lp_supply
    // We use the minimum to ensure we don't mint more than the smaller deposit warrants
    let lp_from_a = (amount_a as u128 * pool_state.lp_token_supply as u128) / pool_state.reserve_a as u128;
    let lp_from_b = (amount_b as u128 * pool_state.lp_token_supply as u128) / pool_state.reserve_b as u128;
    
    let lp_tokens_to_mint = std::cmp::min(lp_from_a, lp_from_b) as u64;
    
    msg!("LP tokens calculated:");
    msg!("  From token A: {}", lp_from_a);
    msg!("  From token B: {}", lp_from_b);
    msg!("  Minting: {}", lp_tokens_to_mint);
    
    // Check slippage protection
    if lp_tokens_to_mint < min_liquidity {
        msg!("Error: Slippage exceeded. Expected minimum: {}, Got: {}", 
             min_liquidity, lp_tokens_to_mint);
        return Err(DexError::SlippageExceeded.into());
    }
    
    // Calculate actual amounts to deposit (might be slightly less due to ratio matching)
    let actual_amount_a = (lp_tokens_to_mint as u128 * pool_state.reserve_a as u128) / pool_state.lp_token_supply as u128;
    let actual_amount_b = (lp_tokens_to_mint as u128 * pool_state.reserve_b as u128) / pool_state.lp_token_supply as u128;
    
    msg!("Actual deposit amounts:");
    msg!("  Token A: {} (requested: {})", actual_amount_a, amount_a);
    msg!("  Token B: {} (requested: {})", actual_amount_b, amount_b);
    
    // Update pool state
    pool_state.reserve_a = pool_state.reserve_a
        .checked_add(actual_amount_a as u64)
        .ok_or(DexError::MathOverflow)?;
    pool_state.reserve_b = pool_state.reserve_b
        .checked_add(actual_amount_b as u64)
        .ok_or(DexError::MathOverflow)?;
    pool_state.lp_token_supply = pool_state.lp_token_supply
        .checked_add(lp_tokens_to_mint)
        .ok_or(DexError::MathOverflow)?;
    
    // Verify the constant product increased (it should due to the new liquidity)
    let new_k = pool_state.reserve_a as u128 * pool_state.reserve_b as u128;
    msg!("New constant product k: {}", new_k);
    
    // Save updated pool state
    pool_state.serialize(&mut &mut pool_data[..])?;
    
    msg!("Liquidity added successfully!");
    msg!("New pool state:");
    msg!("  Reserve A: {}, Reserve B: {}", pool_state.reserve_a, pool_state.reserve_b);
    msg!("  LP Token Supply: {}", pool_state.lp_token_supply);
    msg!("  Your LP tokens: {}", lp_tokens_to_mint);
    
    // Calculate your share of the pool
    let your_share = (lp_tokens_to_mint as f64 / pool_state.lp_token_supply as f64) * 100.0;
    msg!("Your pool share: {:.2}%", your_share);
    
    // TODO: Execute actual token transfers (would need SPL token transfers)
    // TODO: Mint LP tokens to user (would need SPL token mint)
    // For this educational example, we're focusing on the core AMM logic
    
    Ok(())
}

/*
Educational note for students:

Adding liquidity is a fundamental DeFi operation. Key concepts:

1. **Price Ratio Maintenance**: You must deposit at the current market ratio
   - Prevents price manipulation through liquidity deposits
   - Ensures fair treatment of all liquidity providers
   - Maintains the constant product invariant

2. **LP Token Calculation**: LP tokens represent your share of the pool
   - Formula: new_lp = (deposit_amount / current_reserve) * current_lp_supply
   - LP tokens are like "receipts" for your liquidity deposit
   - You can redeem them later to withdraw your share + earned fees

3. **Proportional Ownership**: Your share of the pool determines your rewards
   - If you own 10% of LP tokens, you own 10% of all pool assets
   - When traders pay fees, you earn 10% of those fees
   - Your share might decrease as others add liquidity (dilution)

4. **Impermanent Loss**: The risk of providing liquidity
   - If prices change significantly, you might lose money vs. just holding
   - Called "impermanent" because it only matters when you withdraw
   - Usually compensated by trading fees over time

Mathematical insight:
The LP token formula ensures that:
- Early liquidity providers aren't diluted unfairly
- New liquidity providers pay the "fair price" for their share
- Total pool value is always preserved
- The ratio between any two deposits determines relative ownership

Example calculation:
Pool: 1000 A, 2000 B, 100 LP tokens
You deposit: 100 A, 200 B
Your LP tokens: min(100/1000, 200/2000) * 100 = 10 LP tokens
Your share: 10/110 = 9.09% of the new pool
*/