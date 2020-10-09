//! blob-sim settings
//!

// ============================================================================
// Imports
// ============================================================================

use super::TILE_SIZE;

// ============================================================================
// Constants
// ============================================================================

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
    gen_food_decay: (u32, u32, u32),
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
    #[inline(always)]
    pub fn decay_food(&mut self) {
        if self.gen_food > self.gen_food_decay.2 {
            self.gen_food = self.gen_food - self.gen_food_decay.1;
        }
    }
    #[inline(always)]
    pub fn reset_food(&mut self) {
        self.gen_food = self.gen_food_decay.0;
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
            gen_food: 100,
            gen_food_decay: (100, 1, 25),
            // Blob
            blob_energy: size / 2.0,
            blob_speed: (1.0, 0.5),
            blob_sense: (size / 7.5, 0.5),
            blob_size: (1.0, 0.5),
        }
    }
}
