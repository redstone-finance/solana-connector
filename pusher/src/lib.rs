pub mod redstone_sdk;

use std::str::FromStr;

use redstone_sdk::RedstoneClient;
use solana_client_wasm::solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_client_wasm::WasmClient;
use worker::*;

const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";
const METHOD_DISCRIMINATOR: [u8; 8] = [49, 96, 127, 141, 118, 203, 237, 178];
const REDSTONE_SOL_PROGRAM_ID: &str =
    "3oHtb7BCqjqhZt8LyqSAZRAubbrYy8xvDRaYoRghHB1T";
const DATA_SERVICE_ID: &str = "redstone-avalanche-prod";
const UNIQUE_SIGNER_COUNT: u8 = 3;

#[event(fetch)]
pub async fn main(
    _req: Request,
    env: Env,
    _ctx: worker::Context,
) -> Result<Response> {
    console_log!("Fetch function called");

    let result: Result<String> = async {
        let rpc_url = env.secret("RPC_URL").map_err(|e| {
            console_error!("Failed to get RPC_URL: {:?}", e);
            Error::from("Missing RPC_URL environment variable")
        })?;

        let private_key = env.secret("PRIVATE_KEY").map_err(|e| {
            console_error!("Failed to get PRIVATE_KEY: {:?}", e);
            Error::from("Missing PRIVATE_KEY environment variable")
        })?;

        let feed_id = "AVAX".to_string(); // this can be configurable

        let client = WasmClient::new(&rpc_url.to_string());
        let keypair = Keypair::from_base58_string(&private_key.to_string());

        console_log!("Using signer: {}", keypair.pubkey());

        let signature = push_data(&client, &keypair, feed_id).await?;
        console_log!("{}: {}", Date::now(), signature);
        Ok(format!("Data pushed successfully: {}", signature).to_string())
    }
    .await;

    match result {
        Ok(message) => Response::ok(message),
        Err(e) => {
            console_error!("Error: {:?}", e);
            Response::error(format!("Error: {:?}", e), 500)
        }
    }
}

async fn push_data(
    client: &WasmClient,
    signer: &Keypair,
    feed_id: String,
) -> Result<String> {
    console_log!("Pushing data for feed: {}", feed_id);
    let transaction = make_transaction(client, signer, feed_id).await?;
    console_log!("Transaction created");
    let signature = send_transaction(client, transaction).await?;
    console_log!("Transaction sent");
    Ok(signature)
}

async fn make_transaction(
    client: &WasmClient,
    signer: &Keypair,
    feed_id: String,
) -> Result<Transaction> {
    let price_account = get_price_account(feed_id.clone());
    let keys = vec![
        AccountMeta::new(signer.pubkey(), true),
        AccountMeta::new(price_account, false),
        AccountMeta::new_readonly(
            Pubkey::from_str(SYSTEM_PROGRAM_ID).expect("pubkey"),
            false,
        ),
    ];

    let instruction_data = make_instruction_data(feed_id).await?;

    let instruction = Instruction::new_with_bytes(
        Pubkey::from_str(REDSTONE_SOL_PROGRAM_ID).expect("pubkey"),
        &instruction_data,
        keys,
    );

    Ok(Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[signer],
        client.get_latest_blockhash().await.map_err(|e| {
            Error::from(format!("Error getting blockhash: {:?}", e))
        })?,
    ))
}

// TODO implement retries here or use jito
async fn send_transaction(
    client: &WasmClient,
    transaction: Transaction,
) -> Result<String> {
    let signature =
        client.send_transaction(&transaction).await.map_err(|e| {
            Error::from(format!("Error sending transaction: {:?}", e))
        })?;

    Ok(signature.to_string())
}

async fn make_instruction_data(feed_id: String) -> Result<Vec<u8>> {
    let payload = make_payload(&[feed_id.clone()]).await?;
    let mut instruction_data = Vec::new();
    instruction_data.extend_from_slice(&METHOD_DISCRIMINATOR);
    instruction_data.extend_from_slice(&make_feed_id_bytes(feed_id));
    instruction_data.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    instruction_data.extend_from_slice(&payload);
    Ok(instruction_data)
}

async fn make_payload(data_feeds: &[String]) -> Result<Vec<u8>> {
    let redstone_client = RedstoneClient::new();
    let payload = redstone_client
        .request_redstone_payload(
            data_feeds,
            DATA_SERVICE_ID.to_string(),
            UNIQUE_SIGNER_COUNT.into(),
        )
        .await?;
    Ok(payload)
}

fn get_price_account(feed_id: String) -> Pubkey {
    let price_seed = make_price_seed();
    let feed_id_bytes = make_feed_id_bytes(feed_id);
    let seeds = &[
        price_seed.as_slice(),
        feed_id_bytes.as_slice(),
        &METHOD_DISCRIMINATOR,
    ];
    Pubkey::find_program_address(
        seeds,
        &Pubkey::from_str(REDSTONE_SOL_PROGRAM_ID).unwrap(),
    )
    .0
}

pub fn make_feed_id_bytes(feed_id: String) -> Vec<u8> {
    let mut bytes = feed_id.as_bytes().to_vec();
    bytes.resize(32, 0);
    bytes
}

pub fn make_price_seed() -> Vec<u8> {
    let mut bytes = "price".as_bytes().to_vec();
    bytes.resize(32, 0);
    bytes
}
