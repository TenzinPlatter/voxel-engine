# Renderer Architecture Refactor Design

**Date**: 2025-12-02
**Status**: Approved
**Goal**: Separate game state from rendering concerns to prevent bugs like missing model matrix transforms

## Problem Statement

Current architecture has several issues:
1. **No transform abstraction**: Must manually remember to set model matrix before each draw call
2. **Unclear ownership**: World owns shader_program (rendering concern) but also game data (voxels)
3. **Tight coupling**: Rendering logic scattered across World, Mesh, and main.rs
4. **Bug-prone**: Missing model matrix caused recent rendering bug - nothing enforces correct setup

The fundamental issue: no clear separation between "what exists" (game state) and "how to render it" (graphics).

## Design Principles

**Separation of Concerns**:
- World = game state (WHAT exists)
- Renderer = graphics system (HOW to display)
- Mesh = renderable geometry + position (WHAT and WHERE)

**Explicit Control**:
- Mesh rebuilding is explicit via `rebuild_mesh()` call
- Allows batching voxel changes before expensive mesh regeneration

**Safety Through Structure**:
- Renderer handles ALL uniform setup in one place
- Impossible to forget model matrix - it's automatic

## Architecture Overview

### World (Game State)
**Owns**: Set of voxel positions, optional mesh
**Responsibility**: Track which voxels exist, rebuild mesh on demand
**Does NOT know**: Shaders, OpenGL state, cameras, how rendering works

Key operations:
- `set_voxel(position)` - Add voxel to world
- `remove_voxel(position)` - Remove voxel from world
- `rebuild_mesh(texture)` - Generate mesh from current voxel set
- `mesh()` - Get reference to current mesh if it exists

### Renderer (Graphics System)
**Owns**: Shader program
**Responsibility**: Set up OpenGL state, render meshes with proper transforms
**Does NOT know**: What voxels exist, game logic

Key operations:
- `new(vertex_shader, fragment_shader)` - Initialize with shaders
- `render_mesh(mesh, camera, viewport)` - Render a mesh with all proper uniforms

The critical insight: Renderer is the single source of truth for "how to draw a mesh". It handles:
- Shader activation
- Texture binding (from mesh)
- View matrix (from camera)
- Projection matrix (from viewport)
- Model matrix (from mesh transform)
- Draw call

### Mesh (Renderable Geometry)
**Owns**: VAO, VBO, vertex count, transform matrix, texture
**Responsibility**: Bundle of GPU data ready to draw, knows its own position
**Does NOT know**: Cameras, shaders, what it represents in game terms

Key operations:
- `new(vertices, transform, texture)` - Create mesh from vertex data
- `transform()` - Get the mesh's transform matrix
- `texture()` - Get the mesh's texture
- `draw()` - Internal: bind VAO and issue draw call

The key improvement: Mesh now owns its transform, so you can't forget to position it.

## Data Flow

### Voxel Changes
```
User input (e.g., SPACE key)
  → World.set_voxel() or remove_voxel()
  → Set "dirty" flag
  → Later: World.rebuild_mesh()
      → Gather vertices from all voxel positions
      → Create new Mesh with vertices, transform (identity), texture
      → Store in World.mesh
```

### Each Frame
```
main.rs game loop:
  → Clear screen
  → Get World.mesh() if exists
  → Call Renderer.render_mesh(mesh, camera, viewport)
      → Renderer activates shader
      → Renderer binds mesh texture
      → Renderer calculates view matrix from camera
      → Renderer calculates projection matrix from viewport
      → Renderer gets model matrix from mesh
      → Renderer sets all uniforms (view, proj, model)
      → Renderer calls mesh.draw()
          → Mesh binds VAO
          → Mesh issues glDrawArrays
  → Swap buffers
```

## Implementation Considerations

### Mesh Construction
- Take vertex slice, transform matrix, and texture
- Create VAO and VBO
- Upload vertex data to GPU
- Store transform and texture for later use
- **Remember**: Unbind VAO after setup for safety, don't unbind during rendering

### World Mesh Rebuilding
- Iterate through voxel HashSet
- For each voxel position, generate cube vertices (36 vertices per cube)
- Combine into single vertex vector
- Create mesh with identity transform (world-space coordinates)
- Handle errors from mesh creation (VAO/VBO allocation can fail)

### Renderer Setup
- Load and compile shaders during initialization
- Store shader program for entire lifetime
- Each render_mesh call is stateless - sets up everything needed

### Error Handling
**Mesh creation**: Return Result, let World decide what to do on failure (keep old mesh?)
**Shader/Texture loading**: Fail fast with helpful messages - can't render without these
**Rendering**: Assume valid state - World guarantees mesh is valid if it exists

### Testing Strategy
Manual visual testing:
1. Render mesh at origin - verify it appears
2. Add/remove voxels, rebuild mesh - verify visual changes
3. Move camera - verify rendering follows correctly
4. Change texture - verify it applies

## Future Extensions

### Material System (when needed)
When you need multiple shader types (water, glass, emissive blocks):

Create Material struct:
- Bundle shader + texture + properties
- Replace `mesh.texture` with `mesh.material`
- Renderer uses `mesh.material.shader` instead of `self.shader`

**Estimated effort**: 15-minute refactor when actually needed
**Current decision**: YAGNI - build it when you need water/glass/glowing blocks

### Chunk System (when scaling up)
When 32×32 becomes 1000×1000:
- Divide world into chunks (e.g., 16×16×16 blocks)
- `World.mesh` becomes `Vec<ChunkMesh>`
- Only rebuild affected chunk when voxel changes
- Renderer loops over all chunk meshes

Current architecture makes this straightforward to add later.

## Key Learnings

**Conceptual**:
- Meshes are geometry + position, not just geometry
- Materials bundle visual properties (shader + textures)
- Separation of "what" from "how" is fundamental to clean architecture

**OpenGL**:
- VAO stores vertex attribute configuration per-mesh
- Unbind VAOs after setup for safety, skip during rendering for performance
- One VAO/VBO per mesh is standard and performant

**Architecture**:
- Build for today's needs, structure for tomorrow's changes
- Explicit control (rebuild_mesh) > automatic magic when learning
- Type system can prevent entire classes of bugs (missing transforms)
