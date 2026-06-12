# Stift

Digital painting application written in Rust with `egui` UI and `wgpu` GPU rendering.

## Architecture

The project is split into focused crates with clear responsibilities:

| Crate | Folder | Description |
|---|---|---|
| `stift-core` | `crates/core/` | Shared types — `StrokePoint`, `Brush` enums; foundation for brush stroke data |
| `stift-app` | `crates/app/` | **Binary** — `egui`/`eframe` window, toolbar & layer panels, event routing, integrates all crates |
| `stift-compositor` | `crates/compositor/` | **Canvas backend** — in-memory RGBA buffer (u32×u32), redraw tracking, placeholder test shape |
| `stift-renderer` | `crates/renderer/` | **Image bridge** — RGBA→egui `ColorImage` conversion; `wgpu` GPU pipeline & guillotiere texture atlas |
| `stift-storage` | `crates/storage/` | **File I/O** (stub) — reserved for image format I/O, project serialization, undo/redo |
