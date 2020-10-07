//! blob-sim lib
//!

// No Unsafe Code
#![forbid(unsafe_code)]

// ============================================================================
// Modules
// ============================================================================

pub mod blobs;

// ============================================================================
// Imports
// ============================================================================

use blobs::Blob;
use ggez::{event, graphics, nalgebra::Point2, timer, Context, GameResult};

// ============================================================================
// Constants
// ============================================================================

pub const WORLD_WIDTH: f32 = 1280.0;
pub const WORLD_HEIGHT: f32 = 720.0;

pub const FRAMES_PER_SECOND: f32 = 60.0;
pub const SECONDS_PER_GENERATION: f32 = 5.0;
pub const FRAMES_PER_GENERATION: f32 = FRAMES_PER_SECOND * SECONDS_PER_GENERATION;
pub const PX_MOVEMENT_PER_SECOND: f32 = 64.0;
pub const PX_MOVEMENT_PER_FRAME: f32 = PX_MOVEMENT_PER_SECOND / FRAMES_PER_SECOND;

pub const START_BLOB_COUNT: u8 = 8;
pub const FOOD_PER_TURN: u8 = 100;

// ============================================================================
// Simulation
// ============================================================================

pub struct Simulation {
    // Game state
    state: SimulationState,
    blobs: Vec<Blob>,
    food: Vec<Point2<f32>>,
    // Game resources
    blob_img: graphics::Image,
    food_img: graphics::Image,
}

impl Simulation {
    pub fn new(ctx: &mut Context) -> Simulation {
        Simulation {
            state: SimulationState::Stopped,
            blobs: vec![],
            food: vec![],
            blob_img: graphics::Image::new(ctx, "/PNG/mapTile_136.png").unwrap(),
            food_img: graphics::Image::new(ctx, "/PNG/mapTile_104.png").unwrap(),
        }
    }

    pub fn reset(&mut self) {
        self.blobs = vec![];
        self.food = vec![];
        for _ in 0..START_BLOB_COUNT {
            self.blobs.push(Blob::new())
        }
        for _ in 0..FOOD_PER_TURN {
            self.food.push(Point2::new(
                rand::random::<f32>() * WORLD_WIDTH,
                rand::random::<f32>() * WORLD_HEIGHT,
            ))
        }
    }
}

// ============================================================================
// Simulation Event Handler
// ============================================================================

impl event::EventHandler for Simulation {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, FRAMES_PER_SECOND as u32) {
            match self.state {
                SimulationState::Stopped => {
                    // Do nothing
                }
                SimulationState::Running => {
                    for blob in &mut self.blobs {
                        blob.update(&mut self.food);
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        // Draw World
        // Draw Food
        for food in &self.food {
            graphics::draw(
                ctx,
                &self.food_img,
                graphics::DrawParam::default().dest(*food),
            )?;
        }
        // Draw Blobs
        for blob in &self.blobs {
            graphics::draw(
                ctx,
                &self.blob_img,
                graphics::DrawParam::default().dest(blob.position()),
            )?;
        }
        graphics::present(ctx)
    }
}

// ============================================================================
// Simulation State
// ============================================================================

pub enum SimulationState {
    Stopped,
    Running,
}
