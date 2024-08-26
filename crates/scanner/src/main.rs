mod error;

use database::sea_orm::{ConnectOptions, Database};
use dotenv::dotenv;
use error::ScannerError;
use program::{
    accounts::PredictionEvent,
    log::{parse_logs, Event},
    PROGRAM_ID_STR,
};
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config,
};
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use solana_transaction_status::UiTransactionEncoding;
use std::{str::FromStr, time::Duration};

#[tokio::main]
async fn main() -> Result<(), ScannerError> {
    dotenv().expect("failt to load env");

    let client = RpcClient::new("https://api.devnet.solana.com".to_string());

    let program_id = Pubkey::from_str(PROGRAM_ID_STR)?;

    let opt = ConnectOptions::new(std::env::var("DATABASE_URL").expect("missing DATABASE_URL env"));

    let _db = Database::connect(opt)
        .await
        .expect("fail to connect to datbase");

    let account = program::deserialize_account::<PredictionEvent>(
        &client,
        &Pubkey::from_str("2Fm2TWAnX1JDfJ1kLbozjJsg58aCGJhKYXtRNBSwCYs6")?,
    )
    .await
    .unwrap();

    dbg!(account);

    loop {
        let config = GetConfirmedSignaturesForAddress2Config {
            limit: Some(10),
            until:Some(Signature::from_str("2RyjAgV7Hq7fh7i4E4ZwWMDwYYg6RCzQfJPJSXjBhW6pVzCce2H1hTayGrpd5Lp1Q285yfZEh2AAofLyobRTU146").unwrap()),
            ..Default::default()
        };

        let signatures: Vec<String> = client
            .get_signatures_for_address_with_config(&program_id, config)
            .await?
            .into_iter()
            .map(|tx| tx.signature)
            .rev()
            .collect();

        for signature in signatures {
            let tx = client
                .get_transaction(
                    &Signature::from_str(&signature)?,
                    UiTransactionEncoding::Base58,
                )
                .await?;

            let event = tx
                .transaction
                .meta
                .map(|meta| meta.log_messages)
                .and_then(Option::from)
                .and_then(parse_logs);

            if let Some(event) = event {
                match event {
                    Event::DeployEvent(event) => {
                        dbg!(event);
                    }
                    Event::VoteEvent(event) => {
                        dbg!(event);
                    }
                    Event::FinishEvent(event) => {
                        dbg!(event);
                    }
                }
            }
        }

        tokio::time::sleep(Duration::from_millis(2000)).await;
    }
}
