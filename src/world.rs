use wasm_bindgen::prelude::*;

use crate::{brush::Brush, cell::{Cell, CellDispatcher, CellType}, simulator::Simulator, space::Space};

#[wasm_bindgen]
pub struct World {
    space: Space,
    sim: Simulator,
}

impl World {
    pub fn new() -> Self {
        let cd = CellDispatcher::new();
        World {
            space: Space::with_witdh_height(100, 100),
            sim: Simulator::with_dispatcher(cd)
        }
    }

    pub fn tap_on_grid(&mut self, y: usize, x: usize, ct: CellType, brush: &Brush) {
        self.space.cells[x + self.space.height * y] = Cell {
            cell_type: ct,
            dx: 0.,
            dy: 0.,
            temp: 27,
        }
    }

    pub fn pause(&mut self) {

    }

    pub fn resume(&mut self) {

    }

    pub fn reset(&mut self) {
        
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
        let space = &mut self.space;
        space.generation += 1;
        self.sim.tick(space).unwrap();
        std::mem::swap(&mut space.cells, &mut space.shadow_cells);
    }
}