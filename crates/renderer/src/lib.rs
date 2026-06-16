//! UI-agnostic GPU rendering crate.
//!
//! This crate is intentionally free of any UI-framework (e.g. egui) types so it
//! can be reused headlessly. GPU rendering built on `wgpu` lives here.
//! Main power use would be for this crate to do GPU-accelerated compositing of brush strokes, layers, and effects.
