pub mod accounts;
pub mod error;
pub mod events;
pub mod log;

use borsh::BorshDeserialize;
use error::ProgramError;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

pub const PROGRAM_ID_STR: &str = "4om3tmwcjESCkngvezJSyAerKgSfggCgf2LZe4XZk3HL";

pub async fn deserialize_account<T: BorshDeserialize>(
    client: &RpcClient,
    pubkey: &Pubkey,
) -> Result<T, ProgramError> {
    let data = client.get_account_data(pubkey).await?;

    let data: &mut &[u8] = &mut &data.as_slice()[8..];

    T::deserialize(data).map_err(Into::into)
}