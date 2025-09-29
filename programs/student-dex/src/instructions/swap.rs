use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

use crate::{error::DexError, state::PoolState};

/// Swap tokens using the constant product formula
/// 
/// This is the core of any AMM! Understanding this function means you understand
/// how decentralized exchanges work.
/// 
/// The constant product formula: x * y = k
/// - x = reserve of input token
/// - y = reserve of output token  
/// - k = constant (must remain the same after swap + fees)
/// 
/// Educational example:
/// Pool has 1000 USDC and 10 SOL (k = 10,000)
/// User wants to swap 100 USDC for SOL
/// New USDC reserve = 1000 + 100 = 1100
/// New SOL reserve = 10,000 / 1100 = 9.09 SOL
/// User receives = 10 - 9.09 = 0.91 SOL
/// 
/// Accounts expected:
/// 0. Pool state account
/// 1. User authority (signer)
/// 2. User input token account (source)
/// 3. User output token account (destination)  
/// 4. Pool input token account
/// 5. Pool output token account
/// 6. Token program
pub fn swap_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount_in: u64,
    minimum_amount_out: u64,
) -> ProgramResult {
    msg!("Processing token swap");
    
    // Validate inputs
    if amount_in == 0 {
        msg!("Error: Cannot swap zero tokens");
        return Err(DexError::ZeroAmount.into());
    }
    
    let account_iter = &mut accounts.iter();
    
    // Parse accounts
    let pool_state_account = next_account_info(account_iter)?;
    let user_authority = next_account_info(account_iter)?;
    let user_input_token_account = next_account_info(account_iter)?;
    let user_output_token_account = next_account_info(account_iter)?;
    let pool_input_token_account = next_account_info(account_iter)?;
    let pool_output_token_account = next_account_info(account_iter)?;
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
    
    // Determine swap direction (A -> B or B -> A)
    let (input_reserve, output_reserve, is_a_to_b) = 
        if *pool_input_token_account.key == pool_state.token_a_account {
            // Swapping A for B
            (pool_state.reserve_a, pool_state.reserve_b, true)
        } else if *pool_input_token_account.key == pool_state.token_b_account {
            // Swapping B for A  
            (pool_state.reserve_b, pool_state.reserve_a, false)
        } else {
            msg!("Error: Invalid token accounts");
            return Err(DexError::InvalidTokenAccounts.into());
        };
    
    msg!("Swap direction: {} to {}", 
         if is_a_to_b { "A" } else { "B" },
         if is_a_to_b { "B" } else { "A" });
    
    // Calculate swap output using constant product formula
    let amount_out = pool_state.calculate_swap_output(
        amount_in,
        input_reserve,
        output_reserve,
    ).map_err(|_| DexError::MathOverflow)?;
    
    msg!("Calculated swap: {} in -> {} out", amount_in, amount_out);
    
    // Check slippage protection
    if amount_out < minimum_amount_out {
        msg!("Error: Slippage exceeded. Expected minimum: {}, Got: {}", 
             minimum_amount_out, amount_out);
        return Err(DexError::SlippageExceeded.into());
    }
    
    // Check if pool has enough output tokens
    if amount_out >= output_reserve {
        msg!("Error: Insufficient liquidity in pool");
        return Err(DexError::InsufficientLiquidity.into());
    }
    
    // Calculate trading fee (fee goes to liquidity providers)
    let fee_amount = amount_in
        .checked_mul(pool_state.fee_rate as u64)
        .ok_or(DexError::MathOverflow)?
        .checked_div(10000)
        .ok_or(DexError::MathOverflow)?;
    
    msg!("Trading fee: {} tokens", fee_amount);
    
    // Update pool reserves and accumulated fees
    if is_a_to_b {
        // A -> B swap
        pool_state.reserve_a = pool_state.reserve_a
            .checked_add(amount_in)
            .ok_or(DexError::MathOverflow)?;
        pool_state.reserve_b = pool_state.reserve_b
            .checked_sub(amount_out)
            .ok_or(DexError::InsufficientLiquidity)?;
        pool_state.accumulated_fee_a = pool_state.accumulated_fee_a
            .checked_add(fee_amount)
            .ok_or(DexError::MathOverflow)?;
    } else {
        // B -> A swap
        pool_state.reserve_b = pool_state.reserve_b
            .checked_add(amount_in)
            .ok_or(DexError::MathOverflow)?;
        pool_state.reserve_a = pool_state.reserve_a
            .checked_sub(amount_out)
            .ok_or(DexError::InsufficientLiquidity)?;
        pool_state.accumulated_fee_b = pool_state.accumulated_fee_b
            .checked_add(fee_amount)
            .ok_or(DexError::MathOverflow)?;
    }
    
    // Verify constant product (k should increase due to fees)
    let new_k = pool_state.reserve_a as u128 * pool_state.reserve_b as u128;
    msg!("New constant product k: {}", new_k);
    
    // Save updated pool state
    pool_state.serialize(&mut &mut pool_data[..])?;
    
    msg!("Swap completed successfully!");
    msg!("New reserves: {} token A, {} token B", 
         pool_state.reserve_a, pool_state.reserve_b);
    
    // Calculate new prices for educational logging
    if let Ok(price_a_to_b) = pool_state.get_price_a_to_b() {
        msg!("New price A->B: {:.6}", price_a_to_b);
    }
    if let Ok(price_b_to_a) = pool_state.get_price_b_to_a() {
        msg!("New price B->A: {:.6}", price_b_to_a);
    }
    
    // TODO: Execute actual token transfers (would need SPL token transfers)
    // For this educational example, we're focusing on the core AMM math
    
    Ok(())
}

/*
Educational note for students:

The swap function implements the heart of an Automated Market Maker (AMM).
Here's what makes it work:

1. **Constant Product Formula**: x * y = k
   - Before swap: reserve_a * reserve_b = k
   - After swap: (reserve_a + input) * (reserve_b - output) = k (plus fees)
   - This creates a hyperbolic curve for price discovery

2. **Price Impact**: The more you swap, the more the price moves
   - Small swaps: minimal price change
   - Large swaps: significant price change (slippage)
   - This is natural market behavior!

3. **Trading Fees**: Usually 0.3% goes to liquidity providers
   - Incentivizes people to provide liquidity
   - Slightly increases k over time (protocol growth)
   - Helps prevent arbitrage bots from draining pools

4. **Slippage Protection**: minimum_amount_out prevents bad surprises
   - Price can change between transaction submission and execution
   - User sets maximum acceptable slippage
   - Transaction fails if slippage is too high

Mathematical insight:
The constant product creates a curve where:
- Price = derivative of the curve = -y/x
- Deeper pools = less slippage for same trade size
- Pool never runs out of tokens (approaches zero asymptotically)

This is different from order book exchanges where:
- You trade at specific prices set by other users
- You can run out of liquidity at specific price levels
- Price moves in discrete steps based on orders
*/