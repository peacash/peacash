pub type AmountBytes = [u8; AMOUNT_BYTES];
pub type Hash = [u8; 32];
pub type Checksum = [u8; 4];
pub type MerkleRoot = [u8; 32];
pub type Beta = [u8; 32];
pub type Pi = [u8; 81];
pub type AddressBytes = [u8; 20];
pub type PublicKeyBytes = [u8; 33];
pub type SecretKeyBytes = [u8; 32];
pub type SignatureBytes = [u8; 64];
pub const PREFIX_ADDRESS: &str = "0x";
pub const PREFIX_SECRET_KEY: &str = "SECRETx";
pub const BLOCK_TRANSACTIONS_LIMIT: usize = BLOCK_TIME_MIN as usize * 100;
pub const BLOCK_STAKES_LIMIT: usize = BLOCK_TIME_MIN as usize;
pub const PENDING_TRANSACTIONS_LIMIT: usize = BLOCK_TRANSACTIONS_LIMIT;
pub const PENDING_STAKES_LIMIT: usize = BLOCK_STAKES_LIMIT;
pub const PROTOCOL_VERSION: &str = "peacash/1.0.0";
pub const SYNC_BLOCKS_PER_TICK: usize = 2;
pub const DECIMAL_PLACES: usize = 18;
pub const COIN: u128 = 10_u128.pow(DECIMAL_PLACES as u32);
pub const BLOCK_TIME_MIN: u32 = 1;
pub const TIME_DELTA: u32 = 1; // ping delay & perception of time
pub const BLOCK_TIME_MAX: u32 = BLOCK_TIME_MIN + TIME_DELTA;
pub const EXTENSION: &str = "pea";
pub const AMOUNT_BYTES: usize = 4;
pub const GENESIS_BETA: Beta = [0; 32];
pub const RECOVERY_ID: i32 = 0;
pub const RATELIMIT_TOPIC_BLOCK: usize = 100;
pub const RATELIMIT_TOPIC_TRANSACTION: usize = 100;
pub const RATELIMIT_TOPIC_STAKE: usize = 100;
pub const RATELIMIT_TOPIC_MULTIADDR: usize = 100;
pub const RATELIMIT_DURATION: u32 = 60 * 60;
