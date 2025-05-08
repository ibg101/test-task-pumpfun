use std::sync::Arc;
use solana_commitment_config::CommitmentConfig;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_response::{Response, RpcLogsResponse},
    rpc_config::{RpcTransactionLogsFilter, RpcTransactionLogsConfig}
};
use futures::StreamExt;


pub async fn logs_subscribe(
    client: Arc<PubsubClient>,
    res_tx: tokio::sync::mpsc::UnboundedSender<RpcLogsResponse>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let filter: RpcTransactionLogsFilter = RpcTransactionLogsFilter::Mentions(
        vec![crate::constants::PUMPFUN_PROGRAM_ID.to_owned()]
    );
    let config = RpcTransactionLogsConfig { commitment: Some(CommitmentConfig::processed()) };

    let (mut stream, _) = client.logs_subscribe(filter, config).await?;

    // we dont need ctx, so destructure it directly
    while let Some(Response { value, .. }) = stream.next().await {
        res_tx.send(value)?
    }
    
    Ok(())
}