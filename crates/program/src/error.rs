#[derive(thiserror::Error, Debug)]
pub enum ProgramError {
    #[error(transparent)]
    RpcClientError(#[from] solana_client::client_error::ClientError),

    #[error(transparent)]
    BorshDeserializeError(#[from] std::io::Error),
}
