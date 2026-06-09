# Stift

## Crates

| Crate | Folder | Description |
|---|---|---|
| `stift-core` | `crates/core/` | Shared primitive types — `Color`, `Rect`, `Point`, `Size`, `Layer`, `BlendMode` |
| `stift-app` | `crates/app/` | Binary entry point — window management, UI layer, event handling, wires everything together |
| `stift-compositor` | `crates/compositor/` | Core image processing — layer stack, blend modes, masks, filters, adjustment layers |
| `stift-renderer` | `crates/renderer/` | GPU rendering pipeline — draws the canvas to screen via `wgpu` |
| `stift-storage` | `crates/storage/` | File I/O — reading and writing image formats, project serialization, undo/redo history |
