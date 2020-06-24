use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferFileMeta {
    pub name: String,
    pub size: u64,
}
