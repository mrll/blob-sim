//! blob-sim lib
//!

// No Unsafe Code
#![forbid(unsafe_code)]

// ============================================================================
// Modules
// ============================================================================

pub mod blobs;
pub mod resources;
pub mod settings;

// ============================================================================
// Imports
// ============================================================================

use blobs::Blob;
use ggez::{
    event, graphics, input,
    nalgebra::{Point2, Translation2},
    timer, Context, GameResult,
};
use resources::Resources;
use settings::Settings;
use std::sync::{Arc, RwLock};

// ============================================================================
// Constants
// ============================================================================

pub const TILE_SIZE: f32 = 64.0;

// ============================================================================
// Simulation
// ============================================================================

pub struct Simulation {
    // State
    state: SimulationState,
    blobs: Vec<Blob>,
    food: Vec<Point2<f32>>,
    generation_frames: u32,
    // Resources
    res: Resources,
    // Settings
    settings: Arc<RwLock<Settings>>,
}

impl Simulation {
    pub fn new(ctx: &mut Context) -> Simulation {
        Simulation {
            state: SimulationState::Stopped,
            blobs: vec![],
            food: vec![],
            generation_frames: 0,
            res: Resources::new(ctx),
            settings: Arc::new(RwLock::new(Settings::default())),
        }
    }

    pub fn reset(&mut self, blobs: bool) {
        self.food = vec![];
        self.generation_frames = 0;
        if blobs {
            self.blobs = vec![];
            for _ in 0..self.settings.read().unwrap().start_blobs() {
                self.blobs.push(Blob::new(self.settings.clone()))
            }
        }
        for _ in 0..self.settings.read().unwrap().food_per_gen() {
            self.food.push(Point2::new(
                rand::random::<f32>() * self.settings.read().unwrap().world_size().0,
                rand::random::<f32>() * self.settings.read().unwrap().world_size().1,
            ))
        }
        self.settings.write().unwrap().decay_food();
        if blobs {
            self.settings.write().unwrap().reset_food();
        }
    }
}

// ============================================================================
// Simulation Event Handler
// ============================================================================

impl event::EventHandler for Simulation {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, self.settings.read().unwrap().fps()) {
            match self.state {
                SimulationState::Stopped => {
                    // Do nothing
                }
                SimulationState::Running => {
                    self.generation_frames = self.generation_frames + 1;
                    if self.generation_frames
                        > self.settings.read().unwrap().fps()
                            * self.settings.read().unwrap().gen_duration()
                    {
                        let mut new_blobs = vec![];
                        let mut dead_blobs = vec![];
                        for blob in &mut self.blobs {
                            match blob.next_gen() {
                                blobs::GenerationResult::Starve => dead_blobs.push(blob.clone()),
                                blobs::GenerationResult::Reproduce => {
                                    new_blobs.push(Blob::evolve(blob))
                                }
                                blobs::GenerationResult::Live => {
                                    // Nothing happens
                                }
                            }
                        }
                        self.blobs.retain(|b| !dead_blobs.contains(b));
                        self.blobs.append(&mut new_blobs);
                        self.reset(false);
                        self.generation_frames = 0;
                        let mut avg = (0.0, 0.0, 0.0);
                        for blob in &self.blobs {
                            avg.0 = avg.0 + blob.speed();
                            avg.1 = avg.1 + blob.sense();
                            avg.2 = avg.2 + blob.size();
                        }
                        avg.0 = avg.0 / self.blobs.len() as f32;
                        avg.1 = avg.1 / self.blobs.len() as f32;
                        avg.2 = avg.2 / self.blobs.len() as f32;
                        println!(
                            "speed: {}, sense: {}, size: {}, blobs: {}, food; {}",
                            avg.0,
                            avg.1,
                            avg.2,
                            self.blobs.len(),
                            self.food.len()
                        );
                    } else {
                        for blob in &mut self.blobs {
                            blob.update(&mut self.food);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let world_offset = Translation2::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0);
        graphics::clear(ctx, graphics::WHITE);
        // Draw World
        self.res.draw_map(
            ctx,
            (self.settings.read().unwrap().screen_size().0 / TILE_SIZE).ceil() as usize,
            (self.settings.read().unwrap().screen_size().1 / TILE_SIZE).ceil() as usize,
        );
        // Draw Food
        for food in &self.food {
            graphics::draw(
                ctx,
                self.res.food(),
                graphics::DrawParam::default().dest(world_offset.transform_point(food)),
            )?;
        }
        // Draw Blobs
        for blob in &self.blobs {
            graphics::draw(
                ctx,
                self.res.blob(),
                graphics::DrawParam::default().dest(world_offset.transform_point(&blob.position())),
            )?;
        }
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        keymods: input::keyboard::KeyMods,
        repeat: bool,
    ) {
        if input::keyboard::KeyMods::empty() == keymods && !repeat {
            match keycode {
                event::KeyCode::Escape => event::quit(ctx),
                event::KeyCode::Space => match self.state {
                    SimulationState::Running => self.state = SimulationState::Stopped,
                    SimulationState::Stopped => self.state = SimulationState::Running,
                },
                event::KeyCode::R => self.reset(true),
                _ => {}
            }
        }
    }
}

// ============================================================================
// Simulation State
// ============================================================================

pub enum SimulationState {
    Stopped,
    Running,
}
