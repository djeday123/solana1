use solana_program::program_error::ProgramError;
use thiserror::Error;

/// Custom error types for our educational DEX
/// This helps students understand different error scenarios
#[derive(Error, Debug, Copy, Clone)]
pub enum DexError {
    #[error("Invalid instruction data provided")]
    InvalidInstruction,
    
    #[error("Pool already initialized")]
    PoolAlreadyInitialized,
    
    #[error("Pool not initialized")]
    PoolNotInitialized,
    
    #[error("Insufficient liquidity in pool")]
    InsufficientLiquidity,
    
    #[error("Slippage tolerance exceeded")]
    SlippageExceeded,
    
    #[error("Invalid token accounts")]
    InvalidTokenAccounts,
    
    #[error("Unauthorized access")]
    Unauthorized,
    
    #[error("Mathematical overflow occurred")]
    MathOverflow,
    
    #[error("Zero amount not allowed")]
    ZeroAmount,
    
    #[error("Invalid pool state")]
    InvalidPoolState,
    
    #[error("Insufficient user balance")]
    InsufficientBalance,
}

impl From<DexError> for ProgramError {
    fn from(e: DexError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

/*
Educational note for students:

Error handling is crucial in DeFi applications because:
1. **User funds are at stake** - Any bug could lead to loss of money
2. **Transparent feedback** - Users need to understand what went wrong
3. **Security** - Proper error handling prevents exploits

Common DEX error scenarios:
- Slippage: Price changed too much during the transaction
- Insufficient liquidity: Not enough tokens in the pool for the swap
- Math overflow: Numbers got too large for the system to handle
- Unauthorized access: Someone trying to access funds they don't own
*/