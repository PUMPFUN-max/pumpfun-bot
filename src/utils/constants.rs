use solana_sdk::{pubkey, pubkey::Pubkey};

pub const MINT_LEN: usize = 82;
pub const ACCOUNT_LEN: usize = 165;
pub const MULTISIG_LEN: usize = 355;

pub const WSOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
    pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
pub const TOKEN_PROGRAM_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub const TOKEN_PROGRAM_ID_2022: Pubkey = pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

pub const RAY_V4: Pubkey = pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
pub const RAY_AUTHORITY_V4: Pubkey = pubkey!("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1");
pub const OPEN_BOOK_PROGRAM: Pubkey = pubkey!("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX");

pub const PUMPFUN_PROGRAM: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
pub const PUMPFUN_GLOBAL: Pubkey = pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf");
pub const PUMPFUN_FEE_RECIPENT: Pubkey = pubkey!("CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM");
pub const PUMPFUN_EVENT_AUTHORITY: Pubkey = pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1");

pub const DEFAULT_BUY: [u8; 8] = [0x66, 0x06, 0x3d, 0x12, 0x01, 0xda, 0xeb, 0xea];
pub const DEFAULT_SELL: [u8; 8] = [0x33, 0xe6, 0x85, 0xa4, 0x01, 0x7f, 0x83, 0xad];

pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;
pub const LIQUIDITY_FEES_NUMERATOR: u32 = 25;
pub const LIQUIDITY_FEES_DENOMINATOR: u32 = 10_000;

//  https://frankfurt.mainnet.block-engine.jito.wtf
//  https://ny.mainnet.block-engine.jito.wtf
// https://tokyo.mainnet.block-engine.jito.wtf
//  https://amsterdam.mainnet.block-engine.jito.wtf
//  ntp.dallas.jito.wtf

pub const BLOCK_ENGINE_URL: &str = "ny.mainnet.block-engine.jito.wtf";
