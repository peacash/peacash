use crate::types;
pub const BLOCK_TRANSACTIONS_LIMIT: usize = BLOCK_TIME_MIN as usize * 100;
pub const BLOCK_STAKES_LIMIT: usize = BLOCK_TIME_MIN as usize;
pub const PENDING_TRANSACTIONS_LIMIT: usize = BLOCK_TRANSACTIONS_LIMIT;
pub const PENDING_STAKES_LIMIT: usize = BLOCK_STAKES_LIMIT;
pub const PREFIX_ADDRESS: &str = "0x";
pub const PREFIX_ADDRESS_KEY: &str = "SECRETx";
pub const PROTOCOL_VERSION: &str = "peacash/1.0.0";
pub const SYNC_BLOCKS_PER_TICK: usize = 16;
pub const DECIMAL_PLACES: u32 = 18;
pub const COIN: u128 = 10_u128.pow(DECIMAL_PLACES);
pub const BLOCK_TIME_MIN: u32 = 1;
pub const TIME_DELTA: u32 = 1; // ping delay & perception of time
pub const BLOCK_TIME_MAX: u32 = BLOCK_TIME_MIN + TIME_DELTA;
pub const EXTENSION: &str = "pea";
pub const AMOUNT_BYTES: usize = 4;
pub const GENESIS_BETA: types::Beta = [0; 32];
