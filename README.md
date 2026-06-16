# Stift

Digital painting workspace written in Rust with `egui`/`eframe` UI.

Current code is a thin editor shell around an in-memory layered document. `stift-app` opens the window, `stift-core` holds the document model and brush rasterizer, `stift-compositor` flattens the layer stack into one RGBA image, and `stift-app` turns that image into an `egui` texture.

## Architecture

The project is split into focused crates with clear responsibilities:

| Crate | Folder | Description |
|---|---|---|
| `stift-core` | `crates/core/` | **Data model + brush engine** — `Document`, `Layer`, `BlendMode`, `Brush`, `StrokePoint`, and the CPU `paint` rasterizer (`paint::rasterize`) |
| `stift-app` | `crates/app/` | **Binary** — launches the `eframe` window, builds docked tabs, drives drawing/layers/properties, and owns the egui image conversion |
| `stift-compositor` | `crates/compositor/` | **Compositor** — flattens a `Document`'s layer stack into one RGBA buffer, applying per-layer opacity and blend modes |
| `stift-renderer` | `crates/renderer/` | **GPU renderer** — UI-agnostic `wgpu`-based rendering (placeholder for now) |
| `stift-storage` | `crates/storage/` | **Stub crate** — placeholder for future file I/O and project persistence |

## Dependencies

Dependencies are deliberately scoped so each crate only carries what its own
responsibility requires. The guiding rule: **own the domain, depend on the
undifferentiated heavy lifting, and keep UI/binary concerns out of the libraries.**

| Crate | Dependencies | Rationale |
|---|---|---|
| `stift-core` | *(none)* | Pure domain model plus self-contained CPU rasterization. No UI, GPU, or I/O. |
| `stift-compositor` | `stift-core` | Flattens core's layer stack into one buffer. |
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
- `Canvas` shows the compositor's flattened buffer in an `egui::TextureHandle`; dragging the pointer rasterizes a round brush onto the active layer and re-composites.
- `Layers` lists the document's layers (top-most first) with per-layer visibility toggles and active-layer selection.
- `Properties` edits the active layer's opacity and blend mode; `Toolbar` is still a placeholder.
- The document starts `1000x1000` with an opaque white `Background` layer and a transparent `Layer 1` (active) on top.
- `stift-renderer` is currently a UI-agnostic placeholder, not yet a GPU pipeline.

## Notes

- `StrokePoint` exists in the data model but is not yet wired into stroke interpolation.
- `stift-renderer` declares `wgpu` and `guillotiere`, but the current implementation does not use them yet.
- `stift-storage` is still a placeholder crate.
