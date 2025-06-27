pub mod crypto;
pub mod error;
pub mod sha256;
pub mod types;
pub mod util;

use uint::construct_uint;
construct_uint! {
    #[derive(serde::Serialize,serde::Deserialize)]
    pub struct U256(4);
}

//initial reward to mine a block
pub const INITIAL_REWARD: u64 = 50;

//this halving interval is in terms of block
pub const HALVING_INTERVAL: u64 = 210;

//block time in secs
pub const IDEAL_BLOCK_TIME: u64 = 10;
//minium target
pub const MIN_TARGET: U256 = U256([
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0x0000_FFFF_FFFF_FFFF,
]);

//difficulty interval in blocks
pub const DIFFICULTY_UPDATE_INTERVAL: u64 = 50;
