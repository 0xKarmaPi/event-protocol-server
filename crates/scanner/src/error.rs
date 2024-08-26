#[derive(thiserror::Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum ScannerError {
    #[error(transparent)]
    ParsePubkeyError(#[from] solana_sdk::pubkey::ParsePubkeyError),

    #[error(transparent)]
    RpcClientError(#[from] solana_client::client_error::ClientError),

    #[error(transparent)]
    ParseSignatureError(#[from] solana_sdk::signature::ParseSignatureError),
}
