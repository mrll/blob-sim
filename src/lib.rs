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
use ggez::{
    event, graphics, input,
    nalgebra::{Point2, Translation2},
    timer, Context, GameResult,
};
use std::sync::Arc;

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
    settings: Arc<Settings>,
}

impl Simulation {
    pub fn new(ctx: &mut Context) -> Simulation {
        Simulation {
            state: SimulationState::Stopped,
            blobs: vec![],
            food: vec![],
            generation_frames: 0,
            res: Resources::new(ctx),
            settings: Arc::new(Settings::default()),
        }
    }

    pub fn reset(&mut self, blobs: bool) {
        self.food = vec![];
        self.generation_frames = 0;
        if blobs {
            self.blobs = vec![];
            for _ in 0..self.settings.start_blobs() {
                self.blobs.push(Blob::new(self.settings.clone()))
            }
        }
        for _ in 0..self.settings.food_per_gen() {
            self.food.push(Point2::new(
                rand::random::<f32>() * self.settings.world_size().0,
                rand::random::<f32>() * self.settings.world_size().1,
            ))
        }
    }
}

// ============================================================================
// Simulation Event Handler
// ============================================================================

impl event::EventHandler for Simulation {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, self.settings.fps()) {
            match self.state {
                SimulationState::Stopped => {
                    // Do nothing
                }
                SimulationState::Running => {
                    self.generation_frames = self.generation_frames + 1;
                    if self.generation_frames > self.settings.fps() * self.settings.gen_duration() {
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
            (self.settings.screen_size().0 / TILE_SIZE).ceil() as usize,
            (self.settings.screen_size().1 / TILE_SIZE).ceil() as usize,
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

// ============================================================================
// Settings
// ============================================================================

#[derive(Clone, PartialEq)]
pub struct Settings {
    // Simulation
    sim_screen: (f32, f32),
    sim_fps: u32,
    sim_start_blobs: u32,
    sim_food_energy: f32,
    // Generation
    gen_duration: u32,
    gen_food: u32,
    // Blob
    blob_energy: f32,
    blob_speed: (f32, f32),
    blob_sense: (f32, f32),
    blob_size: (f32, f32),
}

impl Settings {
    // Simulation
    #[inline(always)]
    pub fn screen_size(&self) -> (f32, f32) {
        self.sim_screen
    }
    #[inline(always)]
    pub fn world_size(&self) -> (f32, f32) {
        (
            self.sim_screen.0 - (2.0 * TILE_SIZE),
            self.sim_screen.1 - (2.0 * TILE_SIZE),
        )
    }

    #[inline(always)]
    pub fn fps(&self) -> u32 {
        self.sim_fps
    }

    #[inline(always)]
    pub fn start_blobs(&self) -> u32 {
        self.sim_start_blobs
    }
    #[inline(always)]
    pub fn food_energy(&self) -> f32 {
        self.sim_food_energy
    }

    // Generation
    #[inline(always)]
    pub fn gen_duration(&self) -> u32 {
        self.gen_duration
    }
    #[inline(always)]
    pub fn food_per_gen(&self) -> u32 {
        self.gen_food
    }

    // Blob
    #[inline(always)]
    pub fn blob_energy(&self) -> f32 {
        self.blob_energy
    }
    #[inline(always)]
    pub fn blob_speed(&self) -> (f32, f32) {
        self.blob_speed
    }
    #[inline(always)]
    pub fn blob_sense(&self) -> (f32, f32) {
        self.blob_sense
    }
    #[inline(always)]
    pub fn blob_size(&self) -> (f32, f32) {
        self.blob_size
    }
    #[inline(always)]
    pub fn blob_step(&self) -> f32 {
        ((self.world_size().0 / 2.0) / self.gen_duration() as f32) / self.fps() as f32
    }
}

impl Default for Settings {
    fn default() -> Settings {
        let size = 960.0;
        Settings {
            // Simulation
            sim_screen: (size, size),
            sim_fps: 60,
            sim_start_blobs: 8,
            sim_food_energy: 0.0,
            // Generation
            gen_duration: 5,
            gen_food: 50,
            // Blob
            blob_energy: size / 2.0,
            blob_speed: (1.0, 0.5),
            blob_sense: (size / 7.5, 0.5),
            blob_size: (1.0, 0.5),
        }
    }
}

// ============================================================================
// Resources
// ============================================================================

pub struct Resources {
    blob: graphics::Image,
    food: graphics::Image,
    map: [graphics::Image; 10],
}

impl Resources {
    fn new(ctx: &mut Context) -> Resources {
        Resources {
            blob: graphics::Image::new(ctx, "/tiles/mapTile_136.png").unwrap(),
            food: graphics::Image::new(ctx, "/tiles/mapTile_104.png").unwrap(),
            map: [
                graphics::Image::new(ctx, "/tiles/mapTile_006.png").unwrap(),
                graphics::Image::new(ctx, "/tiles/mapTile_007.png").unwrap(),
                graphics::Image::new(ctx, "/tiles/mapTile_008.png").unwrap(),
                graphics::Image::new(ctx, "/tiles/mapTile_021.png").unwrap(),
                graphics::Image::new(ctx, "/tiles/mapTile_022.png").unwrap(),
                graphics::Image::new(ctx, "/tiles/mapTile_023.png").unwrap(),
                graphics::Image::new(ctx, "/tiles/mapTile_036.png").unwrap(),
                graphics::Image::new(ctx, "/tiles/mapTile_037.png").unwrap(),
                graphics::Image::new(ctx, "/tiles/mapTile_038.png").unwrap(),
                graphics::Image::new(ctx, "/tiles/mapTile_188.png").unwrap(),
            ],
        }
    }

    pub fn blob(&self) -> &graphics::Image {
        &self.blob
    }
    pub fn food(&self) -> &graphics::Image {
        &self.food
    }

    pub fn draw_map(&self, ctx: &mut Context, width: usize, height: usize) {
        for x in 0..width {
            for y in 0..height {
                if x == 0 {
                    // Left side
                    // Draw water
                    graphics::draw(
                        ctx,
                        &self.map[9],
                        graphics::DrawParam::default()
                            .dest(Point2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)),
                    )
                    .unwrap();
                    if y == 0 {
                        // Upper left corner
                        graphics::draw(
                            ctx,
                            &self.map[0],
                            graphics::DrawParam::default()
                                .dest(Point2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)),
                        )
                        .unwrap();
                    } else if y == height - 1 {
                        // Lower left corner
                        graphics::draw(
                            ctx,
                            &self.map[6],
                            graphics::DrawParam::default()
                                .dest(Point2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)),
                        )
                        .unwrap();
                    } else {
                        // Left border
                        graphics::draw(
                            ctx,
                            &self.map[3],
                            graphics::DrawParam::default()
                                .dest(Point2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)),
                        )
                        .unwrap();
                    }
                } else if x == width - 1 {
                    // Right side
                    // Draw water
                    graphics::draw(
                        ctx,
                        &self.map[9],
                        graphics::DrawParam::default()
                            .dest(Point2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)),
                    )
                    .unwrap();
                    if y == 0 {
                        // Upper right corner
                        graphics::draw(
                            ctx,
                            &self.map[2],
                            graphics::DrawParam::default()
                                .dest(Point2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)),
                        )
                        .unwrap();
                    } else if y == height - 1 {
                        // Lower right corner
                        graphics::draw(
                            ctx,
                            &self.map[8],
                            graphics::DrawParam::default()
                                .dest(Point2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)),
                        )
                        .unwrap();
                    } else {
                        // Right border
                        graphics::draw(
                            ctx,
                            &self.map[5],
                            graphics::DrawParam::default()
                                .dest(Point2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)),
                        )
                        .unwrap();
                    }
                } else {
                    if y == 0 {
                        // Draw water
                        graphics::draw(
                            ctx,
                            &self.map[9],
                            graphics::DrawParam::default()
                                .dest(Point2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)),
                        )
                        .unwrap();
                        // Upper border
                        graphics::draw(
                            ctx,
                            &self.map[1],
                            graphics::DrawParam::default()
                                .dest(Point2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)),
                        )
                        .unwrap();
                    } else if y == height - 1 {
                        // Draw water
                        graphics::draw(
                            ctx,
                            &self.map[9],
                            graphics::DrawParam::default()
                                .dest(Point2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)),
                        )
                        .unwrap();
                        // Lower border
                        graphics::draw(
                            ctx,
                            &self.map[7],
                            graphics::DrawParam::default()
                                .dest(Point2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)),
                        )
                        .unwrap();
                    } else {
                        // Middle
                        graphics::draw(
                            ctx,
                            &self.map[4],
                            graphics::DrawParam::default()
                                .dest(Point2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)),
                        )
                        .unwrap();
                    }
                }
            }
        }
    }
}
