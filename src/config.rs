#![allow(dead_code)]  // silence compiler

use std::sync::Arc;
use solana_client::nonblocking::{
    pubsub_client::PubsubClient,
    rpc_client::RpcClient
};


pub struct Config {
    pub pubsub_client: Arc<PubsubClient>,
    pub rpc_client: Arc<RpcClient>,
    ws_rpc_url: String,  // i decided to stick with default RPC provider instead of gRPC
    http_rpc_url: String
}

impl Config {
    pub async fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let ws_rpc_url: String = std::env::var("WS_RPC_URL")?;
        let http_rpc_url: String = std::env::var("HTTP_RPC_URL")?;
        let pubsub_client = Arc::new(PubsubClient::new(&ws_rpc_url).await?);
        let rpc_client = Arc::new(RpcClient::new(http_rpc_url.clone()));

        Ok(Self {
            pubsub_client,
            rpc_client,
            ws_rpc_url,
            http_rpc_url,
        })
    }

    pub fn get_ws_rpc_url(&self) -> &str {
        &self.ws_rpc_url
    }

    pub fn get_http_rpc_url(&self) -> &str {
        &self.http_rpc_url
    }
}