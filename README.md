# Stift

Digital painting workspace written in Rust with `egui`/`eframe` UI.

Current code is a thin editor shell around an in-memory RGBA canvas. `stift-app` opens the window, `stift-compositor` owns the pixel buffer, and `stift-renderer` turns that buffer into an `egui` image.

## Architecture

The project is split into focused crates with clear responsibilities:

| Crate | Folder | Description |
|---|---|---|
| `stift-core` | `crates/core/` | Shared paint primitives — `StrokePoint` and `Brush::Round { size }` |
| `stift-app` | `crates/app/` | **Binary** — launches the `eframe` window, builds docked tabs, and shows the canvas texture |
| `stift-compositor` | `crates/compositor/` | **Canvas backend** — in-memory RGBA buffer, size accessors, and redraw tracking |
| `stift-renderer` | `crates/renderer/` | **Image bridge** — converts RGBA byte slices into `egui::ColorImage` |
| `stift-storage` | `crates/storage/` | **Stub crate** — placeholder for future file I/O and project persistence |

## Current Runtime

- `stift-app` calls `app::run()`, sets `RUST_LOG=info`, and opens a `1280x720` window titled `Stift Editor`.
- The main UI uses `egui_dock` with four tabs: `Canvas`, `Layers`, `Properties`, and `Toolbar`.
- `Canvas` loads the compositor buffer into an `egui::TextureHandle` and shows it inside a scroll area.
- `Layers`, `Properties`, and `Toolbar` are UI placeholders for now; they do not yet drive painting logic.
- `stift-compositor` starts as a solid white `1000x1000` RGBA buffer and only tracks whether a redraw is needed.
- `stift-renderer` is currently a small conversion helper, not a GPU pipeline.

## Notes

- `stift-core` provides the basic stroke data model, but it is not yet wired into canvas editing.
- Workspace dependencies already include `wgpu` and `guillotiere`, but the current implementation does not use them yet.
- `stift-storage` is still a placeholder crate.
