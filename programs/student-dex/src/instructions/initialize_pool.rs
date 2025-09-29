use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use borsh::BorshSerialize;

use crate::{error::DexError, state::PoolState};

/// Initialize a new liquidity pool
/// 
/// This is the most important function for understanding AMMs!
/// The first liquidity provider essentially "sets the price" by choosing
/// the initial ratio of tokens to deposit.
/// 
/// Educational example:
/// If you initialize with 100 USDC and 1 SOL, you're saying:
/// 1 SOL = 100 USDC (initial price)
/// 
/// Accounts expected:
/// 0. Pool state account (to be initialized)
/// 1. Pool authority (signer)
/// 2. Token A mint
/// 3. Token B mint  
/// 4. Pool's token A account
/// 5. Pool's token B account
/// 6. LP token mint
/// 7. User's token A account (source)
/// 8. User's token B account (source)
/// 9. User's LP token account (destination)
/// 10. Token program
/// 11. System program
/// 12. Rent sysvar
pub fn initialize_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    initial_amount_a: u64,
    initial_amount_b: u64,
) -> ProgramResult {
    msg!("Initializing new liquidity pool");
    
    // Validate inputs
    if initial_amount_a == 0 || initial_amount_b == 0 {
        msg!("Error: Cannot initialize pool with zero amounts");
        return Err(DexError::ZeroAmount.into());
    }
    
    let account_iter = &mut accounts.iter();
    
    // Parse accounts
    let pool_state_account = next_account_info(account_iter)?;
    let pool_authority = next_account_info(account_iter)?;
    let token_a_mint = next_account_info(account_iter)?;
    let token_b_mint = next_account_info(account_iter)?;
    let pool_token_a_account = next_account_info(account_iter)?;
    let pool_token_b_account = next_account_info(account_iter)?;
    let lp_token_mint = next_account_info(account_iter)?;
    let user_token_a_account = next_account_info(account_iter)?;
    let user_token_b_account = next_account_info(account_iter)?;
    let user_lp_token_account = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;
    let rent_sysvar = next_account_info(account_iter)?;
    
    // Verify the pool authority is a signer
    if !pool_authority.is_signer {
        msg!("Error: Pool authority must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Verify the pool state account is owned by our program
    if pool_state_account.owner != program_id {
        msg!("Error: Pool state account not owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // Check if account has enough space and is rent-exempt
    let rent = Rent::from_account_info(rent_sysvar)?;
    if !rent.is_exempt(pool_state_account.lamports(), PoolState::LEN) {
        msg!("Error: Pool state account is not rent-exempt");
        return Err(ProgramError::AccountNotRentExempt);
    }
    
    // Initialize the pool state
    let mut pool_state = PoolState::new();
    
    // Check if already initialized
    if pool_state.is_initialized {
        msg!("Error: Pool already initialized");
        return Err(DexError::PoolAlreadyInitialized.into());
    }
    
    // Set pool parameters
    pool_state.is_initialized = true;
    pool_state.admin = *pool_authority.key;
    pool_state.token_a_mint = *token_a_mint.key;
    pool_state.token_b_mint = *token_b_mint.key;
    pool_state.token_a_account = *pool_token_a_account.key;
    pool_state.token_b_account = *pool_token_b_account.key;
    pool_state.lp_token_mint = *lp_token_mint.key;
    pool_state.reserve_a = initial_amount_a;
    pool_state.reserve_b = initial_amount_b;
    
    // Calculate initial LP tokens using geometric mean
    // This is a common approach: sqrt(amount_a * amount_b)
    // This ensures the initial LP tokens represent the "value" of both deposits
    let initial_liquidity = (initial_amount_a as u128 * initial_amount_b as u128)
        .integer_sqrt() as u64;
    
    if initial_liquidity == 0 {
        msg!("Error: Initial liquidity calculation resulted in zero");
        return Err(DexError::InsufficientLiquidity.into());
    }
    
    pool_state.lp_token_supply = initial_liquidity;
    
    // Save the pool state
    let mut pool_data = pool_state_account.try_borrow_mut_data()?;
    pool_state.serialize(&mut &mut pool_data[..])?;
    
    msg!("Pool initialized successfully!");
    msg!("Initial reserves: {} token A, {} token B", initial_amount_a, initial_amount_b);
    msg!("Initial LP tokens minted: {}", initial_liquidity);
    msg!("Initial price: {} token B per token A", initial_amount_b as f64 / initial_amount_a as f64);
    
    // TODO: Transfer tokens from user to pool (would need SPL token transfers)
    // TODO: Mint LP tokens to user (would need SPL token mint)
    // For this educational example, we're focusing on the core AMM logic
    
    Ok(())
}

// Helper trait for integer square root
trait IntegerSquareRoot {
    fn integer_sqrt(self) -> Self;
}

impl IntegerSquareRoot for u128 {
    fn integer_sqrt(self) -> Self {
        if self == 0 {
            return 0;
        }
        
        let mut x = self;
        let mut y = (self + 1) / 2;
        
        while y < x {
            x = y;
            y = (x + self / x) / 2;
        }
        
        x
    }
}

/*
Educational note for students:

Pool initialization is critical because:

1. **Price Setting**: The first depositor sets the initial exchange rate
2. **Liquidity Foundation**: Creates the base liquidity for all future trades
3. **LP Token Calculation**: Uses sqrt(x * y) to ensure fair representation

Key formulas:
- Initial price: price = amount_b / amount_a
- Initial LP tokens: lp = sqrt(amount_a * amount_b)
- Constant product: k = amount_a * amount_b (this must be maintained)

Why sqrt(x * y) for LP tokens?
- It's proportional to the geometric mean of deposits
- It grows slower than either individual deposit
- It ensures that the LP token value represents both assets fairly
- It's the standard used by Uniswap and most AMMs

Security considerations:
- First depositor has significant power over initial price
- Small initial deposits can be manipulated easily
- Always verify token mint addresses to avoid fake tokens
*/