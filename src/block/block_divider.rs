use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct DividerBlock {
    pub block_id: Option<String>,
}

impl DividerBlock {
    pub fn new(block_id: String) -> DividerBlock {
        DividerBlock {
            block_id: String(block_id),
        }
    }
}
