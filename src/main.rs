use ethers::{providers::{Provider, Ws, Middleware, StreamExt}};
use eyre::Result;
use std::{sync::Arc};


const WSS_URL: &str = "wss://your_link";


#[tokio::main]
async fn main() -> Result<()> {
    let provider = Arc::new(Provider::<Ws>::connect(WSS_URL).await.unwrap());

    provider
        .subscribe_pending_txs()
        .await
        .unwrap()
        .for_each(|tx_hash| process_hash(tx_hash, provider.clone()))
        .await;


    Ok(())
}


async fn process_hash(
    tx_hash: ethers::types::H256,
    provider: Arc<Provider<Ws>>,
) {
    tokio::spawn(async move{
        match provider.get_transaction(tx_hash).await {
            Ok(Some(tx)) => {
                println!("Hash: {:?}, status: {:?}", tx.hash, tx.block_hash.is_some());
            },
            Ok(None) => {},
            Err(_) => {}
        }
    });
}