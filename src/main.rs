use std::env;

use bitcoincore_rpc::{Auth, Client, RpcApi};
use rawtx_rs::input::InputType;
use rawtx_rs::output::OutputType;
use rawtx_rs::tx::TxInfo;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 + 1 {
        println!("Expected exactly two arguments: RPC username and RPC password");
        return;
    }
    let rpc_user = &args[1];
    let rpc_password = &args[2];

    let rpc = Client::new(
        "http://127.0.0.1:8332",
        Auth::UserPass(rpc_user.to_string(), rpc_password.to_string()),
    )
    .unwrap();

    let start_height = 780_000;

    let end_height = rpc.get_block_count().unwrap();

    for height in start_height..end_height {
        let hash = rpc.get_block_hash(height).unwrap();
        let block = rpc.get_block(&hash).unwrap();
        for tx in block.txdata.iter() {
            let tx_info = TxInfo::new(tx).unwrap();

            // As demonstration, print txids of transactions which have at least one taproot output
            // but no taproot script-path spends as input.
            if tx_info
                .output_infos
                .iter()
                .any(|output| output.out_type == OutputType::P2tr)
                && !tx_info
                    .input_infos
                    .iter()
                    .any(|input| input.in_type == InputType::P2trsp)
            {
                println!("{}", tx.txid());
            }
        }
    }
}
