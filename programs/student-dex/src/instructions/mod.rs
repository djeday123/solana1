use borsh::{BorshDeserialize, BorshSerialize};

pub mod initialize_pool;
pub mod add_liquidity;
pub mod remove_liquidity;
pub mod swap;

pub use initialize_pool::initialize_pool;
pub use add_liquidity::add_liquidity;
pub use remove_liquidity::remove_liquidity;
pub use swap::swap_tokens;

/// All possible instructions that can be sent to our DEX program
/// This enum defines the API of our decentralized exchange
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum DexInstruction {
    /// Create a new liquidity pool for two tokens
    /// 
    /// Arguments:
    /// - initial_amount_a: Amount of token A to deposit initially
    /// - initial_amount_b: Amount of token B to deposit initially
    /// 
    /// Educational note: The first liquidity provider sets the initial price!
    /// Price ratio = initial_amount_b / initial_amount_a
    InitializePool {
        initial_amount_a: u64,
        initial_amount_b: u64,
    },
    
    /// Add liquidity to an existing pool
    /// 
    /// Arguments:
    /// - amount_a: Amount of token A to deposit
    /// - amount_b: Amount of token B to deposit  
    /// - min_liquidity: Minimum LP tokens to receive (slippage protection)
    /// 
    /// Educational note: When adding liquidity, you must maintain the current
    /// price ratio. If you deposit at a different ratio, you'll experience
    /// impermanent loss immediately!
    AddLiquidity {
        amount_a: u64,
        amount_b: u64,
        min_liquidity: u64,
    },
    
    /// Remove liquidity from a pool
    /// 
    /// Arguments:
    /// - liquidity_amount: Amount of LP tokens to burn
    /// - min_amount_a: Minimum token A to receive (slippage protection)
    /// - min_amount_b: Minimum token B to receive (slippage protection)
    /// 
    /// Educational note: You'll receive tokens proportional to your share
    /// of the pool. If the price ratio changed since you added liquidity,
    /// you might experience impermanent loss.
    RemoveLiquidity {
        liquidity_amount: u64,
        min_amount_a: u64,
        min_amount_b: u64,
    },
    
    /// Swap one token for another
    /// 
    /// Arguments:
    /// - amount_in: Amount of input token to swap
    /// - minimum_amount_out: Minimum output tokens to receive (slippage protection)
    /// 
    /// Educational note: The swap follows the constant product formula.
    /// Larger swaps will have higher price impact (slippage).
    /// The pool collects a small fee on each swap.
    Swap {
        amount_in: u64,
        minimum_amount_out: u64,
    },
}

/*
Educational note for students:

This instruction enum defines all the operations our DEX supports.
Each instruction corresponds to a common DeFi operation:

1. **InitializePool**: Like creating a new trading pair on Uniswap
2. **AddLiquidity**: Like providing liquidity to earn fees
3. **RemoveLiquidity**: Like withdrawing your liquidity position
4. **Swap**: Like trading one token for another

Key concepts:
- **Slippage Protection**: min/max amounts prevent unexpected losses
- **Price Impact**: Large trades move prices more than small ones
- **Liquidity Provision**: Earn fees by providing tokens to the pool
- **Impermanent Loss**: Risk that comes with providing liquidity
*/