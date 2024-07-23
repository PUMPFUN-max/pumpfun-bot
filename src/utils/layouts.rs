use solana_sdk::pubkey::Pubkey;

use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AmmInfoV4 {
    pub status: u64,
    pub nonce: u64,
    pub order_num: u64,
    pub depth: u64,
    pub base_decimals: u64,
    pub quote_decimals: u64,
    pub state: u64,
    pub reset_flag: u64,
    pub min_size: u64,
    pub vol_max_cut_ratio: u64,
    pub amount_wave_ratio: u64,
    pub base_lot_size: u64,
    pub quote_lot_size: u64,
    pub min_price_multiplier: u64,
    pub max_price_multiplier: u64,
    pub system_decimals_value: u64,
    pub min_separate_numerator: u64,
    pub min_separate_denominator: u64,
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub pnl_numerator: u64,
    pub pnl_denominator: u64,
    pub swap_fee_numerator: u64,
    pub swap_fee_denominator: u64,
    pub need_take_pnl_base: u64,
    pub need_take_pnl_quote: u64,
    pub total_pnl_quote: u64,
    pub total_pnl_base: u64,
    pub pool_total_deposit_quote: u128,
    pub pool_total_deposit_base: u128,
    pub swap_base_in_amount: u128,
    pub swap_quote_out_amount: u128,
    pub swap_base2quote_fee: u64,
    pub swap_quote_in_amount: u128,
    pub swap_base_out_amount: u128,
    pub swap_quote2base_fee: u64,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    pub base_mint_address: Pubkey,
    pub quote_mint_address: Pubkey,
    pub lp_mint_address: Pubkey,
    pub amm_open_orders: Pubkey,
    pub serum_market: Pubkey,
    pub serum_program_id: Pubkey,
    pub amm_target_orders: Pubkey,
    pub pool_withdraw_queue: Pubkey,
    pub pool_temp_lp_token_account: Pubkey,
    pub amm_owner: Pubkey,
    pub pnl_owner: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SerumMarket {
    _blob_5: [u8; 5], //KOKIEZ
    account_flags: u64, // Assuming ACCOUNT_FLAGS_LAYOUT is a u64 for this example
    serum_market: Pubkey,
    vault_signer_nonce: u64,
    base_mint: Pubkey,
    quote_mint: Pubkey,
    base_vault: Pubkey,
    base_deposits_total: u64,
    base_fees_accrued: u64,
    quote_vault: Pubkey,
    quote_deposits_total: u64,
    quote_fees_accrued: u64,
    quote_dust_threshold: u64,
    request_queue: Pubkey,
    event_queue: Pubkey,
    bids: Pubkey,
    asks: Pubkey,
    base_lot_size: u64,
    quote_lot_size: u64,
    fee_rate_bps: u64,
    referrer_rebate_accrued: u64,
    _blob_7: [u8; 7],
}

#[inline(never)]
pub fn decode_pool_info(mut pool_info: &[u8]) -> AmmInfoV4 {
    AmmInfoV4::deserialize(&mut pool_info).unwrap()
}

#[inline(never)]
pub fn decode_market_info(mut market_info: &[u8]) -> SerumMarket {
    SerumMarket::deserialize(&mut market_info).unwrap()
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct BondingCurveLayoutPF {
    virtual_token_reserves: u64,
    virtual_sol_reserves: u64,
    real_token_reserves: u64,
    real_sol_reserves: u64,
    token_total_supply: u64,
    complete: bool,
}

// # First 8 bytes = f\x06=\x12\x01\xda\xeb\xea
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct BuyLayoutpf {
    amount: u64,
    max_sol_cost: u64,
}

// # First 8 bytes = b'3\xe6\x85\xa4\x01\x7f\x83\xad'
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SellLayoutpf {
    amount: u64,
    min_sol_output: u64,
}

impl BondingCurveLayoutPF {
    /// Returns `complete` as a boolean.
    pub fn complete_bool(&self) -> bool {
        self.complete
    }

    pub fn get_virtual_token_reserves(&self) -> u64 {
        self.virtual_token_reserves
    }

    // Getter for virtual_sol_reserves
    pub fn get_virtual_sol_reserves(&self) -> u64 {
        self.virtual_sol_reserves
    }

    /// Decodes from bytes, ensuring proper type management.
    #[inline(never)]
    pub fn decode_pump_fun_bonding_curve_info(mut bonding_curve_layout_pf: &[u8]) -> Self {
        let deserialized = Self::deserialize(&mut bonding_curve_layout_pf).unwrap();
        deserialized
    }
}

// trade event
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct TradeEvent {
    pub mint: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub is_buy: bool,
    pub user: Pubkey,
    pub timestamp: i64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
}

impl TradeEvent {
    /// Returns `is_buy` as a boolean.
    pub fn is_buy_bool(&self) -> bool {
        self.is_buy
    }

    pub fn get_sol_amount(&self) -> u64 {
        self.sol_amount
    }

    pub fn get_token_amount(&self) -> u64 {
        self.token_amount
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn get_virtual_sol_reserves(&self) -> u64 {
        self.virtual_sol_reserves
    }

    pub fn get_virtual_token_reserves(&self) -> u64 {
        self.virtual_token_reserves
    }

    /// Decodes from bytes, ensuring proper type management.
    #[inline(never)]
    pub fn decode_trade_event(mut trade_event_data: &[u8]) -> Self {
        let deserialized = Self::deserialize(&mut trade_event_data).unwrap();
        deserialized
    }
}

// bitfield! {
//     pub struct AccountFlags(u64);
//     impl Debug;
//     bool, get_initialized, set_initialized: 0;
//     bool, get_market, set_market: 1;
//     bool, get_open_orders, set_open_orders: 2;
//     bool, get_request_queue, set_request_queue: 3;
//     bool, get_event_queue, set_event_queue: 4;
//     bool, get_bids, set_bids: 5;
//     bool, get_asks, set_asks: 6;
// }

// impl Market {

//     pub fn base_mint(&self) -> &[u8; 32] {
//         &self.base_mint
//     }

//     pub fn from_bytes(bytes: &[u8]) -> Self {
//         let mut idx = 0;
//         let account_flags = AccountFlags(LittleEndian::read_u64(&bytes[idx..idx+8])); idx += 8;
//         let mut own_address = [0u8; 32];
//         own_address.copy_from_slice(&bytes[idx..idx+32]); idx += 32;
//         let vault_signer_nonce = LittleEndian::read_u64(&bytes[idx..idx+8]); idx += 8;
//         let mut base_mint = [0u8; 32];
//         base_mint.copy_from_slice(&bytes[idx..idx+32]); idx += 32;
//         let mut quote_mint = [0u8; 32];
//         quote_mint.copy_from_slice(&bytes[idx..idx+32]); idx += 32;
//         let mut base_vault = [0u8; 32];
//         base_vault.copy_from_slice(&bytes[idx..idx+32]); idx += 32;
//         let base_deposits_total = LittleEndian::read_u64(&bytes[idx..idx+8]); idx += 8;
//         let base_fees_accrued = LittleEndian::read_u64(&bytes[idx..idx+8]); idx += 8;
//         let mut quote_vault = [0u8; 32];
//         quote_vault.copy_from_slice(&bytes[idx..idx+32]); idx += 32;
//         let quote_deposits_total = LittleEndian::read_u64(&bytes[idx..idx+8]); idx += 8;
//         let quote_fees_accrued = LittleEndian::read_u64(&bytes[idx..idx+8]); idx += 8;
//         let quote_dust_threshold = LittleEndian::read_u64(&bytes[idx..idx+8]); idx += 8;
//         let mut request_queue = [0u8; 32];
//         request_queue.copy_from_slice(&bytes[idx..idx+32]); idx += 32;
//         let mut event_queue = [0u8; 32];
//         event_queue.copy_from_slice(&bytes[idx..idx+32]); idx += 32;
//         let mut bids = [0u8; 32];
//         bids.copy_from_slice(&bytes[idx..idx+32]); idx += 32;
//         let mut asks = [0u8; 32];
//         asks.copy_from_slice(&bytes[idx..idx+32]); idx += 32;
//         let base_lot_size = LittleEndian::read_u64(&bytes[idx..idx+8]); idx += 8;
//         let quote_lot_size = LittleEndian::read_u64(&bytes[idx..idx+8]); idx += 8;
//         let fee_rate_bps = LittleEndian::read_u64(&bytes[idx..idx+8]); idx += 8;
//         let referrer_rebate_accrued = LittleEndian::read_u64(&bytes[idx..idx+8]);

//         Self {
//             account_flags,
//             own_address,
//             vault_signer_nonce,
//             base_mint,
//             quote_mint,
//             base_vault,
//             base_deposits_total,
//             base_fees_accrued,
//             quote_vault,
//             quote_deposits_total,
//             quote_fees_accrued,
//             quote_dust_threshold,
//             request_queue,
//             event_queue,
//             bids,
//             asks,
//             base_lot_size,
//             quote_lot_size,
//             fee_rate_bps,
//             referrer_rebate_accrued,
//         }
//     }
// }
