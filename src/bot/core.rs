use crate::config::Config;

use std::sync::Arc;
use solana_client::rpc_response::RpcLogsResponse;


/// There are 3 things this fn does:
///
/// 0. Initializes PubSub client.
/// 
/// 1. Subscribes to the pumpfun program, that creates bonding curves via WS. 
///
/// 2. Filters logs and keeps only those, which are related to the creation event.
///
/// 3. Processes the appropriated tx. Makes an JSON-RPC http req in order to get the tx's info
pub async fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let (log_tx, log_rx) = tokio::sync::mpsc::unbounded_channel::<RpcLogsResponse>();
    tokio::task::spawn(super::listener::logs_subscribe(Arc::clone(&config.pubsub_client), log_tx));

    let (filtered_log_tx, mut filtered_log_rx) = tokio::sync::mpsc::channel::<String>(30);
    tokio::task::spawn(super::filter::handle_raw_logs(log_rx, filtered_log_tx));

    while let Some(signature) = filtered_log_rx.recv().await {
        let rpc_client = Arc::clone(&config.rpc_client);
        tokio::task::spawn(async move {
            if let Err(e) = super::process::process_tx(&rpc_client, signature).await {
                log::error!("{e}");
            }
        });
    }
    
    Ok(())
}