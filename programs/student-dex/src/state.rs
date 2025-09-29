use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// The main pool state that stores all information about a trading pair
/// This is stored on-chain and represents the core of our AMM
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PoolState {
    /// Whether this pool has been initialized
    pub is_initialized: bool,
    
    /// The authority that can perform admin operations
    pub admin: Pubkey,
    
    /// Token A mint address (e.g., USDC)
    pub token_a_mint: Pubkey,
    
    /// Token B mint address (e.g., SOL)
    pub token_b_mint: Pubkey,
    
    /// Pool's token A account (holds the reserves)
    pub token_a_account: Pubkey,
    
    /// Pool's token B account (holds the reserves)
    pub token_b_account: Pubkey,
    
    /// LP (Liquidity Provider) token mint
    /// Users receive LP tokens when providing liquidity
    pub lp_token_mint: Pubkey,
    
    /// Current reserves of token A in the pool
    pub reserve_a: u64,
    
    /// Current reserves of token B in the pool
    pub reserve_b: u64,
    
    /// Total supply of LP tokens
    pub lp_token_supply: u64,
    
    /// Trading fee in basis points (e.g., 30 = 0.3%)
    pub fee_rate: u16,
    
    /// Accumulated fees for token A
    pub accumulated_fee_a: u64,
    
    /// Accumulated fees for token B
    pub accumulated_fee_b: u64,
}

impl PoolState {
    /// Size of the pool state in bytes (for account allocation)
    pub const LEN: usize = 32 + // is_initialized (bool) + padding
                           32 + // admin
                           32 + // token_a_mint
                           32 + // token_b_mint
                           32 + // token_a_account
                           32 + // token_b_account
                           32 + // lp_token_mint
                           8 +  // reserve_a
                           8 +  // reserve_b
                           8 +  // lp_token_supply
                           2 +  // fee_rate
                           8 +  // accumulated_fee_a
                           8;   // accumulated_fee_b
    
    /// Create a new uninitialized pool state
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            admin: Pubkey::default(),
            token_a_mint: Pubkey::default(),
            token_b_mint: Pubkey::default(),
            token_a_account: Pubkey::default(),
            token_b_account: Pubkey::default(),
            lp_token_mint: Pubkey::default(),
            reserve_a: 0,
            reserve_b: 0,
            lp_token_supply: 0,
            fee_rate: 30, // 0.3% default trading fee
            accumulated_fee_a: 0,
            accumulated_fee_b: 0,
        }
    }
    
    /// Calculate the current price of token A in terms of token B
    /// Price = reserve_b / reserve_a
    pub fn get_price_a_to_b(&self) -> Result<f64, &'static str> {
        if self.reserve_a == 0 {
            return Err("Cannot calculate price with zero reserves");
        }
        Ok(self.reserve_b as f64 / self.reserve_a as f64)
    }
    
    /// Calculate the current price of token B in terms of token A
    /// Price = reserve_a / reserve_b
    pub fn get_price_b_to_a(&self) -> Result<f64, &'static str> {
        if self.reserve_b == 0 {
            return Err("Cannot calculate price with zero reserves");
        }
        Ok(self.reserve_a as f64 / self.reserve_b as f64)
    }
    
    /// Calculate swap output using constant product formula
    /// For swapping dx amount of token A for token B:
    /// dy = (reserve_b * dx) / (reserve_a + dx)
    /// But with fees: dy = (reserve_b * dx * (10000 - fee_rate)) / ((reserve_a + dx) * 10000)
    pub fn calculate_swap_output(
        &self,
        input_amount: u64,
        input_reserve: u64,
        output_reserve: u64,
    ) -> Result<u64, &'static str> {
        if input_amount == 0 {
            return Err("Input amount cannot be zero");
        }
        
        if input_reserve == 0 || output_reserve == 0 {
            return Err("Cannot swap with zero reserves");
        }
        
        // Apply trading fee (fee_rate is in basis points)
        let input_with_fee = input_amount
            .checked_mul(10000_u64.checked_sub(self.fee_rate as u64).unwrap())
            .ok_or("Math overflow")?;
        
        let numerator = input_with_fee
            .checked_mul(output_reserve)
            .ok_or("Math overflow")?;
        
        let denominator = input_reserve
            .checked_mul(10000)
            .ok_or("Math overflow")?
            .checked_add(input_with_fee)
            .ok_or("Math overflow")?;
        
        Ok(numerator / denominator)
    }
    
    /// Calculate required input for a desired output (reverse swap calculation)
    pub fn calculate_swap_input(
        &self,
        output_amount: u64,
        input_reserve: u64,
        output_reserve: u64,
    ) -> Result<u64, &'static str> {
        if output_amount == 0 {
            return Err("Output amount cannot be zero");
        }
        
        if output_amount >= output_reserve {
            return Err("Output amount exceeds reserves");
        }
        
        if input_reserve == 0 || output_reserve == 0 {
            return Err("Cannot swap with zero reserves");
        }
        
        let numerator = input_reserve
            .checked_mul(output_amount)
            .ok_or("Math overflow")?
            .checked_mul(10000)
            .ok_or("Math overflow")?;
        
        let denominator = output_reserve
            .checked_sub(output_amount)
            .ok_or("Math underflow")?
            .checked_mul(10000_u64.checked_sub(self.fee_rate as u64).unwrap())
            .ok_or("Math overflow")?;
        
        Ok((numerator / denominator) + 1) // Add 1 to round up
    }
}

/*
Educational note for students:

The PoolState is the heart of our AMM. Key concepts:

1. **Reserves**: The actual tokens held by the pool
2. **Constant Product**: x * y = k (where x and y are reserves)
3. **LP Tokens**: Represent ownership share in the pool
4. **Price Discovery**: Prices are determined by the ratio of reserves
5. **Trading Fees**: Small fee on each trade (typically 0.3%)

Mathematical formulas used:
- Price: price_a = reserve_b / reserve_a
- Swap: output = (reserve_out * input * (1 - fee)) / (reserve_in + input * (1 - fee))
- Liquidity: lp_tokens = sqrt(amount_a * amount_b) for first deposit

This creates an automated market maker where:
- Larger trades have higher price impact (slippage)
- Pool automatically balances supply and demand
- Liquidity providers earn fees from all trades
*/