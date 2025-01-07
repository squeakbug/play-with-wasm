use glam::USizeVec2;
use wasm_bindgen::prelude::*;

use crate::{cell::{PhysicalCellBuilder}, physicslife::World};

#[wasm_bindgen]
#[derive(Clone)]
pub enum Shape {
    Square,
    Circle,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Brush {
    tool: PhysicalCellBuilder,
    shape: Shape,
    size: usize,
    left_top_coord: USizeVec2,
}

impl Brush {
    pub fn new() -> Self {
        Brush {
            tool: PhysicalCellBuilder::default(),
            shape: Shape::Circle,
            size: 1,
            left_top_coord: USizeVec2::new(0, 0),
        }
    }

    pub fn apply(&self, world: &mut World) {
        match self.shape {
            Shape::Square => self.apply_square(world),
            Shape::Circle => self.apply_circle(world),
        }
    }

    pub fn set_position(&mut self, x: usize, y: usize) {
        self.left_top_coord = USizeVec2::new(x, y);
    }
}

impl Brush {
    fn set_pixel(&self, world: &mut World, x: i32, y: i32) {
        let shadow_cell = world.get_shadow_cell_mut(y as usize, x as usize);
        *shadow_cell = self.tool.build();
    }

    fn apply_square(&self, world: &mut World) {
        let bs2 = self.size / 2;
        let USizeVec2 { x, y } = self.left_top_coord;
        for x in (x - bs2)..(x + bs2 + 1) {
            for y in (y - bs2)..(y + bs2 + 1) {
                self.set_pixel(world, x as i32, y as i32);
            }
        }
    }

    fn apply_circle(&self, world: &mut World) {
        let radius = (self.size / 2) as i32;
        let USizeVec2 { x, y } = self.left_top_coord;
        let (x0, y0) = (x as i32 + radius, y as i32 + radius);
        let mut x = 0i32;
        let mut y = radius as i32;
        let mut delta = (1 - 2 * radius) as i32;
        let mut error = 0i32;
        while y >= 0 {
            for px in 0..x {
                self.set_pixel(world, x0 + px, y0 + y);
                self.set_pixel(world, x0 + px, y0 - y);
                self.set_pixel(world, x0 - px, y0 + y);
                self.set_pixel(world, x0 - px, y0 - y);
            }
            error = 2 * (delta + y) - 1;
            if delta < 0 && error <= 0 {
                x = x + 1;
                delta += 2 * x + 1;
                continue;
            }
            if delta >= 0 && error > 0 {
                y = y - 1;
                delta += 1 - 2 * y;
                continue;
            }
            x = x + 1;
            delta += 2 * (x - y);
            y = y - 1;
        }
    }
}

impl Default for Brush {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub fn create_default_brush() -> Brush {
    Brush::default()
}
