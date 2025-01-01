use crate::{
    cell::Cell,
    config::get_cell_logics,
    simulator::Simulator,
    space::Space,
};

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
