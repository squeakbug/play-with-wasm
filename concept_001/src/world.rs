use wasm_bindgen::prelude::*;

use crate::{
    brush::Brush,
    cell::{Cell, CellType},
    config::get_cell_logics,
    simulator::Simulator,
    space::Space,
};

#[wasm_bindgen]
pub struct World {
    space: Space,
    sim: Simulator,
    paused: bool,
}

impl Default for World {
   fn default() -> Self {
       Self::new()
   }
}

impl World {
    pub fn new() -> Self {
        let logics = get_cell_logics();
        World {
            space: Space::with_witdh_height(100, 100),
            sim: Simulator::with_logics(logics),
            paused: false,
        }
    }

    pub fn tap_on_grid(&mut self, y: usize, x: usize, ct: CellType, brush: &Brush) {
        let new_cell = Cell {
            cell_type: ct,
            dx: 0.,
            dy: 0.,
            temp: 27,
        };
        if !self.paused {
            let bs2 = brush.brush_size / 2;
            for x in (x - bs2)..(x + bs2 + 1) {
                for y in (y - bs2)..(y + bs2 + 1) {
                    let indx = self.space.get_indx(y, x);
                    self.space.shadow_cells[indx] = new_cell;
                    self.space.cells[indx] = new_cell;
                }
            }
        }
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn reset(&mut self) {
        let logics = get_cell_logics();
        self.space = Space::with_witdh_height(100, 100);
        self.sim = Simulator::with_logics(logics);
        self.paused = false;
    }

    pub fn get_cells(&self) -> &Vec<Cell> {
        &self.space.cells
    }

    pub fn get_width(&self) -> usize {
        self.space.width
    }

    pub fn get_height(&self) -> usize {
        self.space.height
    }

    pub fn get_indx(&self, y: usize, x: usize) -> usize {
        self.space.get_indx(y, x)
    }

    pub fn tick(&mut self) {
        if self.paused {
            return;
        };
        let space = &mut self.space;
        space.generation += 1;
        self.sim.tick(space).unwrap();
        space.cells = space.shadow_cells.clone();
    }
}
