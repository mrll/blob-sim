//! blob-sim main
//!

// No Unsafe Code
#![forbid(unsafe_code)]

// ============================================================================
// Imports
// ============================================================================

extern crate blob_sim;

use ggez::*;
use std::{env, path::PathBuf};

// ============================================================================
// Main
// ============================================================================

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        PathBuf::from("./resources")
    };
    let ctx_builder = ggez::ContextBuilder::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_AUTHORS"))
        .add_resource_path(resource_dir)
        .window_setup(conf::WindowSetup {
            title: env!("CARGO_PKG_NAME").to_owned(),
            samples: conf::NumSamples::Four,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        })
        .window_mode(conf::WindowMode {
            width: 1280.0,
            height: 720.0,
            maximized: false,
            fullscreen_type: conf::FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            max_width: 0.0,
            min_height: 0.0,
            max_height: 0.0,
            resizable: true,
        });
    let (mut ctx, mut events_loop) = ctx_builder.build().unwrap();
    let mut sim = blob_sim::Simulation::default();
    event::run(&mut ctx, &mut events_loop, &mut sim)
}
