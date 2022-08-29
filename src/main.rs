use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoin_pool_identification::PoolIdentification;

fn main() {
    let rpc = Client::new(
        "http://127.0.0.1:8332",
        Auth::UserPass("user".to_string(), "pass".to_string()),
    )
    .unwrap();

    let start_height = 700_000;

    let end_height = rpc.get_block_count().unwrap();

    for height in start_height..end_height {
        let hash = rpc.get_block_hash(height).unwrap();
        let block = rpc.get_block(&hash).unwrap();
        let pool = block.identify_pool();
        println!("Block {} ({}) mined by {:?}", hash, height, pool);
    }
}

