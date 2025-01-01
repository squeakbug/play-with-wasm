use std::cmp::{Eq, PartialEq};

use wasm_bindgen::prelude::*;

use crate::{shared::Vec2d, world::World};

#[wasm_bindgen]
#[derive(Clone)]
pub enum Shape {
    Square,
    Circle,
}

#[wasm_bindgen]
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Tool {
    Delete,
    Rock,
    Wood,
    Sand,
    Gunpowder,
    Water,
    Oil,
    Propane,
    Fire,
    Lava,
    Acid,
    Vapor,
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Rock
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Brush {
    tool: Tool,
    shape: Shape,
    size: usize,
    left_top_coord: Vec2d<usize>,
}

impl Brush {
    pub fn new() -> Self {
        Brush {
            tool: Tool::default(),
            shape: Shape::Circle,
            size: 1,
            left_top_coord: Vec2d { x: 0, y: 0 },
        }
    }

    pub fn apply(&self, world: &mut World) {
        match self.shape {
            Shape::Square => self.apply_square(world),
            Shape::Circle => self.apply_circle(world),
        }
    }

    pub fn set_position(&mut self, x: usize, y: usize) {
        self.left_top_coord = Vec2d { x, y };
    }
}

impl Brush {
    fn apply_square(&self, world: &mut World) {
        // let new_cell = Cell {
        //     cell_type: ct,
        //     dx: 0.,
        //     dy: 0.,
        //     temp: 27,
        // };
        // if !self.paused {
        //     let bs2 = brush.brush_size / 2;
        //     for x in (x - bs2)..(x + bs2 + 1) {
        //         for y in (y - bs2)..(y + bs2 + 1) {
        //             let indx = self.space.get_indx(y, x);
        //             self.space.shadow_cells[indx] = new_cell;
        //             self.space.cells[indx] = new_cell;
        //         }
        //     }
        // }
    }

    fn apply_circle(&self, world: &mut World) {

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
