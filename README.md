# Stift

Digital painting workspace written in Rust with `egui`/`eframe` UI.

Current code is a thin editor shell around an in-memory RGBA canvas. `stift-app` opens the window, `stift-compositor` owns the pixel buffer, and `stift-renderer` turns that buffer into an `egui` image.

## Architecture

The project is split into focused crates with clear responsibilities:

| Crate | Folder | Description |
|---|---|---|
| `stift-core` | `crates/core/` | Shared paint primitives — `StrokePoint` and `Brush::Round { size }` |
| `stift-app` | `crates/app/` | **Binary** — launches the `eframe` window, builds docked tabs, shows the canvas texture, and owns the egui image conversion |
| `stift-compositor` | `crates/compositor/` | **Canvas backend** — in-memory RGBA buffer, size accessors, and redraw tracking |
| `stift-renderer` | `crates/renderer/` | **GPU renderer** — UI-agnostic `wgpu`-based rendering (placeholder for now) |
| `stift-storage` | `crates/storage/` | **Stub crate** — placeholder for future file I/O and project persistence |

## Dependencies

Dependencies are deliberately scoped so each crate only carries what its own
responsibility requires. The guiding rule: **own the domain, depend on the
undifferentiated heavy lifting, and keep UI/binary concerns out of the libraries.**

| Crate | Dependencies | Rationale |
|---|---|---|
| `stift-core` | *(none)* | Pure domain types. No behavior, no UI, no I/O — nothing to depend on. |
| `stift-compositor` | `stift-core` | Owns pixel buffers built from core primitives only. |
| `stift-renderer` | `stift-core`, `wgpu`, `bytemuck`, `guillotiere` | GPU rendering and texture packing. Intentionally free of any UI framework so it stays reusable/headless. |
| `stift-storage` | `stift-core`, `image` | File-format crate; image codecs belong here. |
| `stift-app` | `stift-*`, `eframe`, `egui`, `egui_dock`, `arboard`, `env_logger`, `log`, `pollster` | Composition root and UI edge. All egui coupling and app-level concerns (logging, future-blocking) live here. |

Notes on the boundaries:

- **`egui` lives only in `stift-app`.** The RGBA → `egui::ColorImage` conversion sits
  in `crates/app/src/render.rs`, not in `stift-renderer`, so the renderer never
  references a UI type.
- **`env_logger` / `log` / `pollster` live only in `stift-app`.** Logging init and
  future-blocking are binary concerns, not library concerns.
- **`wgpu`, `guillotiere`, and `image` are present but not yet wired up.** They mark
  where future GPU rendering and file I/O will land, and sit in the crate that will
  actually use them.

## Current Runtime

- `stift-app` calls `app::run()`, sets `RUST_LOG=info`, and opens a `1280x720` window titled `Stift Editor`.
- The main UI uses `egui_dock` with four tabs: `Canvas`, `Layers`, `Properties`, and `Toolbar`.
- `Canvas` loads the compositor buffer into an `egui::TextureHandle` and shows it inside a scroll area.
- `Layers`, `Properties`, and `Toolbar` are UI placeholders for now; they do not yet drive painting logic.
- `stift-compositor` starts as a solid white `1000x1000` RGBA buffer and only tracks whether a redraw is needed.
- `stift-renderer` is currently a UI-agnostic placeholder, not yet a GPU pipeline.

## Notes

- `stift-core` provides the basic stroke data model, but it is not yet wired into canvas editing.
- `stift-renderer` declares `wgpu` and `guillotiere`, but the current implementation does not use them yet.
- `stift-storage` is still a placeholder crate.
