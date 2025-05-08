#![allow(dead_code)]

use std::str::FromStr;

use serde::Deserialize;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_commitment_config::CommitmentConfig;
use solana_pubkey::Pubkey;
use solana_transaction_status_client_types::{EncodedConfirmedTransactionWithStatusMeta, UiInstruction, UiParsedInstruction, UiTransactionEncoding};
use solana_signature::Signature;
use mpl_token_metadata::accounts::Metadata as MetadataAccount;


const ADDR: usize = 44; 

#[derive(Debug, Default)]
struct ParsedTx {
    mint: String,
    bonding_curve: String,
    name: String,
    symbol: String
}

impl ParsedTx {
    fn default_preallocated() -> Self {
        Self {
            mint: String::with_capacity(ADDR),
            bonding_curve: String::with_capacity(ADDR),
            ..Default::default()
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct MaybeInitializeAccount3 {
    info: InitializeAccount3Info,
    #[serde(rename = "type")]
    i_type: String
}

#[derive(Debug, serde::Deserialize)]
struct InitializeAccount3Info {
    account: String,
    mint: String,
    owner: String
}

pub async fn process_tx(
    client: &RpcClient,
    signature: String
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("{}", signature);    
    let mut parsed_tx: ParsedTx = ParsedTx::default_preallocated();
    
    let sig = Signature::from_str(&signature)?;
    
    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::JsonParsed),
        max_supported_transaction_version: Some(0),
        commitment: Some(CommitmentConfig::confirmed())
    };

    let retries: u8 = 5;

    for attempt in 1..=retries {
        match client.get_transaction_with_config(&sig, config).await {
            Ok(tx) => {
                parse_tx(tx, &mut parsed_tx)?;
                break;
            }
            Err(e) if attempt == retries => return Err(e.into()),
            Err(_) => tokio::time::sleep(tokio::time::Duration::from_millis(500)).await,
        }
    }

    get_metaplex_metadata(client, &mut parsed_tx).await?;

    log::info!("{:#?}", parsed_tx);

    Ok(())
}

fn parse_tx(
    tx: EncodedConfirmedTransactionWithStatusMeta,
    parsed_tx: &mut ParsedTx
) -> Result<(), Box<dyn std::error::Error>> {
    let inner_instructions = tx.transaction.meta
        .ok_or("Failed to get tx.meta!")?
        .inner_instructions
        .ok_or("Failed to get tx.inner_instructions!")?;

    let create_instructions = inner_instructions
        .first()
        .ok_or("Failed to get `create` instructions!")?;

    for instruction in create_instructions.instructions.iter() {
        match instruction {
            UiInstruction::Parsed(UiParsedInstruction::Parsed(i)) => {
                if let Ok(instruction) = MaybeInitializeAccount3::deserialize(&i.parsed) {
                    // extra check, because there are instructions with similar body
                    if instruction.i_type == "initializeAccount3" {
                        parsed_tx.mint.push_str(&instruction.info.mint);
                        parsed_tx.bonding_curve.push_str(&instruction.info.owner);
                        return Ok(())
                    }
                }
            },
            _ => continue
        }
    }

    Err("Failed to parse tx".into())
}

// i dont have time to write my own parser, so i use onchain call
async fn get_metaplex_metadata(
    client: &RpcClient,
    parsed_tx: &mut ParsedTx
) -> Result<(), Box<dyn std::error::Error>> {
    let (pda, _bump) = MetadataAccount::find_pda(&Pubkey::from_str(&parsed_tx.mint)?);
    
    const MAX_RETRIES: u8 = 5;
    let mut retries: u8 = 0;
    let data = loop {
        match client.get_account_data(&pda).await {
            Ok(v) => break Ok::<Vec<u8>, Box<dyn std::error::Error>>(v),
            Err(e) if retries == MAX_RETRIES => return Err(e.into()),
            Err(_) => tokio::time::sleep(tokio::time::Duration::from_millis(500)).await,
        };
        retries += 1;
    }?;

    let meta: MetadataAccount = MetadataAccount::safe_deserialize(&data)?;

    parsed_tx.name = meta.name;
    parsed_tx.symbol = meta.symbol;

    Ok(())
}