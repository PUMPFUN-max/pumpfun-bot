use std::ops::{Add, Div, Mul};

pub async fn get_sol2tokens(
    virtual_sol_reserves: u64,
    virtual_token_reserves: u64,
    investment_lamported: f64,
    slippage: f64,
) -> Result<f64, Box<dyn std::error::Error>> {
    // let bc_info: solana_client::rpc_response::Response<Option<solana_sdk::account::Account>> =
    //     client
    //         .get_account_with_commitment(&bc_pk, CommitmentConfig::processed())
    //         .await
    //         .unwrap();
    // let bc_decoded =
    //     BondingCurveLayoutPF::decode_pump_fun_bonding_curve_info(&bc_info.value.unwrap().data[8..]);
    // // println!("bc_info: {:?}",bc_decoded);

    // let v_tokens = (bc_decoded.get_virtual_token_reserves()) as f64;
    // let v_sol = (bc_decoded.get_virtual_sol_reserves()) as f64;

    let v_tokens = (virtual_token_reserves) as f64;
    let v_sol = (virtual_sol_reserves) as f64;

    let price = v_sol.div(v_tokens);

    let total_tokens = investment_lamported.div(price);
    let slippage_tokens = total_tokens.mul(slippage);
    let final_with_slippage_int = total_tokens - slippage_tokens;

    // let final_with_slippage_int = 0.0;

    // println!("final_with_slippage_int: {}", final_with_slippage_int);

    Ok(final_with_slippage_int)
}

// let _completed = bc_decoded.complete_bool();
// let price = vSOL as f64 / vTokens as f64;

// println!("BC Decoded: \n{:?}", &bc_decoded);
// let is_complete = bc_decoded.complete_bool();
// println!("Is complete: {}", is_complete);

// println!("Price: {}", &price);
