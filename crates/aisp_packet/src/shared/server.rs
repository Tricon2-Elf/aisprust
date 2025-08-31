use crate::util::fixed_array::FixedArray;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct ServerInfo {
    pub port: u16,
    pub address: FixedArray<65>,
}
