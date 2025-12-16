use anyhow::{Result, bail};
use glam::{UVec2, Vec2};
use image::{ImageBuffer, Rgba};
use serde::Deserialize;

use std::{collections::HashMap, path::Path};

use crate::render::texture::Texture;

const BLOCK_IMG_PREFIX: &str = "assets/textures/blocks/imgs/";
const BYTES_PER_PX: usize = 4;

pub struct TextureAtlas {
    pub size: Vec2,
    pub textures: HashMap<String, TextureAtlasEntry>,
    pub texture: Texture,
}

/// NOTE: in UV 0 - 1 coords
pub struct TextureAtlasEntry {
    pub top_left: Vec2,
    pub size: Vec2,
}

#[derive(Deserialize)]
struct TextureAtlasKeyEntry {
    img: String,
}

impl TextureAtlasEntry {
    /// Returns the four UV coordinates (Vec2) for the corners of the rectangle
    /// Order: top-left, top-right, bottom-right, bottom-left
    pub fn to_uvs(&self) -> [Vec2; 4] {
        let tl = self.top_left;
        let tr = Vec2::new(self.top_left.x + self.size.x, self.top_left.y);
        let br = Vec2::new(self.top_left.x + self.size.x, self.top_left.y + self.size.y);
        let bl = Vec2::new(self.top_left.x, self.top_left.y + self.size.y);
        [tl, tr, br, bl]
    }
}

impl TextureAtlas {
    /// NOTE: assumes all textures are 16x16
    pub fn try_parse_block_atlas() -> Result<Self> {
        let key_contents = include_str!("../../assets/textures/blocks/key.json");
        let textures: HashMap<String, TextureAtlasKeyEntry> = serde_json::from_str(key_contents)?;
        let (atlas_pixels_buf, (atlas_width_px, atlas_height_px)) = generate_texture_atlas_pixels(&textures)?;
        let textures = get_textures_as_uv(textures, atlas_width_px, atlas_height_px);

        write_image_from_pixels(&atlas_pixels_buf, "/home/tenzin/.cache/voxel-engine/atlas.png");

        let texture = match Texture::new() {
            Some(t) => t,
            None => bail!("Failed to create texture"),
        };

        texture.bind();
        Texture::set_from_pixels(atlas_pixels_buf, atlas_width_px, atlas_height_px);

        Ok(Self {
            size: Vec2::new(atlas_width_px as f32, atlas_height_px as f32),
            textures,
            texture,
        })
    }
}

fn get_textures_as_uv(
    textures: HashMap<String, TextureAtlasKeyEntry>,
    atlas_width_px: usize,
    atlas_height_px: usize,
) -> HashMap<String, TextureAtlasEntry> {
    textures
        .into_iter()
        .enumerate()
        .map(|(i, (name, _entry))| {
            (
                name,
                TextureAtlasEntry {
                    top_left: Vec2::new((i * 16 / atlas_width_px) as f32, (i * 16 / atlas_height_px) as f32),
                    size: Vec2::new(16. / atlas_width_px as f32, 16. / atlas_height_px as f32),
                },
            )
        })
        .collect()
}

/// @return (pixel_buffer, (width, height))
fn generate_texture_atlas_pixels(textures: &HashMap<String, TextureAtlasKeyEntry>) -> Result<(Vec<u8>, (usize, usize))> {
    // width and height in pixels
    let (width_px, height_px) = (1024, 1024);
    let size = width_px * height_px * 4;
    let mut pixel_buf: Vec<u8> = vec![255; size];

    for (i, (name, tex)) in textures.iter().enumerate() {
        let img = image::open(BLOCK_IMG_PREFIX.to_owned() + &tex.img)?;

        if img.width() != 16 || img.height() != 16 {
            bail!("img {} is not 16x16 (got size {}x{})", name, img.width(), img.height());
        }

        let rgba_img = img.to_rgba8();
        let rbga_buf = rgba_img.as_raw();
        let top_left_px = UVec2::new((i % width_px) as u32, (i / height_px) as u32);

        for i in 0..16 {
            // standard offset for 2D coord -> 1D
            let dest_offset_px = (top_left_px.x + (top_left_px.y * 16 * width_px as u32)) as usize;
            let dest_offset = dest_offset_px + (i * width_px) * BYTES_PER_PX;

            // same thing, we just are always copying blocks of 16 (a row)
            let src_offset = (i * 16) * BYTES_PER_PX;

            let nbytes = 16 * BYTES_PER_PX;

            let dest = &mut pixel_buf[dest_offset..dest_offset + nbytes];
            let src = &rbga_buf[src_offset..src_offset + nbytes];

            dest.copy_from_slice(src);
        }
    }

    Ok((pixel_buf, (width_px, height_px)))
}

fn write_image_from_pixels(pixels: &[u8], path: &str) {
    // Assumes the image is square and 4 bytes per pixel (RGBA)
    let side = (pixels.len() / 4).isqrt() as u32;
    let img: ImageBuffer<Rgba<u8>, _> =
        ImageBuffer::from_raw(side, side, pixels.to_vec()).expect("Failed to create image buffer from raw pixels");
    img.save(path).expect("Failed to save image");
}
