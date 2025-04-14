use anchor_client::{
    solana_client::rpc_client::RpcClient, solana_sdk::commitment_config::CommitmentConfig,
};
use anchor_lang::prelude::*;
use std::{fmt::Debug, str::FromStr};

declare_program!(pamm);
use pamm::accounts::Pool;

declare_program!(raydium_amm_cpmm_new);
use raydium_amm_cpmm_new::accounts::PoolState;

#[derive(Debug, Clone)]
pub enum PoolType {
    PumpFun { program_id: Pubkey },
    Raydium { program_id: Pubkey },
}

impl PoolType {
    pub fn program_id(&self) -> Pubkey {
        match self {
            PoolType::PumpFun { program_id } => *program_id,
            PoolType::Raydium { program_id } => *program_id,
        }
    }

    pub fn pool_name(&self) -> &'static str {
        match self {
            PoolType::PumpFun { .. } => "PumpFun AMM",
            PoolType::Raydium { .. } => "Raydium AMM",
        }
    }
}

fn handle_pump_amm_deserialize(program_id: Pubkey, con: RpcClient) -> anyhow::Result<Pool> {
    let data = con.get_account_data(&program_id)?;
    let pool = Pool::try_deserialize(&mut &data[..])?;
    Ok(pool)
}

fn handle_raydium_amm_deserialize(
    program_id: Pubkey,
    con: RpcClient,
) -> anyhow::Result<raydium_amm_cpmm_new::accounts::PoolState> {
    let data = con.get_account_data(&program_id)?;
    let pool = PoolState::try_deserialize(&mut &data[..])?;
    Ok(pool)
}

enum AmmPool {
    PumpFun(Pool),
    Raydium(PoolState),
}

impl Debug for AmmPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AmmPool::PumpFun(pool) => f.debug_tuple("PumpFun").field(pool).finish(),
            AmmPool::Raydium(pool_state) => f.debug_tuple("Raydium").field(pool_state).finish(),
        }
    }
}

pub async fn get_info_struct(pool_type: PoolType, rpc_url: String) -> anyhow::Result<()> {
    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let pool = match pool_type {
        PoolType::PumpFun { program_id } => {
            AmmPool::PumpFun(handle_pump_amm_deserialize(program_id, connection)?)
        }
        PoolType::Raydium { program_id } => {
            AmmPool::Raydium(handle_raydium_amm_deserialize(program_id, connection)?)
        }
    };

    println!("Deserialized pool: {:?}", pool);

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rpc_url = "https://mainnet.helius-rpc.com/?api-key=...".to_string();
    let program_id = Pubkey::from_str("PROGRAM_ID_HERE")?;

    let pool_type = PoolType::Raydium { program_id };
    get_info_struct(pool_type, rpc_url).await?;

    Ok(())
}
