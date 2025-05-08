use tokio::sync::mpsc::{UnboundedReceiver, Sender};
use solana_client::rpc_response::RpcLogsResponse;


/// ! Spawns a new task for each log in order to prevent blocking in listener.
pub async fn handle_raw_logs(
    mut log_rx: UnboundedReceiver<RpcLogsResponse>,
    filtered_log_tx: Sender<String>
) -> () {
    while let Some(res_value) = log_rx.recv().await {
        let filtered_log_tx = filtered_log_tx.clone(); 
        tokio::task::spawn(async move {            
            for log in res_value.logs {
                if log.contains(crate::constants::PUMPFUN_CREATE_INSTRUCTION) {
                    if let Err(e) = filtered_log_tx.send(res_value.signature.clone()).await {
                        log::error!("Failed to extend the filtered logs channel!\nCause: {e}");
                    }
                }
            }   
        });
    }
}