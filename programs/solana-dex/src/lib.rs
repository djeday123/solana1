use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};
use borsh::{BorshDeserialize, BorshSerialize};

// Program entrypoint
entrypoint!(process_instruction);

// Program ID (this would be generated when deploying)
solana_program::declare_id!("DEX1111111111111111111111111111111111111111");

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum DexInstruction {
    /// Initialize a new liquidity pool
    /// Accounts expected:
    /// 0. [signer] Initializer
    /// 1. [writable] Pool state account
    /// 2. [] Token A mint
    /// 3. [] Token B mint
    InitializePool {
        initial_liquidity_a: u64,
        initial_liquidity_b: u64,
    },
    
    /// Swap tokens (simplified version for educational purposes)
    /// Accounts expected:
    /// 0. [signer] User
    /// 1. [writable] Pool state
    Swap {
        amount_in: u64,
        minimum_amount_out: u64,
        swap_a_to_b: bool, // true = A to B, false = B to A
    },
    
    /// Add liquidity to pool
    AddLiquidity {
        amount_a: u64,
        amount_b: u64,
        minimum_liquidity_tokens: u64,
    },
    
    /// Remove liquidity from pool
    RemoveLiquidity {
        liquidity_tokens: u64,
        minimum_amount_a: u64,
        minimum_amount_b: u64,
    },
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PoolState {
    pub is_initialized: bool,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_reserve: u64,
    pub token_b_reserve: u64,
    pub fee_numerator: u64,
    pub fee_denominator: u64,
    pub total_liquidity_tokens: u64,
}

impl Default for PoolState {
    fn default() -> Self {
        Self {
            is_initialized: false,
            token_a_mint: Pubkey::default(),
            token_b_mint: Pubkey::default(),
            token_a_reserve: 0,
            token_b_reserve: 0,
            fee_numerator: 3,
            fee_denominator: 1000,
            total_liquidity_tokens: 0,
        }
    }
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = DexInstruction::try_from_slice(instruction_data)?;
    
    match instruction {
        DexInstruction::InitializePool {
            initial_liquidity_a,
            initial_liquidity_b,
        } => {
            msg!("Instruction: InitializePool");
            process_initialize_pool(accounts, initial_liquidity_a, initial_liquidity_b, program_id)
        }
        DexInstruction::Swap {
            amount_in,
            minimum_amount_out,
            swap_a_to_b,
        } => {
            msg!("Instruction: Swap");
            process_swap(accounts, amount_in, minimum_amount_out, swap_a_to_b)
        }
        DexInstruction::AddLiquidity {
            amount_a,
            amount_b,
            minimum_liquidity_tokens,
        } => {
            msg!("Instruction: AddLiquidity");
            process_add_liquidity(accounts, amount_a, amount_b, minimum_liquidity_tokens)
        }
        DexInstruction::RemoveLiquidity {
            liquidity_tokens,
            minimum_amount_a,
            minimum_amount_b,
        } => {
            msg!("Instruction: RemoveLiquidity");
            process_remove_liquidity(accounts, liquidity_tokens, minimum_amount_a, minimum_amount_b)
        }
    }
}

pub fn process_initialize_pool(
    accounts: &[AccountInfo],
    initial_liquidity_a: u64,
    initial_liquidity_b: u64,
    _program_id: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let pool_state = next_account_info(account_info_iter)?;
    let token_a_mint = next_account_info(account_info_iter)?;
    let token_b_mint = next_account_info(account_info_iter)?;
    
    if !initializer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Initialize pool state (simplified for educational purposes)
    let mut pool_data = PoolState::default();
    
    if pool_data.is_initialized {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    
    pool_data.is_initialized = true;
    pool_data.token_a_mint = *token_a_mint.key;
    pool_data.token_b_mint = *token_b_mint.key;
    pool_data.token_a_reserve = initial_liquidity_a;
    pool_data.token_b_reserve = initial_liquidity_b;
    pool_data.fee_numerator = 3; // 0.3% fee
    pool_data.fee_denominator = 1000;
    pool_data.total_liquidity_tokens = (initial_liquidity_a * initial_liquidity_b).integer_sqrt();
    
    // Serialize the pool state back to the account
    pool_data.serialize(&mut &mut pool_state.data.borrow_mut()[..])?;
    
    msg!("Pool initialized with {} token A and {} token B", initial_liquidity_a, initial_liquidity_b);
    
    Ok(())
}

pub fn process_swap(
    accounts: &[AccountInfo],
    amount_in: u64,
    minimum_amount_out: u64,
    swap_a_to_b: bool,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user = next_account_info(account_info_iter)?;
    let pool_state = next_account_info(account_info_iter)?;
    
    if !user.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    let mut pool_data = PoolState::try_from_slice(&pool_state.data.borrow())?;
    
    if !pool_data.is_initialized {
        return Err(ProgramError::UninitializedAccount);
    }
    
    // Calculate swap output using constant product formula: x * y = k
    let (amount_out, _fee) = if swap_a_to_b {
        calculate_swap_amount(amount_in, pool_data.token_a_reserve, pool_data.token_b_reserve, &pool_data)?
    } else {
        calculate_swap_amount(amount_in, pool_data.token_b_reserve, pool_data.token_a_reserve, &pool_data)?
    };
    
    if amount_out < minimum_amount_out {
        msg!("Slippage exceeded: expected {}, got {}", minimum_amount_out, amount_out);
        return Err(ProgramError::Custom(0)); // Slippage exceeded
    }
    
    // Update pool reserves (in a real implementation, this would involve actual token transfers)
    if swap_a_to_b {
        pool_data.token_a_reserve = pool_data.token_a_reserve.checked_add(amount_in).ok_or(ProgramError::ArithmeticOverflow)?;
        pool_data.token_b_reserve = pool_data.token_b_reserve.checked_sub(amount_out).ok_or(ProgramError::ArithmeticOverflow)?;
    } else {
        pool_data.token_b_reserve = pool_data.token_b_reserve.checked_add(amount_in).ok_or(ProgramError::ArithmeticOverflow)?;
        pool_data.token_a_reserve = pool_data.token_a_reserve.checked_sub(amount_out).ok_or(ProgramError::ArithmeticOverflow)?;
    }
    
    // Serialize updated pool state
    pool_data.serialize(&mut &mut pool_state.data.borrow_mut()[..])?;
    
    msg!("Swap completed: {} in, {} out", amount_in, amount_out);
    
    Ok(())
}

pub fn process_add_liquidity(
    _accounts: &[AccountInfo],
    amount_a: u64,
    amount_b: u64,
    _minimum_liquidity_tokens: u64,
) -> ProgramResult {
    msg!("Adding liquidity: {} token A, {} token B", amount_a, amount_b);
    // Simplified implementation for educational purposes
    // In a real DEX, this would:
    // 1. Check token balances
    // 2. Calculate optimal ratio
    // 3. Transfer tokens to pool
    // 4. Mint LP tokens to user
    Ok(())
}

pub fn process_remove_liquidity(
    _accounts: &[AccountInfo],
    liquidity_tokens: u64,
    _minimum_amount_a: u64,
    _minimum_amount_b: u64,
) -> ProgramResult {
    msg!("Removing liquidity: {} tokens", liquidity_tokens);
    // Simplified implementation for educational purposes
    // In a real DEX, this would:
    // 1. Burn LP tokens
    // 2. Calculate proportional amounts
    // 3. Transfer tokens back to user
    Ok(())
}

fn calculate_swap_amount(
    amount_in: u64,
    reserve_in: u64,
    reserve_out: u64,
    pool_data: &PoolState,
) -> Result<(u64, u64), ProgramError> {
    if amount_in == 0 || reserve_in == 0 || reserve_out == 0 {
        return Err(ProgramError::InvalidArgument);
    }
    
    // Calculate fee
    let fee = amount_in
        .checked_mul(pool_data.fee_numerator)
        .ok_or(ProgramError::ArithmeticOverflow)?
        .checked_div(pool_data.fee_denominator)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    
    let amount_in_with_fee = amount_in
        .checked_sub(fee)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    
    // Constant product formula: (x + dx) * (y - dy) = x * y
    // dy = y * dx / (x + dx)
    let numerator = reserve_out
        .checked_mul(amount_in_with_fee)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    
    let denominator = reserve_in
        .checked_add(amount_in_with_fee)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    
    let amount_out = numerator
        .checked_div(denominator)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    
    Ok((amount_out, fee))
}

// Helper trait for integer square root
trait IntegerSquareRoot {
    fn integer_sqrt(self) -> Self;
}

impl IntegerSquareRoot for u64 {
    fn integer_sqrt(self) -> Self {
        if self < 2 {
            return self;
        }
        
        let mut x = self;
        let mut y = (x + 1) / 2;
        
        while y < x {
            x = y;
            y = (x + self / x) / 2;
        }
        
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_swap_calculation() {
        let pool_data = PoolState {
            is_initialized: true,
            token_a_mint: Pubkey::default(),
            token_b_mint: Pubkey::default(),
            token_a_reserve: 1000,
            token_b_reserve: 1000,
            fee_numerator: 3,
            fee_denominator: 1000,
            total_liquidity_tokens: 1000,
        };
        
        let (amount_out, fee) = calculate_swap_amount(100, 1000, 1000, &pool_data).unwrap();
        
        println!("Input: 100, Output: {}, Fee: {}", amount_out, fee);
        
        // With 0.3% fee: 100 * 3 / 1000 = 0 (integer division)
        // So amount_in_with_fee = 100 - 0 = 100
        // Amount out: 1000 * 100 / (1000 + 100) = 100000 / 1100 = 90.909... ≈ 90
        assert_eq!(fee, 0); // 100 * 3 / 1000 = 0 (integer division)
        assert_eq!(amount_out, 90); // 100000 / 1100 = 90 (integer division)
    }
    
    #[test]
    fn test_integer_sqrt() {
        assert_eq!(0u64.integer_sqrt(), 0);
        assert_eq!(1u64.integer_sqrt(), 1);
        assert_eq!(4u64.integer_sqrt(), 2);
        assert_eq!(9u64.integer_sqrt(), 3);
        assert_eq!(15u64.integer_sqrt(), 3);
        assert_eq!(16u64.integer_sqrt(), 4);
    }
}
