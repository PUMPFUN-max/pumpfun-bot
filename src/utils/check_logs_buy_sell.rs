use chrono::Local;
use std::sync::Arc;
use std::time::Duration;

use solana_client::nonblocking::rpc_client::RpcClient;

use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer};
use spl_associated_token_account;
use spl_token::instruction::close_account;

use super::constants::*;
use super::create_ix::{create_sell_ix, get_buy_ix};
use super::pf_price::*;

use crate::txn::spam_txn::spammer;

use super::layouts::TradeEvent;

async fn valid_logs(logs: &Vec<String>) -> bool {
    let mut a = false;
    let mut b = false;
    for msg in logs {
        if msg.contains("InitializeMint2") || msg.contains("Create Metadata Accounts v3") {
            a = true;
        } else if msg.contains("Buy") {
            b = true;
            break;
        }
    }

    if a && b {
        return true;
    }

    return false;
}

pub async fn process_logs(
    logs: &Vec<String>,
    client: Arc<RpcClient>,
    PAYER: Arc<Keypair>,
    investment_lamported: f64,
    slippage: f64,
    adjusted_investment_for_fees: f64,
    unit_limit_ix: Instruction,
    prices_4_spam: Vec<Instruction>,
    m_pk: &Pubkey,
) {
    let mut mint = Pubkey::default();
    let mut bc_pk = Pubkey::default();
    let mut user = Pubkey::default();
    let mut logs_counted = 0;
    let mut virtual_sol_reserves = 0;
    let mut virtual_token_reserves = 0;

    if valid_logs(logs).await {
        for log in logs {
            if log.contains("Program data:") {
                let log_data = log.replace("Program data: ", "");
                let log_decoded = base64::decode(&log_data).expect("Failed to decode base64");

                let len_data = log_decoded.len();

                if len_data >= 180 && logs_counted == 0 {
                    logs_counted = logs_counted + 1;

                    // Convert slices to arrays safely
                    if let Some(user_bytes) = log_decoded
                        .get(len_data - 32..len_data)
                        .and_then(|slice| slice.try_into().ok())
                    {
                        user = Pubkey::new_from_array(user_bytes);
                        if let Some(bonding_curve_bytes) = log_decoded
                            .get(len_data - 64..len_data - 32)
                            .and_then(|slice| slice.try_into().ok())
                        {
                            bc_pk = Pubkey::new_from_array(bonding_curve_bytes);
                            if let Some(mint_bytes) = log_decoded
                                .get(len_data - 96..len_data - 64)
                                .and_then(|slice| slice.try_into().ok())
                            {
                                let mint_temp = Pubkey::new_from_array(mint_bytes);
                                if mint_temp.to_string().contains("pump") {
                                    mint = mint_temp;
                                }
                            }
                        }
                    }
                } else if logs_counted > 0 {
                    //get bonding curve data from the create txn directly...
                    let trade_event = TradeEvent::decode_trade_event(&log_decoded[8..]);
                    virtual_sol_reserves = trade_event.get_virtual_sol_reserves();
                    virtual_token_reserves = trade_event.get_virtual_token_reserves();
                }
            }
        }
    }

    // check and send,,,,,,,,
    if user != Pubkey::default()
        && mint != Pubkey::default()
        && bc_pk != Pubkey::default()
        && virtual_sol_reserves > 0
        && virtual_token_reserves > 0
    {
        println!(
            "{}:: user: {:?} \nmint: {:?}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            user,
            mint
        );
        // println!("-----------------");

        let bc_pk_ata = Pubkey::find_program_address(
            &[
                &bc_pk.to_bytes(),
                &TOKEN_PROGRAM_ID.to_bytes(),
                &mint.to_bytes(),
            ],
            &ASSOCIATED_TOKEN_PROGRAM_ID,
        )
        .0;

        // println!("BC ATA:{}", &bc_pk_ata);

        // price and tokens calcualtion

        let final_with_slippage_int = get_sol2tokens(
            virtual_sol_reserves,
            virtual_token_reserves,
            investment_lamported,
            slippage,
        )
        .await
        .expect("Failed to get price, terminating program.");

        println!("final_with_slippage_int: {}", final_with_slippage_int);

        // --------------------------------
        //create token ata.
        let mint_ata =
            spl_associated_token_account::get_associated_token_address(&PAYER.pubkey(), &mint);

        let ix_ata: Instruction =
            spl_associated_token_account::instruction::create_associated_token_account(
                &PAYER.pubkey(),
                &PAYER.pubkey(),
                &mint,
                &TOKEN_PROGRAM_ID,
            );

        // buy ix-----------
        let buy_ix = get_buy_ix(
            final_with_slippage_int as u64,
            adjusted_investment_for_fees as u64,
            mint,
            bc_pk,
            bc_pk_ata,
            mint_ata,
            PAYER.as_ref(),
        )
        .unwrap();

        // tx info--------------------
        let ixs: Vec<Instruction> = vec![ix_ata, buy_ix, unit_limit_ix.clone()];

        spammer(prices_4_spam.clone(), &client, &PAYER, &m_pk, &ixs).await;

        // // incase you wanted to exit on specific profits......  not fully implemented
        // let mut account_token_balance = 0;
        // let start_checking_balance = Instant::now();
        // loop {
        //     match client
        //         .get_token_account_balance_with_commitment(&mint_ata, CommitmentConfig::processed())
        //         .await
        //     {
        //         Ok(account_balance) => {
        //             let amount: u64 = account_balance.value.amount.parse::<u64>().unwrap_or(0);
        //             account_token_balance = amount;
        //             if amount > 0 {
        //                 println!(
        //                     "{}::Balance found: {:?}",
        //                     Local::now().format("%Y-%m-%d %H:%M:%S"),
        //                     &amount
        //                 );

        //                 let duration = start_checking_balance.elapsed(); //current time
        //                 println!("Time Consumed to amount shown in account: {:?}", duration); //print it
        //                 break;
        //             }
        //         }
        //         Err(e) => {
        //             tokio::time::sleep(Duration::from_millis(100)).await;
        //             // println!("Failed to fetch balance: {:?}", e);
        //             // if start_checking_balance.elapsed() > Duration::from_secs(20) {
        //             //     println!("Balance not found");
        //             //     break;
        //             // }
        //         }
        //     }
        // }

        //----------------------------------------------------------------
        //----------------------------------------------------------------
        println!("Going to sleep"); //----------------------------------------------------------------
        tokio::time::sleep(Duration::from_secs(10)).await;
        //----------------------------------------------------------------
        //----------------------------------------------------------------
        //----------------------------------------------------------------

        let sell_ix = create_sell_ix(
            final_with_slippage_int as u64,
            0 as u64,
            mint,
            bc_pk,
            bc_pk_ata,
            mint_ata,
            PAYER.as_ref(),
        )
        .unwrap();

        let close_acc_ix = close_account(
            &TOKEN_PROGRAM_ID,
            &mint_ata,
            &PAYER.pubkey(),
            &PAYER.pubkey(),
            &[&PAYER.pubkey()],
        )
        .unwrap();

        let ixs_sell: Vec<Instruction> = vec![sell_ix, unit_limit_ix.clone()];

        //          let recent_blockhash1 = client.get_latest_blockhash_with_commitment(CommitmentConfig::processed()).await.unwrap(); //get blockhash
        //        let tx = Transaction::new_signed_with_payer(&ixs_sell,Some(&PAYER.pubkey()), &[&PAYER], recent_blockhash1.0);
        //            let sig = client.send_transaction(&tx).await.unwrap();
        //   println!("sig: {}",&sig.to_string());

        println!("going to spam sell");
        spammer(prices_4_spam.to_vec(), &client, &PAYER, &m_pk, &ixs_sell).await;

        println!("{}::DOne", Local::now().format("%Y-%m-%d %H:%M:%S"));
        println!("------------------------------------------------------------------");
    }
}
