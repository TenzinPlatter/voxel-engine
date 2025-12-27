# AGENTS.md - Voxel Engine Codebase Guide

This document provides essential information for AI agents working in this Rust-based voxel engine codebase.

## Project Overview

A Minecraft-like voxel engine built in Rust using OpenGL 3.3 for rendering. The engine features:
- Voxel-based world with AABB collision detection
- First-person player controller with mouse look and WASD movement
- Texture atlas system for efficient texture management
- Fixed timestep physics simulation (120 Hz)
- Real-time mesh generation from voxel data

## Essential Commands

### Build & Run
```bash
cargo build              # Build the project
cargo run                # Build and run
cargo run --release      # Build with optimizations and run
cargo check              # Fast compile check without producing binary
```

### Development
```bash
cargo clippy             # Run linter
cargo fmt                # Format code (see rustfmt.toml for style)
cargo clean              # Clean build artifacts
```

### Git
The project uses git. Recent commits show active development on:
- Texture atlas implementation
- Voxel raycast/selection ("looking at voxel")
- Block type toggling on click

## Code Organization

### Module Structure

```
src/
├── main.rs              # Entry point: game loop, event handling, rendering
├── lib.rs               # Library root: SDL/window init, utilities
├── engine/              # Core voxel engine logic
│   ├── mod.rs
│   ├── block.rs         # BlockType enum (Dirt, Stone, etc.)
│   ├── voxel.rs         # Voxel struct with geometry generation
│   └── world.rs         # World struct (HashMap<IVec3, Voxel>), collision, raycasting
├── render/              # OpenGL rendering pipeline
│   ├── mod.rs           # Common render functions (polygon_mode, clear_color, draw_voxel_at)
│   ├── atlas.rs         # Texture atlas generation and UV mapping
│   ├── buffer.rs        # OpenGL buffer abstractions (VBO, VAO)
│   ├── camera.rs        # First-person camera with yaw/pitch
│   ├── mesh.rs          # Mesh struct (VAO, VBO, vertex count, transform, texture)
│   ├── renderer.rs      # Renderer struct with shader program
│   ├── shader.rs        # Shader compilation and uniform setting
│   ├── texture.rs       # OpenGL texture wrapper
│   └── vertex.rs        # Vertex traits and types (VertexTex, VertexColor)
├── physics/
│   └── mod.rs           # PhysicsBody, AABB collision, gravity constants
├── input.rs             # Keyboard input state (WASD, Space)
└── player.rs            # Player controller with physics and camera

shaders/
├── vertex.glsl          # Vertex shader (MVP transforms)
└── fragment.glsl        # Fragment shader (texture sampling)

assets/
└── textures/
    └── blocks/
        ├── key.json     # Texture atlas key (maps block names to image files)
        └── imgs/        # Individual block texture PNG files
```

### Key Data Structures

- **World**: `HashMap<IVec3, Voxel>` - Sparse voxel storage with mesh cache
- **Voxel**: Position (PhysicsBody) + BlockType - Generates 36 vertices (6 faces × 6 vertices)
- **BlockType**: Enum of block types (Dirt, Stone) with string mapping for atlas lookup
- **PhysicsBody**: Position, velocity, size, accumulator for fixed timestep
- **Mesh**: VAO, VBO, vertex count, transform matrix, texture handle
- **TextureAtlas**: Generated from `key.json`, writes debug output to `~/.cache/voxel-engine/atlas.png`

## Code Conventions

### Style Rules (from rustfmt.toml)
- **Max line width**: 130 characters
- **Indentation**: 4 spaces (not tabs)
- **Edition**: 2024

### Naming Patterns
- **Modules**: lowercase snake_case (e.g., `texture.rs`, `atlas.rs`)
- **Structs**: PascalCase (e.g., `PhysicsBody`, `TextureAtlas`)
- **Functions**: snake_case (e.g., `rebuild_mesh`, `get_verticies`)
- **Constants**: SCREAMING_SNAKE_CASE (e.g., `PHYSICS_DT`, `GRAVITY`)
- **Enum variants**: PascalCase (e.g., `BlockType::Dirt`)

### Common Patterns

#### Error Handling
- Use `anyhow::Result` for fallible operations
- Use `.context()` to add error context
- Use `bail!()` for custom errors
- Use `.expect()` with descriptive messages for unrecoverable errors

```rust
pub fn try_parse_block_atlas() -> Result<Self> {
    let textures: BTreeMap<String, TextureAtlasKeyEntry> = 
        serde_json::from_str(key_contents).context("Failed to parse key.json")?;
    // ...
}
```

#### OpenGL Usage
- Unsafe blocks required for all OpenGL calls (via `gl33` crate)
- VAO must be bound before VBO operations
- Textures bound via `texture.bind()`
- Always check shader compilation/linking with error messages

```rust
unsafe {
    glViewport(0, 0, drawable_width, drawable_height);
    glEnable(GL_DEPTH_TEST);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
}
```

#### Vector Math
- Uses `glam` crate (version 0.30.9)
- Coordinate system: Right-handed, Y-up
- Common types: `Vec3`, `Vec2`, `IVec3`, `Mat4`
- Bytemuck integration for GPU uploads

```rust
let model = Mat4::from_translation(pos.as_vec3());
Mat4::set_uniform(shader_program, "model", model);
```

#### Physics
- Fixed timestep: `PHYSICS_DT = 1.0 / 120.0` (120 Hz)
- Gravity: `GRAVITY = -9.81`
- Accumulator pattern for consistent physics regardless of frame rate

```rust
self.body.accumulator += frame_delta;
while self.body.accumulator > PHYSICS_DT {
    // apply physics
    self.body.accumulator -= PHYSICS_DT;
}
```

## Architecture Patterns

### Initialization Flow
1. `env_logger::init()` - Set up logging
2. `init_sdl_and_win()` - Create SDL context and OpenGL window
3. Load OpenGL function pointers via `load_global_gl`
4. Create `Renderer` with shaders
5. Build `Resources` (texture atlas)
6. Initialize `World`, `Player`, `InputState`
7. Generate initial world mesh with `create_mesh()`
8. Enter game loop

### Game Loop (main.rs)
1. Calculate delta time
2. Clear color and depth buffers
3. Poll SDL events (keyboard, mouse, quit)
4. Update player physics (fixed timestep)
5. Raycast to find voxel player is looking at
6. Handle mouse clicks (toggle block type, rebuild mesh)
7. Render world mesh
8. Swap window buffers

### Mesh Rebuilding
When voxels change:
1. Clear vertex vector
2. Iterate all voxels, call `voxel.get_verticies(resources)`
3. Each voxel returns 36 vertices (6 faces × 2 triangles × 3 vertices)
4. Create new `Mesh` from vertices
5. Old mesh is dropped (OpenGL resources cleaned up)

**Performance note**: Full mesh rebuild on every voxel change is slow. TODO: chunk-based mesh updates.

### Texture Atlas
- Defined in `assets/textures/blocks/key.json`
- Maps block names to PNG files in `assets/textures/blocks/imgs/`
- Atlas generated at startup with 1px padding
- UV coordinates calculated with 0.5px inset to prevent bleeding
- Debug output written to `~/.cache/voxel-engine/atlas.png`

## Important Gotchas

### 1. Voxel Positions are Integer-Based
Voxels are stored at `IVec3` positions (integer coordinates), but rendered at `Vec3` (float) positions. When setting voxels or checking collisions, ensure you're using the correct type.

```rust
// Correct
world.set_voxel(IVec3::new(0, 0, 0));

// Wrong - won't compile
world.set_voxel(Vec3::new(0.0, 0.0, 0.0));
```

### 2. Mesh Must Be Rebuilt After Voxel Changes
The `World` struct caches a single `Mesh` containing all voxels. Modifying `world.voxels` directly won't update the visual representation - you must call `world.rebuild_mesh(&resources)`.

```rust
// Add voxel
world.set_voxel(IVec3::new(5, 0, 5));
world.rebuild_mesh(&resources);  // Required!
```

### 3. Physics Bodies Use Bottom Corner as Origin
`PhysicsBody.position` represents the **bottom-left-back corner**, not the center. This matters for AABB collision detection.

```rust
// Player body: position is at feet, extends up 1.8 units
Player::new(Vec3::new(0.0, 2.0, 0.0));  // Spawns at y=2.0 (feet)
// Body extends from y=2.0 to y=3.8
```

### 4. Camera Euler Angles in Radians
Camera yaw/pitch are stored in **radians**, not degrees. Use helper functions for conversion.

```rust
use crate::{degrees_to_radians, radians_to_degrees};

let fov_radians = degrees_to_radians(45.0);
```

### 5. Mouse Sensitivity Applied in Player, Not Input
Mouse delta is processed directly in `player.process_mouse()`, not stored in `InputState`. Input state only tracks keyboard.

### 6. Fixed Timestep Accumulator
The physics accumulator can run multiple physics steps per frame or skip frames entirely. Don't assume 1:1 correspondence between frames and physics updates.

```rust
// Could run 0, 1, 2, or more times per frame
while self.body.accumulator > PHYSICS_DT {
    // physics update
}
```

### 7. Texture Atlas Key Must Match BlockType Enum
Block names in `key.json` must match `BlockType::as_str()` output exactly. Adding a new block requires:
1. Add variant to `BlockType` enum
2. Add case to `as_str()` method
3. Add entry to `key.json`
4. Add PNG file to `assets/textures/blocks/imgs/`

```rust
// block.rs
pub enum BlockType {
    Dirt,
    Stone,
    Grass,  // New block
}

impl BlockType {
    pub fn as_str(&self) -> &'static str {
        match self {
            BlockType::Dirt => "dirt",
            BlockType::Stone => "stone",
            BlockType::Grass => "grass",  // Must match key.json
        }
    }
}
```

### 8. Window Title Has Special Meaning
The window title is set to `"(float)"` to integrate with the developer's niri window manager (tiling WM that floats windows with this title). Don't remove this unless you know what you're doing.

```rust
// lib.rs
const WINDOW_TITLE: &str = "(float)";  // For niri WM
```

### 9. Vertex Attribute Layout is Hardcoded
Vertex shaders expect specific attribute locations:
- Location 0: `vec3 position`
- Location 1: `vec2 tex` (for VertexTex) or `vec3 color` (for VertexColor)

Changing vertex structure requires updating both Rust and GLSL.

### 10. No Backface Culling
The engine doesn't enable backface culling, so all 6 faces of every voxel are rendered even if hidden. This is inefficient but simple. Adding culling would require checking neighbor voxels during mesh generation.

## Dependencies & Libraries

### Core Dependencies
- **gl33** (0.2.1): OpenGL 3.3 bindings (raw, unsafe API)
- **beryllium** (0.13.3): SDL2 wrapper for windowing and input
- **glam** (0.30.9): Math library (vectors, matrices) with GPU-friendly layout
- **bytemuck** (1.24.0): Safe transmutation for uploading vertex data to GPU

### Utility Dependencies
- **anyhow** (1.0.100): Error handling with context
- **log** (0.4.29) + **env_logger** (0.11.8): Logging (use `RUST_LOG=debug cargo run`)
- **serde** (1.0.228) + **serde_json** (1.0.145): JSON parsing for texture atlas key
- **num_enum** (0.7.5): `TryFromPrimitive` for BlockType conversions
- **imagine** (0.5.3) + **image** (0.25.9): Image loading and manipulation
- **rand** (0.9.2): Random block type selection in `World::set_voxel`

### Logging
Enable debug logging with:
```bash
RUST_LOG=debug cargo run
```

## Testing Approach

**No tests currently exist in this project.** The codebase uses manual testing via:
1. Run the application
2. Visual inspection of rendering
3. Interactive testing of controls (WASD, mouse, clicking blocks)

When adding tests:
- Physics/collision: Unit tests with known positions
- Math functions: Test `degrees_to_radians`, vector operations
- Block type conversions: Test enum <-> string <-> ID mappings
- Avoid testing OpenGL code directly (requires headless context)

## Common Development Tasks

### Adding a New Block Type

1. Add enum variant to `src/engine/block.rs`:
```rust
pub enum BlockType {
    Dirt,
    Stone,
    Grass,  // New
}
```

2. Update `as_str()`:
```rust
BlockType::Grass => "grass",
```

3. Add to `assets/textures/blocks/key.json`:
```json
{
  "dirt": { "img": "dirt.png" },
  "stone": { "img": "stone.png" },
  "grass": { "img": "grass.png" }
}
```

4. Add `grass.png` to `assets/textures/blocks/imgs/`

### Changing World Generation
Modify `create_mesh()` in `src/lib.rs`:
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

### Modifying Shaders
Shaders are embedded at compile time via `include_str!` in `main.rs`:
```rust
const VERT_SHADER: &str = include_str!("../shaders/vertex.glsl");
const FRAG_SHADER: &str = include_str!("../shaders/fragment.glsl");
```

After editing `.glsl` files, **rebuild the project** (changes won't be hot-reloaded).

### Debugging Texture Atlas
The atlas is written to disk on startup:
```bash
open ~/.cache/voxel-engine/atlas.png  # macOS
xdg-open ~/.cache/voxel-engine/atlas.png  # Linux
```

Check this image if textures appear incorrect - it shows the actual UV layout.

### Adjusting Player Physics
Constants in `src/player.rs`:
```rust
const DEFAULT_PLAYER_SPEED: f32 = 6.0;           // Movement speed
const DEFAULT_PLAYER_JUMP_HEIGHT: f32 = 1.25;   // Jump height
const DEFAULT_PLAYER_REACH: f32 = 5.0;           // Raycast distance
const DEFAULT_MOUSE_SENS: f32 = 0.2;             // Mouse sensitivity
```

Constants in `src/physics/mod.rs`:
```rust
pub const PHYSICS_DT: f32 = 1. / 120.;  // Physics timestep
pub const GRAVITY: f32 = -9.81;          // Gravity acceleration
```

## Known TODOs and Limitations

From inline comments in the code:

1. **No 2D UI rendering pipeline** (main.rs:45) - Crosshair is missing, need 2D overlay system
2. **Slow per-voxel buffer updates** (render/mod.rs:84) - `draw_voxel_at` rebuffers every call
3. **Full world mesh rebuild** (world.rs:17) - Should chunk the world for partial updates
4. **No position clamping** (player.rs:56) - Player can fall infinitely
5. **Naive collision detection** (world.rs:48) - Checks every voxel, needs spatial partitioning
6. **No extrusion prevention** (world.rs:47) - High speeds can phase through floors
7. **No backface culling** - All 6 faces rendered even if hidden by neighbors
8. **Raycasting uses integer steps** (world.rs:60) - Could miss voxels, should use DDA
9. **Random block assignment** (world.rs:30-35) - `set_voxel` randomly picks Dirt or Stone

## Environment Setup

### Linux
Requires SDL2 development libraries:
```bash
# Ubuntu/Debian
sudo apt-get install libsdl2-dev

# Fedora
sudo dnf install SDL2-devel

# Arch
sudo pacman -S sdl2
```

### macOS
Uses `GlContextFlags::FORWARD_COMPATIBLE` (see lib.rs:50-51). SDL2 can be installed via Homebrew:
```bash
brew install sdl2
```

### Windows
Not explicitly tested, but beryllium supports Windows. May need SDL2.dll in path.

## Performance Considerations

### Current Bottlenecks
1. **Full mesh rebuild**: Every voxel change regenerates entire world mesh
2. **No frustum culling**: All voxels rendered even if off-screen
3. **No occlusion culling**: Hidden faces are rendered
4. **Immediate mode collision**: Checks all voxels every frame

### Optimization Opportunities
1. Implement chunk-based mesh updates (16×16×16 chunks)
2. Only rebuild meshes for modified chunks
3. Greedy meshing to combine adjacent voxel faces
4. Spatial hash for collision queries
5. Only generate visible faces (check neighbors)
6. Index buffer reuse for cube geometry

### Current Scale
The default world is 64×64×1 (4096 voxels), which is manageable. Larger worlds will require chunking.

## Reference Materials

### External Resources
- **Coordinate system**: Right-handed, Y-up (OpenGL convention)
- **Texture coords**: Origin at top-left (0,0), bottom-right (1,1)
- **OpenGL docs**: https://docs.gl/ (for GL 3.3 reference)
- **glam docs**: https://docs.rs/glam/ (for vector math)

### Related Files
- **Plans**: `docs/plans/2025-12-02-renderer-architecture-refactor-design.md` contains design notes for future renderer improvements

## Final Notes

This is an active project under development. The architecture prioritizes simplicity and learning over performance. Many "proper" game engine features (ECS, asset management, threading) are intentionally absent.

When making changes:
- Maintain the current architecture unless explicitly refactoring
- Match existing code style and patterns
- Test changes by running the application visually
- Update this document if you discover new patterns or gotchas
- Don't optimize prematurely - keep code readable and simple

The codebase is well-structured for a small engine. Most files are under 200 lines and focused on a single responsibility.
