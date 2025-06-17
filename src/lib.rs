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
