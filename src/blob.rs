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

pub const FOOD_ENERGY: f32 = BASE_ENERGY / 2.0;
pub const BASE_ENERGY: f32 = super::TILE_SIZE * 8.0;
pub const BASE_SPEED: f32 = (super::TILE_SIZE / super::TICKS_PER_SECOND as f32) * 2.0;
pub const BASE_SIGHT: f32 = super::TILE_SIZE * 5.0;

pub const SURVIVAL_ENERGY: f32 = FOOD_ENERGY * 1.5;
pub const REPRODUCTION_ENERGY: f32 = FOOD_ENERGY * super::DEFAULT_FOOD_PER_BLOB;

// ============================================================================
// Blob
// ============================================================================

#[derive(Clone, Debug, PartialEq)]
pub struct Blob {
    energy: f32,
    speed: f32,
    sight: f32,

    pos: nalgebra::Point2<f32>,
    img: graphics::Image,

    last_dest: Option<nalgebra::Point2<f32>>,
}

impl Blob {
    pub fn new(ctx: &mut Context) -> Self {
        let img = match random::<usize>() % 5 {
            0 => graphics::Image::new(ctx, "/PNG/mapTile_136.png").unwrap(),
            1 => graphics::Image::new(ctx, "/PNG/mapTile_137.png").unwrap(),
            2 => graphics::Image::new(ctx, "/PNG/mapTile_153.png").unwrap(),
            3 => graphics::Image::new(ctx, "/PNG/mapTile_154.png").unwrap(),
            _ => graphics::Image::new(ctx, "/PNG/mapTile_170.png").unwrap(),
        };

        Blob {
            energy: BASE_ENERGY,
            speed: BASE_SPEED,
            sight: BASE_SIGHT,
            pos: nalgebra::Point2::new(
                random::<f32>() * super::WORLD_WIDTH,
                random::<f32>() * super::WORLD_HEIGTH,
            ),
            img,
            last_dest: None,
        }
    }

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
            if self.energy > self.speed {
                // Move randomly
                let dest = if let Some(last_dest) = self.last_dest {
                    last_dest
                } else {
                    nalgebra::Point2::new(
                        random::<f32>() * super::WORLD_WIDTH,
                        random::<f32>() * super::WORLD_HEIGTH,
                    )
                };
                self.last_dest = Some(dest);
                let distance = nalgebra::distance(&self.pos, &dest);
                let translation =
                    nalgebra::Translation2::from((dest - self.pos) * (self.speed / distance));
                self.pos = translation.transform_point(&self.pos);
                self.energy = self.energy - self.speed;
                if nalgebra::distance(&dest, &self.pos).abs() < self.speed {
                    self.last_dest = None;
                }
            }
            None
        }
    }

    pub fn next_gen(&mut self) -> GenerationResult {
        if self.energy < SURVIVAL_ENERGY {
            GenerationResult::Death
        } else {
            if self.energy < REPRODUCTION_ENERGY {
                self.energy = self.energy - FOOD_ENERGY;
                GenerationResult::Survival
            } else {
                self.energy = self.energy - FOOD_ENERGY;
                GenerationResult::Reproduction
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.img, (self.pos,))
    }
}

// ============================================================================
// Generation Result
// ============================================================================

pub enum GenerationResult {
    Survival,
    Reproduction,
    Death,
}
