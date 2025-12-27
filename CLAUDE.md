# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Minecraft-like voxel engine in Rust using OpenGL 3.3. Features first-person camera, physics, texture atlasing, and real-time mesh generation from voxel data.

**Note**: See `AGENTS.md` for comprehensive documentation. This file covers essential quick-start information.

## Essential Commands

### Build & Run
```bash
cargo run                # Build and run (debug mode)
cargo run --release      # Build with optimizations and run
cargo check              # Fast compile check without binary
cargo clippy             # Run linter
cargo fmt                # Format code (max line width: 130 chars)
```

### Logging
```bash
RUST_LOG=debug cargo run  # Enable debug logging
```

### Testing
No test suite currently exists. Testing is done by running the application and visually inspecting rendering + interactive controls (WASD movement, mouse look, left-click to toggle blocks).

## High-Level Architecture

### Core Separation of Concerns

```
Game State (World)          Graphics System (Renderer)      Data (Mesh)
├─ Voxel positions         ├─ Shader program              ├─ VAO/VBO
├─ Collision detection     ├─ Uniform setup               ├─ Transform matrix
└─ Mesh caching            └─ Draw calls                  └─ Texture handle
```

**Critical Pattern**: World owns game state, Renderer handles display. They communicate via Mesh, which bundles geometry + position + texture.

### Key Data Flow

1. **Initialization**: `main.rs` → SDL/OpenGL setup → Renderer + Resources (texture atlas) → World + Player → Generate initial mesh
2. **Game Loop**: Poll input → Update physics (fixed 120Hz timestep) → Raycast for voxel selection → Handle clicks (toggle blocks) → Render
3. **Voxel Changes**: `world.set_voxel()` → Mark dirty → `world.rebuild_mesh(resources)` → Creates new Mesh from all voxels → Old mesh dropped

### Module Responsibilities

- **src/engine/**: Core game logic (Block types, Voxels, World with HashMap<IVec3, Voxel>)
- **src/render/**: OpenGL pipeline (Renderer, Mesh, Shader, Texture, Camera, Atlas)
- **src/physics/**: AABB collision, PhysicsBody with fixed timestep accumulator
- **src/player.rs**: First-person controller (combines PhysicsBody + Camera + Input)
- **src/main.rs**: Game loop, event handling, orchestration

## Critical Gotchas

### 1. Mesh Rebuilding Required After Voxel Changes
World caches a single Mesh containing all voxels. Directly modifying `world.voxels` won't update visuals.

```rust
// WRONG - won't render
world.voxels.insert(IVec3::new(5, 0, 5), voxel);

// CORRECT - updates visual
world.set_voxel(IVec3::new(5, 0, 5));
world.rebuild_mesh(&resources);
```

### 2. Integer vs Float Positions
Voxels use `IVec3` (integer coords) for storage, but `Vec3` (float) for rendering. Don't mix them.

```rust
world.set_voxel(IVec3::new(0, 0, 0));  // ✓ Correct
world.set_voxel(Vec3::new(0.0, 0.0, 0.0));  // ✗ Won't compile
```

### 3. Shaders Embedded at Compile Time
Shaders are loaded via `include_str!` in main.rs. **After editing .glsl files, rebuild the project** - changes won't hot-reload.

```rust
const VERT_SHADER: &str = include_str!("../shaders/vertex.glsl");
const FRAG_SHADER: &str = include_str!("../shaders/fragment.glsl");
```

### 4. Texture Atlas Key Must Match BlockType Enum
Adding a block requires changes in 4 places:
1. Add variant to `BlockType` enum in `src/engine/block.rs`
2. Update `BlockType::as_str()` to return matching string
3. Add entry to `assets/textures/blocks/key.json` with same string
4. Add PNG file to `assets/textures/blocks/imgs/`

### 5. Physics Body Position is Bottom Corner
`PhysicsBody.position` is the **bottom-left-back corner**, not center. Matters for collision detection.

```rust
Player::new(Vec3::new(0.0, 2.0, 0.0));  // Feet at y=2.0, head at y=3.8
```

### 6. Fixed Timestep Accumulator Can Run Multiple Times Per Frame
Physics runs at 120Hz regardless of framerate. Don't assume 1:1 frame-to-physics correspondence.

```rust
while self.body.accumulator > PHYSICS_DT {
    // May run 0, 1, 2+ times per frame
}
```

### 7. Window Title "(float)" is for Niri WM
The title integrates with the developer's tiling window manager. Don't remove unless intentional.

### 8. No Backface Culling
All 6 faces of every voxel are rendered even if hidden. Simple but inefficient. Adding culling requires checking neighbor voxels during mesh generation.

### 9. Texture Atlas Debug Output
Atlas is written to `~/.cache/voxel-engine/atlas.png` on startup. Check this if textures appear wrong.

```bash
xdg-open ~/.cache/voxel-engine/atlas.png  # Linux
```

## Common Patterns

### Error Handling
```rust
use anyhow::{Result, Context, bail};

pub fn fallible_fn() -> Result<T> {
    let data = serde_json::from_str(contents)
        .context("Failed to parse key.json")?;

    if invalid { bail!("Custom error message"); }

    Ok(result)
}
```

### OpenGL Safety
All OpenGL calls are `unsafe`. VAO must be bound before VBO operations.

```rust
unsafe {
    glViewport(0, 0, width, height);
    glEnable(GL_DEPTH_TEST);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
}
```

### Vector Math (glam)
Right-handed coordinate system, Y-up. Types: `Vec3`, `Vec2`, `IVec3`, `Mat4`.

```rust
let model = Mat4::from_translation(pos.as_vec3());
Mat4::set_uniform(shader_program, "model", model);
```

## Performance Notes

### Current Bottlenecks
- **Full mesh rebuild**: Every voxel change regenerates entire world mesh (4096 voxels in default 64×64 world)
- **No frustum culling**: All voxels rendered even if off-screen
- **No occlusion culling**: Hidden internal faces are rendered
- **Immediate collision**: Checks all voxels every frame

### Scale Considerations
Default world is 64×64×1 (4096 voxels). Larger worlds will require chunking (see AGENTS.md for optimization roadmap).

## Key Dependencies

- **gl33**: OpenGL 3.3 bindings (raw unsafe API)
- **beryllium**: SDL2 wrapper for windowing/input
- **glam**: GPU-friendly math library (vectors, matrices)
- **anyhow**: Error handling with context
- **bytemuck**: Safe vertex data transmutation for GPU upload

## Environment Setup

### Linux (required)
```bash
# Ubuntu/Debian
sudo apt-get install libsdl2-dev

# Arch
sudo pacman -S sdl2
```

### macOS
Uses `GlContextFlags::FORWARD_COMPATIBLE` for compatibility.
```bash
brew install sdl2
```

## Quick Reference: Common Tasks

### Add a New Block Type
1. Add enum variant to `src/engine/block.rs`: `BlockType::Grass`
2. Update `as_str()`: `BlockType::Grass => "grass"`
3. Add to `assets/textures/blocks/key.json`: `"grass": { "img": "grass.png" }`
4. Add `grass.png` to `assets/textures/blocks/imgs/`

### Modify World Generation
Edit `create_mesh()` in `src/lib.rs`:
```rust
pub fn create_mesh(world: &mut World, resources: &Resources) {
    for z in -32..32 {
        for x in -32..32 {
            world.set_voxel(IVec3::new(x, 0, z));
        }
    }
    world.rebuild_mesh(resources);
}
```

### Adjust Player Physics
Constants in `src/player.rs`:
- `DEFAULT_PLAYER_SPEED`: Movement speed (default: 6.0)
- `DEFAULT_PLAYER_JUMP_HEIGHT`: Jump height (default: 1.25)
- `DEFAULT_MOUSE_SENS`: Mouse sensitivity (default: 0.2)

Constants in `src/physics/mod.rs`:
- `PHYSICS_DT`: Physics timestep (default: 1/120 sec)
- `GRAVITY`: Gravity acceleration (default: -9.81)

## Known Limitations & TODOs

From inline comments:
1. No 2D UI rendering pipeline (crosshair missing)
2. Slow per-voxel buffer updates (`draw_voxel_at` rebuffers every call)
3. Full world mesh rebuild instead of chunk-based updates
4. No player position clamping (can fall infinitely)
5. Naive collision detection (checks every voxel)
6. High speeds can phase through floors (no extrusion prevention)
7. Raycasting uses integer steps (could miss voxels, should use DDA algorithm)
8. Random block assignment in `set_voxel` (picks Dirt or Stone randomly)

## Philosophy

This codebase prioritizes **simplicity and learning over performance**. Many "proper" game engine features (ECS, asset management, threading) are intentionally absent.

When making changes:
- Maintain current architecture unless explicitly refactoring
- Match existing code style (see rustfmt.toml)
- Keep code readable and simple - don't optimize prematurely
- Test visually by running the application
- Update AGENTS.md if you discover new patterns or gotchas

**Refer to AGENTS.md** for comprehensive architecture details, complete API patterns, and in-depth explanations of all systems.
