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

pub const WORLD_WIDTH: f32 = 720.0;
pub const WORLD_HEIGTH: f32 = 720.0;

// ============================================================================
// Main
// ============================================================================

fn main() {
    let mut b1 = blob::Blob::default();
    let mut b2 = blob::Blob::default();
    println!("{:?}", b1);
    println!("{:?}", b2);
    let mut f = vec![
        nalgebra::Point2::new(
            random::<f32>() * WORLD_WIDTH,
            random::<f32>() * WORLD_HEIGTH,
        ),
        nalgebra::Point2::new(
            random::<f32>() * WORLD_WIDTH,
            random::<f32>() * WORLD_HEIGTH,
        ),
        nalgebra::Point2::new(
            random::<f32>() * WORLD_WIDTH,
            random::<f32>() * WORLD_HEIGTH,
        ),
        nalgebra::Point2::new(
            random::<f32>() * WORLD_WIDTH,
            random::<f32>() * WORLD_HEIGTH,
        ),
        nalgebra::Point2::new(
            random::<f32>() * WORLD_WIDTH,
            random::<f32>() * WORLD_HEIGTH,
        ),
        nalgebra::Point2::new(
            random::<f32>() * WORLD_WIDTH,
            random::<f32>() * WORLD_HEIGTH,
        ),
        nalgebra::Point2::new(
            random::<f32>() * WORLD_WIDTH,
            random::<f32>() * WORLD_HEIGTH,
        ),
        nalgebra::Point2::new(
            random::<f32>() * WORLD_WIDTH,
            random::<f32>() * WORLD_HEIGTH,
        ),
        nalgebra::Point2::new(
            random::<f32>() * WORLD_WIDTH,
            random::<f32>() * WORLD_HEIGTH,
        ),
        nalgebra::Point2::new(
            random::<f32>() * WORLD_WIDTH,
            random::<f32>() * WORLD_HEIGTH,
        ),
        nalgebra::Point2::new(
            random::<f32>() * WORLD_WIDTH,
            random::<f32>() * WORLD_HEIGTH,
        ),
        nalgebra::Point2::new(
            random::<f32>() * WORLD_WIDTH,
            random::<f32>() * WORLD_HEIGTH,
        ),
    ];
    println!("Starting with food at 12 locations:\n{:#?}", f);
    println!("Searching...");
    loop {
        if b1.energy() >= b1.speed() {
            let result = b1.search(f.clone());
            if let Some(result) = result {
                f.retain(|&x| x != result);
                println!("B1 found: {:?}", result);
                println!("Blob 1:   {:?}", b1);
            }
        }
        if b2.energy() >= b2.speed() {
            let result = b2.search(f.clone());
            if let Some(result) = result {
                f.retain(|&x| x != result);
                println!("B2 found: {:?}", result);
                println!("Blob 2:   {:?}", b2);
            }
        }
        if f.is_empty() {
            println!("No more food! Ate everything =)\n{:?}\n{:?}", b1, b2);
            break;
        }
        if b1.energy() < b1.speed() && b2.energy() < b2.speed() {
            println!("No energy to move! =(\n{:?}\n{:?}", b1, b2);
            println!("But there's still food\n{:?}", f);
            break;
        }
    }
}
