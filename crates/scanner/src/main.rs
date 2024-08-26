mod error;
mod handlers;

use database::{
    native_enums::Context,
    repositories::{
        setting::{self, Setting},
        signature_snapshot,
    },
    sea_orm::{ConnectOptions, Database, DatabaseConnection},
};
use dotenv::dotenv;
use error::ScannerError;
use handlers::{
    process_claim_reward, process_close_event, process_deploy_event, process_finish_event,
    process_vote_event, process_withdraw,
};
use program::{
    log::{parse_logs, Event},
    PROGRAM_ID_STR,
};
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config,
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature};
use solana_transaction_status::UiTransactionEncoding;
use std::{str::FromStr, time::Duration};

#[tokio::main]
async fn main() {
    dotenv().expect("fail to load env");

    let client = RpcClient::new("https://api.devnet.solana.com".to_string());

    let program_id = Pubkey::from_str(PROGRAM_ID_STR).expect("invalid program id");

    let opt = ConnectOptions::new(std::env::var("DATABASE_URL").expect("missing DATABASE_URL env"));

    let db = Database::connect(opt)
        .await
        .expect("fail to connect to datbase");

    let mut lastest_signature = setting::get(&db, Setting::LastestScannedSignature)
        .await
        .expect("fail to get lastest_scanned_signature setting")
        .expect("lastest_scanned_signature setting not found");

    loop {
        scan(&db, &client, &program_id, &mut lastest_signature)
            .await
            .unwrap_or_else(|error| eprintln!("{}", error));

        setting::set(
            &db,
            Setting::LastestScannedSignature,
            lastest_signature.clone(),
        )
        .await
        .unwrap_or_else(|error| eprintln!("fail to set lastest_scanned_signature {}", error));

        tokio::time::sleep(Duration::from_millis(6000)).await;
    }
}

async fn scan(
    db: &DatabaseConnection,
    client: &RpcClient,
    program_id: &Pubkey,
    signture_cursor: &mut String,
) -> Result<(), ScannerError> {
    let config = GetConfirmedSignaturesForAddress2Config {
        limit: Some(20),
        until: Some(Signature::from_str(signture_cursor)?),
        commitment: Some(CommitmentConfig::finalized()),
        before: None,
    };

    let signatures: Vec<String> = client
        .get_signatures_for_address_with_config(&program_id, config)
        .await?
        .into_iter()
        .map(|tx| tx.signature)
        .rev()
        .collect();

    dbg!(&signatures);

    for sig in signatures {
        let is_resolved = signature_snapshot::find_by_signature(db, &sig)
            .await?
            .is_some();

        if !is_resolved {
            let tx = client
                .get_transaction(&Signature::from_str(&sig)?, UiTransactionEncoding::Base58)
                .await?;

            let event = tx
                .transaction
                .meta
                .map(|meta| meta.log_messages)
                .and_then(Option::from)
                .and_then(parse_logs);

            if let Some(event) = event {
                match &event {
                    Event::DeployEvent(event) => {
                        process_deploy_event(&db, client, event, &sig).await?
                    }
                    Event::VoteEvent(event) => process_vote_event(&db, event, &sig).await?,
                    Event::FinishEvent(event) => process_finish_event(&db, event, &sig).await?,
                    Event::ClaimRewards(event) => process_claim_reward(&db, event, &sig).await?,
                    Event::CloseEvent(event) => process_close_event(&db, event, &sig).await?,
                    Event::Withdraw(event) => process_withdraw(&db, event, &sig).await?,
                };

                signature_snapshot::create(db, sig.clone(), event.into(), Context::Scanner).await?;
            }
        }

        *signture_cursor = sig;
    }

    Ok(())
}
