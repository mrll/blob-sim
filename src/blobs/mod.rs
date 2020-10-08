//! blob-sim blobs module
//!

// ============================================================================
// Modules
// ============================================================================

// ============================================================================
// Imports
// ============================================================================

use ggez::nalgebra::{distance, Point2, Translation2};
use rand::random;
use std::sync::Arc;

// ============================================================================
// Constants
// ============================================================================

// ============================================================================
// The Blob
// ============================================================================

#[derive(Clone, PartialEq)]
pub struct Blob {
    // Base Stats
    energy: f32,
    speed: f32,
    sense: f32,
    size: f32,
    // State
    state: BlobState,
    // Generation Stats
    food_found: u8,
    // Map
    destination: Point2<f32>,
    position: Point2<f32>,
    // Settings
    settings: Arc<super::Settings>,
}

impl Blob {
    pub fn new(settings: Arc<super::Settings>) -> Blob {
        Blob {
            // Base Stats
            energy: settings.blob_energy(),
            speed: settings.blob_speed().0,
            sense: settings.blob_sense().0,
            size: settings.blob_size().0,
            // State
            state: BlobState::SearchFood,
            // Generation Stats
            food_found: 0,
            // Map
            destination: Point2::new(
                random::<f32>() * settings.world_size().0,
                random::<f32>() * settings.world_size().1,
            ),
            position: Point2::new(
                random::<f32>() * settings.world_size().0,
                random::<f32>() * settings.world_size().1,
            ),
            // Settings
            settings,
        }
    }

    pub fn evolve(from_blob: &Blob) -> Blob {
        let size = from_blob.size()
            * (1.0 + (random::<f32>() * from_blob.settings.blob_size().1)
                - from_blob.settings.blob_size().1 / 2.0);
        Blob {
            // Base Stats
            energy: from_blob.settings.blob_energy() * size,
            speed: from_blob.speed()
                * (1.0 + (random::<f32>() * from_blob.settings.blob_speed().1)
                    - from_blob.settings.blob_speed().1 / 2.0),
            sense: from_blob.sense()
                * (1.0 + (random::<f32>() * from_blob.settings.blob_sense().1)
                    - from_blob.settings.blob_sense().1 / 2.0),
            size,
            // State
            state: BlobState::SearchFood,
            // Generation Stats
            food_found: 0,
            // Map
            destination: Point2::new(
                random::<f32>() * from_blob.settings.world_size().0,
                random::<f32>() * from_blob.settings.world_size().1,
            ),
            position: Point2::new(
                random::<f32>() * from_blob.settings.world_size().0,
                random::<f32>() * from_blob.settings.world_size().1,
            ),
            // Settings
            settings: from_blob.settings.clone(),
        }
    }
}

impl Blob {
    // Stats
    #[inline(always)]
    pub fn energy(&self) -> f32 {
        self.energy
    }
    #[inline(always)]
    pub fn speed(&self) -> f32 {
        self.speed
    }
    #[inline(always)]
    pub fn sense(&self) -> f32 {
        self.sense
    }
    #[inline(always)]
    pub fn size(&self) -> f32 {
        self.size
    }

    // State
    #[inline(always)]
    pub fn state(&self) -> BlobState {
        self.state
    }

    // Actions
    pub fn update(&mut self, food: &mut Vec<Point2<f32>>) {
        match self.state() {
            BlobState::SearchFood => {
                if let Some(next_food) = self.search_food(food) {
                    // Set food as destination and go to
                    self.destination = next_food;
                    self.state = BlobState::GoToFood;
                } else if self.position() == self.destination() {
                    // Generate random destination
                    self.destination = Point2::new(
                        random::<f32>() * self.settings.world_size().0,
                        random::<f32>() * self.settings.world_size().1,
                    );
                }
                self.move_to();
            }
            BlobState::GoToFood => {
                if food.contains(&self.destination()) {
                    if self.move_to() {
                        // Eat and remove food from list
                        self.eat();
                        food.retain(|&f| f != self.destination());
                        // Search for more
                        self.state = BlobState::SearchFood;
                    }
                } else {
                    // Sombody else ate it, go back searching
                    self.state = BlobState::SearchFood;
                    self.update(food);
                }
            }
            BlobState::GoHome => {
                // Just move until at home
                if self.move_to() {
                    self.state = BlobState::AtHome;
                }
            }
            BlobState::AtHome => {
                // Do nothing, wait for next gen
            }
        }
        // Go home on low energy and when enough food found
        match self.state() {
            BlobState::SearchFood | BlobState::GoToFood => {
                if (self.energy() < self.settings.food_energy() * 2.0 && self.food_found == 1)
                    || self.food_found >= 2
                {
                    // Get distance to next edge on x axis
                    let distance_x = if self.position()[0] > (self.settings.world_size().0 / 2.0) {
                        self.position()[0] - self.settings.world_size().0
                    } else {
                        self.position()[0]
                    };
                    // Get distance to nect edge on y axis
                    let distance_y = if self.position()[1] > (self.settings.world_size().1 / 2.0) {
                        self.position()[1] - self.settings.world_size().1
                    } else {
                        self.position()[1]
                    };
                    // Set destination to next edge
                    if distance_x.abs() < distance_y.abs() {
                        self.destination =
                            Point2::new(self.position()[0] - distance_x, self.position()[1]);
                    } else {
                        self.destination =
                            Point2::new(self.position()[0], self.position()[1] - distance_y);
                    }
                    // Go home
                    self.state = BlobState::GoHome;
                }
            }
            BlobState::GoHome | BlobState::AtHome => {
                // Already (or on the way) home
            }
        }
    }

    fn search_food(&mut self, food: &mut Vec<Point2<f32>>) -> Option<Point2<f32>> {
        food.iter()
            // Filter food for stuff in sense range
            .filter(|x| distance(&self.position(), &x).abs() <= self.sense())
            // Return nearest food
            .min_by(|x, y| {
                distance(&self.position(), &x)
                    .abs()
                    .partial_cmp(&distance(&self.position(), &y).abs())
                    .unwrap()
            })
            .copied()
    }

    fn eat(&mut self) {
        // Get energy from food
        self.energy = self.energy() + self.settings.food_energy();
        // Add collected food
        self.food_found = self.food_found + 1;
    }

    fn move_to(&mut self) -> bool {
        // Max move pixel times speed
        let max_distance = self.settings.blob_step() * self.speed();
        // Distance to target
        let distance_to_target = distance(&self.position(), &self.destination());
        // Move either to target if smaller than max possible distance or max_distance
        let distance_to_move = distance_to_target.abs().min(max_distance);
        // Calculate energy needed for move
        let needed_energy = distance_to_move * self.speed().powi(2) * self.size().powi(3);
        // Update position if enough energy and return if target reached
        if self.energy() >= needed_energy {
            self.energy = self.energy() - needed_energy;
            if distance_to_target == distance_to_move {
                // New position equals target
                self.position = self.destination();
                true
            } else {
                // New position between target and current position
                let translation = Translation2::from(
                    (self.destination() - self.position()) * (max_distance / distance_to_target),
                );
                self.position = translation.transform_point(&self.position());
                false
            }
        } else {
            // Not enough energy
            false
        }
    }

    pub fn next_gen(&mut self) -> GenerationResult {
        // Check round outcome...
        let result = if self.state() == BlobState::AtHome {
            match self.food_found {
                0 => GenerationResult::Starve,
                1 => GenerationResult::Live,
                2 => GenerationResult::Reproduce,
                _ => unreachable!(),
            }
        } else {
            GenerationResult::Starve
        };
        // ...reset self...
        self.energy = self.settings.blob_energy() * self.size();
        self.state = BlobState::SearchFood;
        self.food_found = 0;
        self.destination = Point2::new(
            random::<f32>() * self.settings.world_size().0,
            random::<f32>() * self.settings.world_size().1,
        );
        self.position = Point2::new(
            random::<f32>() * self.settings.world_size().0,
            random::<f32>() * self.settings.world_size().1,
        );
        // ...and return outcome
        result
    }

    // Map
    #[inline(always)]
    pub fn destination(&self) -> Point2<f32> {
        self.destination
    }
    #[inline(always)]
    pub fn position(&self) -> Point2<f32> {
        self.position
    }
}

// ============================================================================
// Blob State
// ============================================================================

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BlobState {
    SearchFood,
    GoToFood,
    GoHome,
    AtHome,
}

// ============================================================================
// Generation Result
// ============================================================================

pub enum GenerationResult {
    Starve,
    Live,
    Reproduce,
}

// ============================================================================
// Testing
// ============================================================================

// TODO
