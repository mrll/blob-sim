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

use ggez::*;

// ============================================================================
// Simulation
// ============================================================================

pub struct Simulation {
    state: SimulationState,
}

impl Simulation {}

impl Default for Simulation {
    fn default() -> Simulation {
        Simulation {
            state: SimulationState::Stopped,
        }
    }
}

// ============================================================================
// Simulation Event Handler
// ============================================================================

impl event::EventHandler for Simulation {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, 60) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
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
