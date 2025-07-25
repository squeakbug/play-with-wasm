use std::ops::{Index, IndexMut};

use glam::USizeVec2;

use crate::cell::Cell;

pub struct World {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>, // Render gets cells from here
    pub shadow_cells: Vec<Cell>, // During rendering tick procedure updates this buffer

    paused: bool,
}

impl Index<(usize, usize)> for World {
    type Output = Cell;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        let indx = self.get_indx(row, col);
        &self.cells[indx]
    }
}

impl IndexMut<(usize, usize)> for World {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        let indx = self.get_indx(row, col);
        &mut self.cells[indx]
    }
}

impl World {
    fn get_indx(&self, y: usize, x: usize) -> usize {
        y * self.width + x
    }

    fn saturate_sub(v: usize, a: usize, min: usize) -> usize {
        if v <= min + a {
            min
        } else {
            v - a
        }
    }

    fn saturate_add(v: usize, a: usize, max: usize) -> usize {
        if v >= max - a {
            max
        } else {
            v + a
        }
    }

    // TODO: Use convolution
    fn update_cell(&mut self, y: usize, x: usize) {
        let ny = World::saturate_add(y, 1, self.height - 1);
        let nindx = self.get_indx(ny, x);
        if y != self.height - 1 {
            let cindx = self.get_indx(y, x);
            if let Cell::Empty = self.shadow_cells[nindx] {
                self.shadow_cells[nindx] = self.shadow_cells[cindx];
            } else {
                let (dy1, dx1) = (
                    World::saturate_add(y, 1, self.height - 1),
                    World::saturate_add(x, 1, self.width - 1),
                );
                let dindx1 = self.get_indx(dy1, dx1);
                let (dy2, dx2) = (
                    World::saturate_add(y, 1, self.height - 1),
                    World::saturate_sub(x, 1, 0),
                );
                let dindx2 = self.get_indx(dy2, dx2);
                let (dy3, dx3) = (y, World::saturate_add(x, 1, self.width - 1));
                let dindx3 = self.get_indx(dy3, dx3);
                let (dy4, dx4) = (y, World::saturate_sub(x, 1, 0));
                let dindx4 = self.get_indx(dy4, dx4);
                if let Cell::Empty = self.shadow_cells[dindx1] {
                    self.shadow_cells[dindx1] = self.shadow_cells[cindx];
                } else if let Cell::Empty = self.shadow_cells[dindx2] {
                    self.shadow_cells[dindx2] = self.shadow_cells[cindx];
                } else if let Cell::Empty = self.shadow_cells[dindx3] {
                    self.shadow_cells[dindx3] = self.shadow_cells[cindx];
                } else if let Cell::Empty = self.shadow_cells[dindx4] {
                    self.shadow_cells[dindx4] = self.shadow_cells[cindx];
                } else {
                    // Ячейка остается в прежнем состоянии
                }
            }
        }
    }
}

impl World {
    pub fn with_size(size: USizeVec2) -> Self {
        World {
            width: size.x,
            height: size.y,
            cells: vec![Cell::Empty; size.x * size.y],
            shadow_cells: vec![Cell::Empty; size.x * size.y],
            paused: false,
        }
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn reset(&mut self) {
        self.cells = vec![Cell::Empty; self.width * self.height];
        self.shadow_cells = vec![Cell::Empty; self.width * self.height];
        self.paused = false;
    }

    pub fn get_cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn get_cell(&self, y: usize, x: usize) -> &Cell {
        &self.cells[self.get_indx(y, x)]
    }

    pub fn get_shadow_cell(&self, y: usize, x: usize) -> &Cell {
        &self.shadow_cells[self.get_indx(y, x)]
    }

    pub fn get_shadow_cell_mut(&mut self, y: usize, x: usize) -> &mut Cell {
        let indx = self.get_indx(y, x);
        &mut self.shadow_cells[indx]
    }

    pub fn tick(&mut self) {
        if self.paused {
            return;
        };

        for y in 0..self.height {
            for x in 0..self.width {
                self.update_cell(y, x);
            }
        }

        self.cells = self.shadow_cells.clone();
    }
}
