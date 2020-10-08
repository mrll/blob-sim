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
            width: blob_sim::Settings::default().screen_size().0,
            height: blob_sim::Settings::default().screen_size().1,
            maximized: false,
            fullscreen_type: conf::FullscreenType::Windowed,
            borderless: false,
            min_width: blob_sim::Settings::default().screen_size().0,
            max_width: 0.0,
            min_height: blob_sim::Settings::default().screen_size().1,
            max_height: 0.0,
            resizable: true,
        });
    let (ctx, events_loop) = &mut ctx_builder.build().unwrap();
    let sim = &mut blob_sim::Simulation::new(ctx);
    sim.reset(true);
    event::run(ctx, events_loop, sim)
}
