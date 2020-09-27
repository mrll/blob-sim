//! blob-sim blobs module
//!

// ============================================================================
// Modules
// ============================================================================

pub mod traits;

// ============================================================================
// Imports
// ============================================================================

use ggez::{
    graphics::Image,
    nalgebra::{distance, Point2, Translation2},
};

// ============================================================================
// The Blob
// ============================================================================

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Blob<'a> {
    // Base Stats
    energy: u16,
    speed: u8,
    sight: u8,
    strength: u8,
    // Traits
    nutrition: traits::Nutrition,
    // Map
    position: Point2<f32>,
    resource: &'a Image,
}

impl Blob<'_> {
    // Stats
    #[inline(always)]
    fn energy(&self) -> u16 {
        self.energy
    }
    #[inline(always)]
    fn speed(&self) -> u8 {
        self.speed
    }
    #[inline(always)]
    fn sight(&self) -> u8 {
        self.sight
    }
    #[inline(always)]
    fn strength(&self) -> u8 {
        self.strength
    }

    // Traits
    #[inline(always)]
    fn nutrition(&self) -> traits::Nutrition {
        self.nutrition
    }

    // Actions
    fn search_food(&mut self, food: Vec<Point2<f32>>) -> Option<Point2<f32>> {
        food.iter()
            // Filter food in sight
            .filter(|x| distance(&self.position(), &x).abs() <= self.sight() as f32)
            // Return nearest food
            .min_by(|x, y| {
                distance(&self.position(), &x)
                    .abs()
                    .partial_cmp(&distance(&self.position(), &y).abs())
                    .unwrap()
            })
            .copied()
    }
    fn search_mate(&mut self, blobs: Vec<&Blob>) -> Option<&Blob> {
        unimplemented!()
    }

    fn eat(&mut self) {
        unimplemented!()
    }
    fn mate(&mut self, blob: &Blob) {
        unimplemented!()
    }

    fn move_to(&mut self, dest: Point2<f32>) -> bool {
        if distance(&self.position(), &dest).abs() > self.speed() as f32 {
            let distance = distance(&self.position(), &dest);
            let translation =
                Translation2::from((dest - self.position()) * (self.speed() as f32 / distance));
            self.position = translation.transform_point(&self.position());
            false
        } else {
            self.position = dest;
            true
        }
    }

    // Map
    #[inline(always)]
    fn position(&self) -> Point2<f32> {
        self.position
    }
    #[inline(always)]
    fn resource(&self) -> &Image {
        self.resource
    }
}
