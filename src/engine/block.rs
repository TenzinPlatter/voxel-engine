use anyhow::Result;
use glam::Vec2;
use num_enum::TryFromPrimitive;
use serde::Deserialize;

use std::collections::HashMap;
use std::fmt::Display;

use crate::Resources;

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockType {
    Dirt,
    Grass,
    Stone,
}

#[derive(Deserialize)]
pub struct BlockAtlasEntry {
    #[serde(deserialize_with = "deserialize_grid_index_to_uv")]
    pub x: f32,
    #[serde(deserialize_with = "deserialize_grid_index_to_uv")]
    pub y: f32,
    #[serde(deserialize_with = "deserialize_pixel_size_to_uv", default = "default_sprite_size")]
    pub width: f32,
    #[serde(deserialize_with = "deserialize_pixel_size_to_uv", default = "default_sprite_size")]
    pub height: f32,
}

pub type BlockAtlas = HashMap<BlockType, BlockAtlasEntry>;

impl BlockType {
    pub fn to_str(&self) -> &'static str {
        match self {
            BlockType::Dirt => "dirt",
            BlockType::Grass => "grass",
            BlockType::Stone => "stone",
        }
    }
}

impl BlockAtlasEntry {
    /// Returns the four UV corners of this texture region.
    /// Order: [bottom_left, bottom_right, top_right, top_left]
    /// This matches the vertex generation order in Voxel::get_verticies.
    pub fn as_uv_corners(&self) -> [Vec2; 4] {
        [
            Vec2::new(self.x, self.y + self.height),           // Bottom-left
            Vec2::new(self.x + self.width, self.y + self.height), // Bottom-right
            Vec2::new(self.x + self.width, self.y),            // Top-right
            Vec2::new(self.x, self.y),                         // Top-left
        ]
    }
}

impl Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

pub fn parse_block_atlas() -> BlockAtlas {
    toml::from_str(include_str!("../../assets/blocks.toml"))
        .expect("Failed to parse blocks.toml")
}

pub fn texture_coordinates_from_block_id(resources: &Resources, block_type: BlockType) -> &BlockAtlasEntry {
    resources
        .block_atlas
        .get(&block_type)
        .unwrap_or_else(|| panic!("Block type: {} is not in the texture atlas keys", block_type))
}

pub fn block_type_from_id(id: u8) -> Option<BlockType> {
    id.try_into().ok()
}

pub fn block_id_from_type(block_type: BlockType) -> u8 {
    block_type as u8
}

/// Deserializes grid indices (x, y position) to UV coordinates (0-1 range).
/// For a 512px sprite sheet with 16px sprites:
/// - Grid index 0 → 0.0 UV
/// - Grid index 1 → 0.03125 UV (16/512)
/// - Grid index 2 → 0.0625 UV (32/512)
fn deserialize_grid_index_to_uv<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    const ATLAS_SIZE_PX: f32 = 512.0;
    const SPRITE_SIZE_PX: f32 = 16.0;
    let grid_index: f32 = Deserialize::deserialize(deserializer)?;
    Ok((grid_index * SPRITE_SIZE_PX) / ATLAS_SIZE_PX)
}

/// Deserializes pixel sizes (width, height) to UV coordinates (0-1 range).
/// For a 512px sprite sheet:
/// - 16 pixels → 0.03125 UV (16/512)
/// - 32 pixels → 0.0625 UV (32/512)
fn deserialize_pixel_size_to_uv<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    const ATLAS_SIZE_PX: f32 = 512.0;
    let pixels: f32 = Deserialize::deserialize(deserializer)?;
    Ok(pixels / ATLAS_SIZE_PX)
}

/// Default sprite size: 16 pixels
fn default_sprite_size() -> f32 {
    16. / 512.
}
