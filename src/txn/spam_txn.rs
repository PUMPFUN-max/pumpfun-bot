use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, compute_budget::ComputeBudgetInstruction,
    instruction::Instruction, pubkey::Pubkey, signature::Keypair, transaction::Transaction,
};
use std::sync::Arc;
use std::time::Instant;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

pub async fn spammer(
    prices_4_spam: Vec<Instruction>,
    client: &Arc<RpcClient>,
    PAYER: &Arc<Keypair>,
    m_pk: &Pubkey,
    instructions_vec: &Vec<Instruction>,
) {
    let start = Instant::now(); //get time
    let mut handles: Vec<JoinHandle<Option<String>>> = Vec::new();

    let recent_blockhash1 = client
        .get_latest_blockhash_with_commitment(CommitmentConfig::processed())
        .await
        .unwrap(); //get blockhash
                   // println!("Hash : {}", recent_blockhash1.0.to_string());
    for price_ix in prices_4_spam {
        let recent_blockhash_clone = recent_blockhash1.clone();

        let mut ix_vec = instructions_vec.clone();
        let client_clone = client.clone();
        let payer_clone = PAYER.clone();

        ix_vec.push(price_ix);

        let tx = Transaction::new_signed_with_payer(
            &ix_vec,
            Some(&m_pk),
            &[&payer_clone],
            recent_blockhash_clone.0,
        );

        let handle = tokio::spawn(async move {
            match client_clone.send_transaction(&tx).await {
                Ok(signature) => Some(signature.to_string()),

                Err(e) => {
                    // eprintln!("Failed to send transaction: {}", e);
                    None
                }
            }
        });

        handles.push(handle);
    }

    // comment from here until line 67 if you dont wanna wait for those threads....
    // wait for the above to finish and then print em....
    let mut signatures = Vec::new();
    for handle in handles {
        if let Ok(Some(sig)) = handle.await {
            // println!("Transaction signature: {}", sig);
            signatures.push(sig);
        }
    }
    let successful_txns = signatures.len();

    println!("Successful Txns: {:?}", successful_txns);
    //------------------

    let duration = start.elapsed(); //current time
    println!("Time Consumed to send txns: {:?}", duration); //print it
}

pub async fn array_of_fees(spam_amount: u64, spam_price: u64) -> Vec<Instruction> {
    // Declare an array to store the ComputeBudgetInstruction
    let mut instructions = Vec::new();

    // Loop through the range from 0 to spam_amount
    for i in 0..spam_amount {
        // Create a ComputeBudgetInstruction with the incremented spam_price
        let unit_price_ix = ComputeBudgetInstruction::set_compute_unit_price(spam_price + i);

        // Add the instruction to the array
        instructions.push(unit_price_ix);
    }

    // Return the array of instructions
    instructions
}
