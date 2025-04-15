use anchor_client::{self, solana_client::rpc_client::RpcClient};
use anchor_lang::prelude::{AccountDeserialize, Pubkey, declare_program, error};
use std::fmt::Debug;
use thiserror::Error;

declare_program!(pamm);
use pamm::accounts::Pool;

declare_program!(raydium_amm_cpmm_new);
use raydium_amm_cpmm_new::accounts::PoolState;

declare_program!(raydium_camm);
use raydium_camm::accounts::PoolState as RaydiumCammPoolState;

#[derive(Debug, Clone)]
pub enum PoolType {
    PumpFun { program_id: Pubkey },
    RaydiumCpmmAmm { program_id: Pubkey },
    RaydiumCamm { program_id: Pubkey },
}

impl PoolType {
    pub fn program_id(&self) -> Pubkey {
        match self {
            PoolType::PumpFun { program_id } => *program_id,
            PoolType::RaydiumCpmmAmm { program_id } => *program_id,
            PoolType::RaydiumCamm { program_id } => *program_id,
        }
    }

    pub fn pool_name(&self) -> &'static str {
        match self {
            PoolType::PumpFun { .. } => "PumpFun AMM",
            PoolType::RaydiumCpmmAmm { .. } => "Raydium AMM",
            PoolType::RaydiumCamm { .. } => "Raydium AMM",
        }
    }
}

#[derive(Error, Debug)]
pub enum PoolError {
    #[error("RPC client error: {0}")]
    RpcError(#[from] anchor_client::solana_client::client_error::ClientError),

    #[error("Deserialization error: {0}")]
    DeserializeError(#[from] anchor_lang::error::Error),
}

fn handle_pump_amm_deserialize(
    program_id: Pubkey,
    con: &RpcClient,
) -> std::result::Result<Pool, PoolError> {
    let data = match con.get_account_data(&program_id) {
        Ok(data) => data,
        Err(e) => return Err(PoolError::RpcError(e)),
    };
    let pool = match Pool::try_deserialize(&mut &data[..]) {
        Ok(pool) => pool,
        Err(e) => return Err(PoolError::DeserializeError(e)),
    };
    Ok(pool)
}

fn handle_raydium_cpmm_amm_deserialize(
    program_id: Pubkey,
    con: RpcClient,
) -> std::result::Result<raydium_amm_cpmm_new::accounts::PoolState, PoolError> {
    let data = match con.get_account_data(&program_id) {
        Ok(data) => data,
        Err(e) => return Err(PoolError::RpcError(e)),
    };
    let pool = match PoolState::try_deserialize(&mut &data[..]) {
        Ok(pool_state) => pool_state,
        Err(e) => return Err(PoolError::DeserializeError(e)),
    };
    Ok(pool)
}

fn handle_raydium_camm_deserialize(
    program_id: Pubkey,
    con: RpcClient,
) -> std::result::Result<RaydiumCammPoolState, PoolError> {
    let data = match con.get_account_data(&program_id) {
        Ok(data) => data,
        Err(e) => return Err(PoolError::RpcError(e)),
    };
    let pool = match RaydiumCammPoolState::try_deserialize(&mut &data[..]) {
        Ok(pool_state) => pool_state,
        Err(e) => return Err(PoolError::DeserializeError(e)),
    };
    Ok(pool)
}

pub enum AmmPool {
    PumpFun(Pool),
    RaydiumCpmmAmm(PoolState),
    RaydiumCamm(RaydiumCammPoolState),
}

impl Debug for AmmPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AmmPool::PumpFun(pool) => f.debug_tuple("PumpFun").field(pool).finish(),
            AmmPool::RaydiumCpmmAmm(pool_state) => {
                f.debug_tuple("Raydium").field(pool_state).finish()
            }
            AmmPool::RaydiumCamm(pool_state) => f.debug_tuple("Raydium").field(pool_state).finish(),
        }
    }
}

/// Retrieves the information structure for a given pool type and RPC URL.
///
/// # Arguments
///
/// * `pool_type` - An enum representing the type of pool (PumpFun or Raydium) and its associated program ID.
/// * `rpc_client` - An instance of the `RpcClient`.
///
/// # Returns
///
/// This function returns a `Result` containing an `AmmPool` enum, which can be either a `PumpFun` pool or a `Raydium` pool, or an error if the operation fails.
///
/// # Errors
///
/// This function will return an error if there is an issue with deserializing the pool data or if the RPC client encounters an error.
///
/// # Examples
///
/// ```
/// let rpc_client = RpcClient::new("https://mainnet.helius-rpc.com/?api-key=...");
/// let program_id = Pubkey::from_str("PROGRAM_ID_HERE")?;
/// let pool_type = PoolType::Raydium { program_id };
/// let pool_info = get_info_struct(pool_type, rpc_client).await?;
/// println!("{:?}", pool_info);
/// ```
pub async fn get_info_struct(
    pool_type: PoolType,
    rpc_client: RpcClient,
) -> std::result::Result<AmmPool, PoolError> {
    let pool = match pool_type {
        PoolType::PumpFun { program_id } => {
            AmmPool::PumpFun(handle_pump_amm_deserialize(program_id, &rpc_client)?)
        }
        PoolType::RaydiumCpmmAmm { program_id } => {
            AmmPool::RaydiumCpmmAmm(handle_raydium_cpmm_amm_deserialize(program_id, rpc_client)?)
        }
        PoolType::RaydiumCamm { program_id } => {
            AmmPool::RaydiumCamm(handle_raydium_camm_deserialize(program_id, rpc_client)?)
        }
    };

    Ok(pool)
}
