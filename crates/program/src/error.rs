#[derive(thiserror::Error, Debug)]
pub enum ProgramError {
    #[error(transparent)]
    RpcClientError(#[from] solana_client::client_error::ClientError),

    #[error(transparent)]
    BorshDeserializeError(#[from] std::io::Error),

    #[error("account not found: pubkey={0}")]
    AccountNotFound(String),
}
