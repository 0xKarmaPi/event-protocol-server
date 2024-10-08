mod error;
mod handlers;

use database::{
    native_enums::{self, Context, Network},
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

pub async fn scan(network: Network) {
    dotenv().expect("fail to load env");

    tracing_subscriber::fmt().with_target(false).init();

    let (rpc_url, key) = match network {
        Network::Solana => (
            std::env::var("SOLANA_RPC_URL").expect("missing SOLANA_RPC_URL env"),
            Setting::LastestScannedSolanaSignature,
        ),
        Network::Sonic => (
            std::env::var("SONIC_RPC_URL").expect("missing SONIC_RPC_URL env"),
            Setting::LastestScannedSonicSignature,
        ),
    };

    tracing::info!("network: {}, rpc: {}", network, rpc_url);

    let client = RpcClient::new(rpc_url);

    let program_id = Pubkey::from_str(PROGRAM_ID_STR).expect("invalid program id");

    let mut opt =
        ConnectOptions::new(std::env::var("DATABASE_URL").expect("missing DATABASE_URL env"));

    opt.sqlx_logging(false);

    let db = Database::connect(opt)
        .await
        .expect("fail to connect to datbase");

    let mut lastest_signature = setting::get(&db, key)
        .await
        .expect("fail to get lastest_scanned_signature setting")
        .expect("lastest_scanned_signature setting not found");

    loop {
        process(&db, network, &client, &program_id, &mut lastest_signature)
            .await
            .unwrap_or_else(|error| tracing::error!("{}", error));

        setting::set(&db, key, lastest_signature.clone())
            .await
            .unwrap_or_else(|error| {
                tracing::error!("fail to set lastest_scanned_signature {:#?}", error)
            });

        tokio::time::sleep(Duration::from_millis(12_000)).await;
    }
}

async fn process(
    db: &DatabaseConnection,
    network: Network,
    client: &RpcClient,
    program_id: &Pubkey,
    signture_cursor: &mut String,
) -> Result<(), ScannerError> {
    let config = GetConfirmedSignaturesForAddress2Config {
        limit: None,
        until: Some(Signature::from_str(signture_cursor)?),
        commitment: Some(CommitmentConfig::finalized()),
        before: None,
    };

    let signatures: Vec<String> = client
        .get_signatures_for_address_with_config(program_id, config)
        .await?
        .into_iter()
        .map(|tx| tx.signature)
        .rev()
        .collect();

    for sig in signatures {
        let is_resolved = signature_snapshot::find_by_signature(db, network, sig.clone())
            .await?
            .is_some();

        if is_resolved {
            tracing::info!("skip >> {}", sig);
            continue;
        }

        let tx = client
            .get_transaction(&Signature::from_str(&sig)?, UiTransactionEncoding::Base58)
            .await?;

        if tx
            .transaction
            .meta
            .as_ref()
            .is_some_and(|meta| meta.err.is_some())
        {
            tracing::info!("skip failed signature {}", sig);
            continue;
        }

        let event = tx
            .transaction
            .meta
            .map(|meta| meta.log_messages)
            .and_then(Option::from)
            .and_then(parse_logs);

        if let Some(event) = event {
            let db_event = native_enums::Event::from_ref(&event);
            let block_time = tx
                .block_time
                .unwrap_or(client.get_block_time(tx.slot).await?);

            match event {
                Event::DeployEvent(event) => {
                    process_deploy_event(db, network, client, event, block_time).await?
                }
                Event::VoteEvent(event) => {
                    process_vote_event(db, network, client, event, block_time).await?
                }
                Event::FinishEvent(event) => process_finish_event(db, network, event).await?,
                Event::ClaimRewards(event) => process_claim_reward(db, network, event).await?,
                Event::CloseEvent(event) => process_close_event(db, network, event).await?,
                Event::Withdraw(event) => process_withdraw(db, network, event).await?,
            };

            signature_snapshot::create(db, network, sig.clone(), db_event, Context::Scanner)
                .await?;
        }

        tracing::info!("resolve missing signature >> {}", sig);

        *signture_cursor = sig;
    }

    Ok(())
}
