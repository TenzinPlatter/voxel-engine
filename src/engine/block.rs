use num_enum::TryFromPrimitive;
use serde::Deserialize;

use std::fmt::Display;


#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockType {
    Dirt,
    Stone,
}

impl BlockType {
    pub fn as_str(&self) -> &'static str {
        match self {
            BlockType::Dirt => "dirt",
            BlockType::Stone => "stone",
        }
    }
}


impl Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub fn block_type_from_id(id: u8) -> Option<BlockType> {
    id.try_into().ok()
}

pub fn block_id_from_type(block_type: BlockType) -> u8 {
    block_type as u8
}
