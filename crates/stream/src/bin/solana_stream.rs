use database::native_enums::Network;

#[tokio::main]
async fn main() {
    stream::stream(Network::Solana).await;
}
