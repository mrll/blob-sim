//! blob-sim blob module
//!

// ============================================================================
// Imports
// ============================================================================

use ggez::*;
use rand::random;

// ============================================================================
// Constants
// ============================================================================

pub const FOOD_ENERGY: f32 = (super::WORLD_HEIGTH + super::WORLD_WIDTH) / 4.0;
pub const BASE_ENERGY: f32 = (super::WORLD_HEIGTH + super::WORLD_WIDTH) / 8.0;
pub const BASE_SPEED: f32 = (super::WORLD_HEIGTH + super::WORLD_WIDTH) / 10.0;
pub const BASE_SIGHT: f32 = (super::WORLD_HEIGTH + super::WORLD_WIDTH) / 3.0;

// ============================================================================
// Blob
// ============================================================================

#[derive(Copy, Clone, Debug)]
pub struct Blob {
    energy: f32,
    speed: f32,
    sight: f32,

    pos: nalgebra::Point2<f32>,
}

impl Blob {
    pub fn energy(&self) -> f32 {
        self.energy
    }
    pub fn speed(&self) -> f32 {
        self.speed
    }
    pub fn sight(&self) -> f32 {
        self.sight
    }

    pub fn position(&self) -> nalgebra::Point2<f32> {
        self.pos
    }

    pub fn search(&mut self, food: Vec<nalgebra::Point2<f32>>) -> Option<nalgebra::Point2<f32>> {
        let nearest_food = food
            .iter()
            // Filter food in sight
            .filter(|x| {
                let distance = nalgebra::distance(&self.pos, &x).abs();
                distance <= self.sight
            })
            // Return nearest food
            .min_by(|x, y| {
                nalgebra::distance(&self.pos, &x)
                    .abs()
                    .partial_cmp(&nalgebra::distance(&self.pos, &y).abs())
                    .unwrap()
            });
        if let Some(nearest_food) = nearest_food {
            let distance = nalgebra::distance(&self.pos, nearest_food);
            if distance.abs() <= self.speed {
                self.pos = *nearest_food;
                self.energy = self.energy - distance + FOOD_ENERGY;
                Some(*nearest_food)
            } else {
                let translation = nalgebra::Translation2::from(
                    (nearest_food - self.pos) * (self.speed / distance),
                );
                self.pos = translation.transform_point(&self.pos);
                self.energy = self.energy - self.speed;
                None
            }
        } else {
            //if self.energy < 250.0 {
            // Move randomly
            let dest = nalgebra::Point2::new(
                random::<f32>() * super::WORLD_WIDTH,
                random::<f32>() * super::WORLD_HEIGTH,
            );
            let distance = nalgebra::distance(&self.pos, &dest);
            let translation =
                nalgebra::Translation2::from((dest - self.pos) * (self.speed / distance));
            self.pos = translation.transform_point(&self.pos);
            self.energy = self.energy - self.speed;
            //}
            None
        }
    }
}

impl Default for Blob {
    fn default() -> Self {
        Blob {
            energy: BASE_ENERGY,
            speed: BASE_SPEED,
            sight: BASE_SIGHT,
            pos: nalgebra::Point2::new(
                random::<f32>() * super::WORLD_WIDTH,
                random::<f32>() * super::WORLD_HEIGTH,
            ),
        }
    }
}
