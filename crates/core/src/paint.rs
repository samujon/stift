//! CPU brush rasterization.
//!
//! The brush engine turns a [`Brush`] plus a position into pixel writes on a
//! single [`Layer`]. It deliberately knows nothing about layer stacks or
//! compositing — that is the compositor's job. This module is a natural
//! candidate to grow into its own `stift-brush` crate later.

use crate::{Brush, Layer};

/// Rasterizes `brush` centered at pixel `(x, y)` onto `layer`.
pub fn rasterize(layer: &mut Layer, x: usize, y: usize, brush: Brush) {
    if x >= layer.width as usize || y >= layer.height as usize {
        return;
    }
    match brush {
        Brush::Round { size, color } => round(layer, x, y, size as usize, color),
        Brush::Square { size, color } => square(layer, x, y, size as usize, color),
        Brush::Diamond { size, color } => diamond(layer, x, y, size as usize, color),
    }
}

fn put_pixel(layer: &mut Layer, j: usize, i: usize, color: [u8; 4]) {
    let index = (i * layer.width as usize + j) * 4;
    if index + 3 < layer.buffer.len() {
        layer.buffer[index..index + 4].copy_from_slice(&color);
    }
}

fn round(layer: &mut Layer, x: usize, y: usize, size: usize, color: [u8; 4]) {
    let radius = size / 2;
    for i in y.saturating_sub(radius)..=y + radius {
        for j in x.saturating_sub(radius)..=x + radius {
            if i < layer.height as usize && j < layer.width as usize {
                let dx = j as isize - x as isize;
                let dy = i as isize - y as isize;
                if dx * dx + dy * dy <= (radius as isize * radius as isize) {
                    put_pixel(layer, j, i, color);
                }
            }
        }
    }
}

fn square(layer: &mut Layer, x: usize, y: usize, size: usize, color: [u8; 4]) {
    let radius = size / 2;
    for i in y.saturating_sub(radius)..=y + radius {
        for j in x.saturating_sub(radius)..=x + radius {
            if i < layer.height as usize && j < layer.width as usize {
                put_pixel(layer, j, i, color);
            }
        }
    }
}

fn diamond(layer: &mut Layer, x: usize, y: usize, size: usize, color: [u8; 4]) {
    let radius = size / 2;
    for i in y.saturating_sub(radius)..=y + radius {
        for j in x.saturating_sub(radius)..=x + radius {
            if i < layer.height as usize && j < layer.width as usize {
                let dx = (j as isize - x as isize).abs();
                let dy = (i as isize - y as isize).abs();
                if dx + dy <= radius as isize {
                    put_pixel(layer, j, i, color);
                }
            }
        }
    }
}
