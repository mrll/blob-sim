//! blob-sim resources
//!

// ============================================================================
// Imports
// ============================================================================

use super::TILE_SIZE;
use ggez::{graphics, nalgebra::Point2, Context};

// ============================================================================
// Constants
// ============================================================================

// ============================================================================
// Resources
// ============================================================================

pub struct Resources {
    blob: graphics::Image,
    food: graphics::Image,
    map: [graphics::Image; 10],
}

impl Resources {
    pub fn new(ctx: &mut Context) -> Resources {
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
