//! blob-sim
//!

// No Unsafe Code
#![forbid(unsafe_code)]

// ============================================================================
// Modules
// ============================================================================

mod blob;

// ============================================================================
// Imports
// ============================================================================

use ggez::*;
use rand::random;

// ============================================================================
// Constants
// ============================================================================

pub const WORLD_WIDTH: f32 = 1280.0;
pub const WORLD_HEIGTH: f32 = 640.0;

pub const TILE_SIZE: f32 = 64.0;

pub const DEFAULT_START_BLOBS: u32 = 4;
pub const DEFAULT_FOOD_PER_BLOB: f32 = 2.5;
pub const MAX_FOOD_ON_MAP: u32 =
    (((WORLD_WIDTH / TILE_SIZE) * (WORLD_HEIGTH / TILE_SIZE)) / 3.0) as u32;

pub const TICKS_PER_GENERATION: u32 = 5 * TICKS_PER_SECOND;
pub const TICKS_PER_SECOND: u32 = 30;

// ============================================================================
// Main
// ============================================================================

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_AUTHORS"))
        .add_resource_path(resource_dir)
        .window_setup(conf::WindowSetup {
            title: env!("CARGO_PKG_NAME").to_owned(),
            samples: conf::NumSamples::Zero,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        })
        .window_mode(conf::WindowMode {
            width: WORLD_WIDTH,
            height: WORLD_HEIGTH,
            maximized: false,
            fullscreen_type: conf::FullscreenType::Windowed,
            borderless: false,
            min_width: WORLD_WIDTH,
            max_width: 0.0,
            min_height: WORLD_HEIGTH,
            max_height: 0.0,
            resizable: true,
        });

    let (ctx, events_loop) = &mut cb.build()?;

    let state = &mut GameState::new(DEFAULT_FOOD_PER_BLOB, DEFAULT_START_BLOBS, ctx);
    event::run(ctx, events_loop, state)
}

// ============================================================================
// Game State
// ============================================================================

struct GameState {
    food: Vec<nalgebra::Point2<f32>>,
    blobs: Vec<blob::Blob>,

    food_per_blob: f32,

    round_tick: u32,
    generation: u32,

    img_back: graphics::Image,
    img_food: graphics::Image,
}

impl GameState {
    pub fn new(food_per_blob: f32, start_blob_count: u32, ctx: &mut Context) -> GameState {
        let mut blobs = Vec::new();
        for _ in 0..start_blob_count {
            blobs.push(blob::Blob::new(ctx));
        }
        let mut food = Vec::new();
        for _ in 0..std::cmp::min(
            (food_per_blob * start_blob_count as f32) as usize,
            MAX_FOOD_ON_MAP as usize,
        ) {
            food.push(nalgebra::Point2::new(
                random::<f32>() * WORLD_WIDTH,
                random::<f32>() * WORLD_HEIGTH,
            ));
        }
        GameState {
            food,
            blobs,
            food_per_blob,
            round_tick: 0,
            generation: 0,
            img_back: graphics::Image::new(ctx, "/PNG/mapTile_022.png").unwrap(),
            img_food: graphics::Image::new(ctx, "/PNG/mapTile_104.png").unwrap(),
        }
    }

    pub fn next_gen(&mut self, ctx: &mut Context) {
        self.round_tick = 0;
        self.generation = self.generation + 1;
        // Advance blobs to next generation
        let mut dead_blobs: Vec<&blob::Blob> = vec![];
        let mut new_blobs = 0;
        let mut blobs_clone = self.blobs.clone();
        for b in &mut self.blobs {
            match b.next_gen() {
                blob::GenerationResult::Death => dead_blobs.push(b),
                blob::GenerationResult::Reproduction => new_blobs = new_blobs + 1,
                _ => {}
            }
        }
        // Remove dead blobs
        blobs_clone.retain(|b| dead_blobs.contains(&b) == false);
        self.blobs = blobs_clone;
        // Add new born blobs
        for _ in 0..new_blobs {
            self.blobs.push(blob::Blob::new(ctx));
        }
        // Add food to map
        for _ in 0..std::cmp::min(
            (self.food_per_blob * self.blobs.len() as f32) as usize,
            MAX_FOOD_ON_MAP as usize,
        ) {
            self.food.push(nalgebra::Point2::new(
                random::<f32>() * WORLD_WIDTH,
                random::<f32>() * WORLD_HEIGTH,
            ));
        }
        println!(
            "Gen: {}, Blobs: {}, Food: {}",
            self.generation,
            self.blobs.len(),
            self.food.len()
        );
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if timer::check_update_time(ctx, TICKS_PER_SECOND as u32) {
            for b in &mut self.blobs {
                let ate_food = b.search(self.food.clone());
                if let Some(ate_food) = ate_food {
                    self.food.retain(|food| *food != ate_food);
                }
            }
            self.round_tick += 1;
            if self.round_tick >= TICKS_PER_GENERATION || self.food.is_empty() {
                self.next_gen(ctx);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        // Draw map
        for x in 0..(WORLD_WIDTH / TILE_SIZE) as usize {
            for y in 0..(WORLD_HEIGTH / TILE_SIZE) as usize {
                graphics::draw(
                    ctx,
                    &self.img_back,
                    graphics::DrawParam::default().dest(nalgebra::Point2::new(
                        x as f32 * TILE_SIZE,
                        y as f32 * TILE_SIZE,
                    )),
                )?;
            }
        }
        // Draw food
        for f in &self.food {
            graphics::draw(ctx, &self.img_food, graphics::DrawParam::default().dest(*f))?;
        }
        // Draw blobs
        for b in &self.blobs {
            b.draw(ctx)?;
        }
        graphics::present(ctx)
    }
}
