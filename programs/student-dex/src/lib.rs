use solana_program::{
    account_info::{AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub mod instructions;
pub mod state;
pub mod error;

use instructions::{DexInstruction, initialize_pool, add_liquidity, remove_liquidity, swap_tokens};
use borsh::BorshDeserialize;

// Declare the program ID
solana_program::declare_id!("EduDEX1111111111111111111111111111111111111");

// Program entry point
entrypoint!(process_instruction);

/// Main instruction processor
/// This is where all DEX operations are handled
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Student DEX: Processing instruction");
    
    // Parse the instruction data to determine which operation to perform
    let instruction = DexInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        DexInstruction::InitializePool { 
            initial_amount_a, 
            initial_amount_b 
        } => {
            msg!("Instruction: Initialize Pool");
            initialize_pool(program_id, accounts, initial_amount_a, initial_amount_b)
        }
        DexInstruction::AddLiquidity { 
            amount_a, 
            amount_b, 
            min_liquidity 
        } => {
            msg!("Instruction: Add Liquidity");
            add_liquidity(program_id, accounts, amount_a, amount_b, min_liquidity)
        }
        DexInstruction::RemoveLiquidity { 
            liquidity_amount, 
            min_amount_a, 
            min_amount_b 
        } => {
            msg!("Instruction: Remove Liquidity");
            remove_liquidity(program_id, accounts, liquidity_amount, min_amount_a, min_amount_b)
        }
        DexInstruction::Swap { 
            amount_in, 
            minimum_amount_out 
        } => {
            msg!("Instruction: Swap Tokens");
            swap_tokens(program_id, accounts, amount_in, minimum_amount_out)
        }
    }
}

/*
Educational note for students:

This is the main entry point for our Solana DEX program. Every transaction
that interacts with our DEX will call this function. The key concepts here are:

1. **Program ID**: A unique identifier for our program on the Solana blockchain
2. **Accounts**: All the accounts involved in the transaction (wallets, token accounts, etc.)
3. **Instruction Data**: The specific operation the user wants to perform

Our DEX supports four main operations:
- Initialize Pool: Create a new trading pair with initial liquidity
- Add Liquidity: Provide tokens to earn trading fees
- Remove Liquidity: Withdraw tokens from the pool
- Swap: Exchange one token for another

Each operation has its own handler function that implements the business logic.
*/