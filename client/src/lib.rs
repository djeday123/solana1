use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    system_instruction,
    transaction::Transaction,
};
use spl_token;
use std::str::FromStr;

// Re-export the instruction types from the program
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum DexInstruction {
    InitializePool {
        initial_liquidity_a: u64,
        initial_liquidity_b: u64,
    },
    Swap {
        amount_in: u64,
        minimum_amount_out: u64,
        swap_a_to_b: bool,
    },
    AddLiquidity {
        amount_a: u64,
        amount_b: u64,
        minimum_liquidity_tokens: u64,
    },
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
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub fee_numerator: u64,
    pub fee_denominator: u64,
    pub total_liquidity_tokens: u64,
}

pub struct DexClient {
    pub client: RpcClient,
    pub program_id: Pubkey,
}

impl DexClient {
    pub fn new(rpc_url: String, program_id: Pubkey) -> Self {
        let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
        
        Self { client, program_id }
    }

    pub async fn initialize_pool(
        &self,
        payer: &Keypair,
        pool_state: &Keypair,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
        initial_liquidity_a: u64,
        initial_liquidity_b: u64,
    ) -> Result<Signature> {
        // Create the pool state account
        let rent = self.client.get_minimum_balance_for_rent_exemption(
            std::mem::size_of::<PoolState>()
        )?;
        
        let create_pool_state_ix = system_instruction::create_account(
            &payer.pubkey(),
            &pool_state.pubkey(),
            rent,
            std::mem::size_of::<PoolState>() as u64,
            &self.program_id,
        );

        // Create associated token accounts for the pool
        let pool_token_a = spl_associated_token_account::get_associated_token_address(
            &pool_state.pubkey(),
            token_a_mint,
        );
        let pool_token_b = spl_associated_token_account::get_associated_token_address(
            &pool_state.pubkey(),
            token_b_mint,
        );

        let create_pool_token_a_ix = 
            spl_associated_token_account::instruction::create_associated_token_account(
                &payer.pubkey(),
                &pool_state.pubkey(),
                token_a_mint,
                &spl_token::id(),
            );
            
        let create_pool_token_b_ix = 
            spl_associated_token_account::instruction::create_associated_token_account(
                &payer.pubkey(),
                &pool_state.pubkey(),
                token_b_mint,
                &spl_token::id(),
            );

        // Initialize pool instruction
        let instruction_data = DexInstruction::InitializePool {
            initial_liquidity_a,
            initial_liquidity_b,
        };
        
        let initialize_pool_ix = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(pool_state.pubkey(), false),
                AccountMeta::new_readonly(*token_a_mint, false),
                AccountMeta::new_readonly(*token_b_mint, false),
                AccountMeta::new(pool_token_a, false),
                AccountMeta::new(pool_token_b, false),
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            ],
            data: borsh::to_vec(&instruction_data)?,
        };

        let recent_blockhash = self.client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[
                create_pool_state_ix,
                create_pool_token_a_ix,
                create_pool_token_b_ix,
                initialize_pool_ix,
            ],
            Some(&payer.pubkey()),
            &[payer, pool_state],
            recent_blockhash,
        );

        let signature = self.client.send_and_confirm_transaction(&transaction)?;
        Ok(signature)
    }

    pub async fn swap(
        &self,
        user: &Keypair,
        pool_state: &Pubkey,
        amount_in: u64,
        minimum_amount_out: u64,
        swap_a_to_b: bool,
    ) -> Result<Signature> {
        let pool_data = self.get_pool_state(pool_state).await?;
        
        let user_token_a = spl_associated_token_account::get_associated_token_address(
            &user.pubkey(),
            &pool_data.token_a_mint,
        );
        let user_token_b = spl_associated_token_account::get_associated_token_address(
            &user.pubkey(),
            &pool_data.token_b_mint,
        );

        let instruction_data = DexInstruction::Swap {
            amount_in,
            minimum_amount_out,
            swap_a_to_b,
        };

        let swap_ix = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(user.pubkey(), true),
                AccountMeta::new(*pool_state, false),
                AccountMeta::new(user_token_a, false),
                AccountMeta::new(user_token_b, false),
                AccountMeta::new(pool_data.token_a_account, false),
                AccountMeta::new(pool_data.token_b_account, false),
                AccountMeta::new_readonly(spl_token::id(), false),
            ],
            data: borsh::to_vec(&instruction_data)?,
        };

        let recent_blockhash = self.client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[swap_ix],
            Some(&user.pubkey()),
            &[user],
            recent_blockhash,
        );

        let signature = self.client.send_and_confirm_transaction(&transaction)?;
        Ok(signature)
    }

    pub async fn get_pool_state(&self, pool_state: &Pubkey) -> Result<PoolState> {
        let account_data = self.client.get_account_data(pool_state)?;
        let pool_state = PoolState::try_from_slice(&account_data)?;
        Ok(pool_state)
    }

    pub async fn calculate_swap_output(
        &self,
        pool_state: &Pubkey,
        amount_in: u64,
        swap_a_to_b: bool,
    ) -> Result<u64> {
        let pool_data = self.get_pool_state(pool_state).await?;
        
        // Get current pool balances
        let pool_a_balance = self.client.get_token_account_balance(&pool_data.token_a_account)?.ui_amount_string.parse::<f64>().unwrap_or(0.0) as u64;
        let pool_b_balance = self.client.get_token_account_balance(&pool_data.token_b_account)?.ui_amount_string.parse::<f64>().unwrap_or(0.0) as u64;
        
        let (reserve_in, reserve_out) = if swap_a_to_b {
            (pool_a_balance, pool_b_balance)
        } else {
            (pool_b_balance, pool_a_balance)
        };
        
        if amount_in == 0 || reserve_in == 0 || reserve_out == 0 {
            return Ok(0);
        }
        
        // Calculate with fee
        let fee = amount_in * pool_data.fee_numerator / pool_data.fee_denominator;
        let amount_in_with_fee = amount_in - fee;
        
        // Constant product formula
        let amount_out = (reserve_out * amount_in_with_fee) / (reserve_in + amount_in_with_fee);
        
        Ok(amount_out)
    }
}

// Utility functions for common operations
pub fn create_mint(
    client: &RpcClient,
    payer: &Keypair,
    mint_authority: &Pubkey,
    decimals: u8,
) -> Result<Keypair> {
    let mint_keypair = Keypair::new();
    
    let rent = client.get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)?;
    
    let create_mint_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint_keypair.pubkey(),
        rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
    );
    
    let initialize_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint_keypair.pubkey(),
        mint_authority,
        None,
        decimals,
    )?;
    
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[create_mint_account_ix, initialize_mint_ix],
        Some(&payer.pubkey()),
        &[payer, &mint_keypair],
        recent_blockhash,
    );
    
    client.send_and_confirm_transaction(&transaction)?;
    
    Ok(mint_keypair)
}

pub fn mint_tokens(
    client: &RpcClient,
    payer: &Keypair,
    mint: &Pubkey,
    destination: &Pubkey,
    authority: &Keypair,
    amount: u64,
) -> Result<Signature> {
    let mint_to_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        mint,
        destination,
        &authority.pubkey(),
        &[],
        amount,
    )?;
    
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[mint_to_ix],
        Some(&payer.pubkey()),
        &[payer, authority],
        recent_blockhash,
    );
    
    let signature = client.send_and_confirm_transaction(&transaction)?;
    Ok(signature)
}
