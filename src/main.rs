use dotenv::dotenv;

use serde_json::Value;

use std::env;
use std::sync::Arc;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{compute_budget::ComputeBudgetInstruction, signature::Keypair, signer::Signer};

mod utils;
use utils::check_logs_buy_sell::*;
use utils::constants::*;

mod txn;
use txn::spam_txn::*;

mod sol_wss_methods;
use sol_wss_methods::log_subscribe::*;

use futures_util::stream::StreamExt;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message::Text;

// imports ended ---------------------
// ------------------------------------------
#[tokio::main]
pub async fn main() {
    dotenv().ok();
    env_logger::init();

    let empty_vec: Vec<Value> = vec![];

    let payer_key = env::var("PAYER").expect("payer must be set in .env file");
    let _payer = Arc::new(Keypair::from_base58_string(&payer_key));

    let RPC_HTTPS_URL = env::var("RPC_HTTPS_URL").expect("RPC_HTTPS_URL must be set in .env file");

    let WSS_HTTPS_URL = env::var("WSS_HTTPS_URL").expect("WSS_HTTPS_URL must be set in .env file");

    let spam_limit = env::var("spam_limit")
        .map(|v| v.parse::<u64>().expect("spam_limit must be a valid u64"))
        .unwrap_or(0);

    let budget_limit = env::var("budget_limit")
        .map(|v| v.parse::<u32>().expect("budget_limit must be a valid u32"))
        .unwrap_or(80_000);

    let budget_price = env::var("budget_price")
        .map(|v| v.parse::<u64>().expect("budget_price must be a valid u64"))
        .unwrap_or(10_000);

    let investment = env::var("investment")
        .map(|v| v.parse::<f64>().expect("investment must be a valid f64"))
        .unwrap_or(0.001);

    let slippage = env::var("slippage")
        .map(|v| v.parse::<f64>().expect("slippage must be a valid f64"))
        .unwrap_or(0.0);

    // constants more......
    let prices_4_spam = array_of_fees(spam_limit, budget_price).await;
    let client = Arc::new(RpcClient::new(RPC_HTTPS_URL.to_string()));
    let m_pk = _payer.as_ref().pubkey();

    let investment_lamported = investment * LAMPORTS_PER_SOL as f64;
    let adjusted_investment_for_fees = investment_lamported + (investment_lamported * 0.03);

    //  budget ix
    let unit_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(budget_limit);

    //wssssssss

    loop {
        match connect_async(WSS_HTTPS_URL.to_string()).await {
            Ok((mut stream, _)) => {
                println!("WebSocket is open");

                if let Err(e) = send_request(&mut stream).await {
                    eprintln!("Failed to send request: {:?}", e);
                }

                while let Some(Ok(message)) = stream.next().await {
                    match message {
                        Text(text) => {
                            // let start = Instant::now();
                            let parsed: Value = serde_json::from_str(&text).unwrap();
                            // Use the parsed JSON
                            let jsonified = parsed["params"]["result"]["value"]["logs"]
                                .as_array()
                                .unwrap_or(&empty_vec);

                            let logs: Vec<String> = jsonified
                                .iter()
                                .map(|v| v.as_str().unwrap_or_default().to_string())
                                .collect();

                            // let mut ix_vec = instructions_vec.clone();
                            let client_clone = client.clone();
                            let payer_clone = _payer.clone();

                            let unit_limit_ix_clone = unit_limit_ix.clone();
                            let prices_4_spam_clone = prices_4_spam.clone();

                            // break;

                            //threads
                            tokio::spawn(async move {
                                //comment this line if you wanna test only for once...

                                process_logs(
                                    &logs,
                                    client_clone,
                                    payer_clone,
                                    investment_lamported,
                                    slippage,
                                    adjusted_investment_for_fees,
                                    unit_limit_ix_clone,
                                    prices_4_spam_clone,
                                    &m_pk,
                                )
                                .await;
                            }); //comment this one too and set break points... dont get into second buying coz it will be loss

                            // continue;

                            // let duration = start.elapsed(); //current time
                            // println!("Time Consumed: {:?}", duration); //print it
                        }
                        _ => {
                            println!("Received non-text message");
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {:?}", e);
            }
        }

        println!("WebSocket is closed");
    }
}
