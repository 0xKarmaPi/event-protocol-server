use database::native_enums::Network;

#[tokio::main]
async fn main() {
    scanner::scan(Network::Solana).await;
}
