mod bot;
mod config;
mod constants;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    env_logger::init();
    let config = config::Config::init()
        .await
        .map_err(|e| format!("Failed to initialize config::Config\nCause: {e}"))?;

    log::info!("Starting the bot!");
    bot::core::run(&config).await?;    

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test]
    // async fn ws_conn() -> Result<(), Box<dyn std::error::Error>> {
    //     dotenvy::dotenv()?;
    //     env_logger::init();
    //     let config = config::Config::init()
    //         .await
    //         .map_err(|e| format!("Failed to initialize config::Config\nCause: {e}"))?;

    //     let (log_tx, mut log_rx) = tokio::sync::mpsc::unbounded_channel::<solana_client::rpc_response::RpcLogsResponse>();
    //     let client= solana_client::nonblocking::pubsub_client::PubsubClient::new(&config.ws_rpc_url).await?;
    //     // bot::listener::logs_subscribe(&client, log_tx).await?;  // change the visability of the crate to test it again

    //     while let Some(res_value) = log_rx.recv().await {
    //         println!("{:#?}", res_value);
    //     }

    //     Ok(())
    // }

    // #[tokio::test]
    // async fn get_tx() -> Result<(), Box<dyn std::error::Error>> {
    //     dotenvy::dotenv()?;
    //     env_logger::init();
    //     let config = config::Config::init()
    //         .await
    //         .map_err(|e| format!("Failed to initialize config::Config\nCause: {e}"))?;
        
    //     let sig_string = "5mYkTaGjGgrZTx6oxXUV8fTBRuRpXa15Dxm9efz5VxkLZ27BQCSHpyDGw2HXBTtzZDp79fwJHJpYJJyrt6vrsGEt".to_owned();

    //     bot::process::process_tx(
    //         &config.rpc_client,
    //         sig_string
    //     ).await?;        

    //     Ok(())
    // }

    // #[tokio::test]
    // async fn deserialize_metaplex() -> Result<(), Box<dyn std::error::Error>> {
    //     dotenvy::dotenv()?;
    //     env_logger::init();
    //     let config = config::Config::init()
    //         .await
    //         .map_err(|e| format!("Failed to initialize config::Config\nCause: {e}"))?;
        
    //     // bot::process::get_metaplex_metadata(&config.rpc_client, "FwPEKZEEtmidwX1ZK2EKErmPQduW5tc2zDgC6R2jpump").await?;
    //     Ok(())
    // }
}
