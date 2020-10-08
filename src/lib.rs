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

// ============================================================================
// Constants
// ============================================================================

pub const WORLD_WIDTH: f32 = SCREEN_WIDTH - (2.0 * TILE_SIZE);
pub const WORLD_HEIGHT: f32 = SCREEN_HEIGHT - (2.0 * TILE_SIZE);

pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;

pub const FRAMES_PER_SECOND: f32 = 60.0;
pub const SECONDS_PER_GENERATION: f32 = 10.0;
pub const FRAMES_PER_GENERATION: f32 = FRAMES_PER_SECOND * SECONDS_PER_GENERATION;
pub const PX_MOVEMENT_PER_SECOND: f32 = (WORLD_WIDTH / 2.0) / SECONDS_PER_GENERATION;
pub const PX_MOVEMENT_PER_FRAME: f32 = PX_MOVEMENT_PER_SECOND / FRAMES_PER_SECOND;

pub const START_BLOB_COUNT: u8 = 8;
pub const FOOD_PER_TURN: u8 = 100;

pub const TILE_SIZE: f32 = 64.0;

// ============================================================================
// Simulation
// ============================================================================

pub struct Simulation {
    // Game state
    state: SimulationState,
    blobs: Vec<Blob>,
    food: Vec<Point2<f32>>,
    generation_frames: u32,
    // Game resources
    res: Resources,
}

impl Simulation {
    pub fn new(ctx: &mut Context) -> Simulation {
        Simulation {
            state: SimulationState::Stopped,
            blobs: vec![],
            food: vec![],
            generation_frames: 0,
            res: Resources::new(ctx),
        }
    }

    pub fn reset(&mut self, blobs: bool) {
        self.food = vec![];
        self.generation_frames = 0;
        if blobs {
            self.blobs = vec![];
            for _ in 0..START_BLOB_COUNT {
                self.blobs.push(Blob::new())
            }
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
                    self.generation_frames = self.generation_frames + 1;
                    if self.generation_frames > FRAMES_PER_GENERATION as u32 {
                        let mut new_blobs = vec![];
                        let mut dead_blobs = vec![];
                        for blob in &mut self.blobs {
                            match blob.next_gen() {
                                blobs::GenerationResult::Starve => dead_blobs.push(blob.clone()),
                                blobs::GenerationResult::Reproduce => new_blobs.push(Blob::new()),
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
            (SCREEN_WIDTH / TILE_SIZE).ceil() as usize,
            (SCREEN_HEIGHT / TILE_SIZE).ceil() as usize,
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

pub struct Settings {}

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
