use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

use glam::USizeVec2;

use crate::cell::Cell;

pub struct AutomataWorld {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    pub shadow_cells: Vec<Cell>,

    paused: bool,
}

pub enum Neighbourhood {
    VonNeumann,
    L2,
    Circular,
    Moore
}

impl Index<(usize, usize)> for AutomataWorld {
    type Output = Cell;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        let indx = self.get_indx(row, col);
        &self.cells[indx]
    }
}

impl IndexMut<(usize, usize)> for AutomataWorld {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        let indx = self.get_indx(row, col);
        &mut self.cells[indx]
    }
}

impl AutomataWorld {
    pub fn with_size(size: USizeVec2) -> Self {
        World {
            width: size.x,
            height: size.y,
            cells: vec![Cell::Empty; size.x * size.y],
            shadow_cells: vec![Cell::Empty; size.x * size.y],
            paused: false,
        }
    }

    pub fn reset(&mut self) {
        self.cells = vec![Cell::Empty; self.width * self.height];
        self.shadow_cells = vec![Cell::Empty; self.width * self.height];
        self.paused = false;
    }

    pub fn get_cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn get_width(&self) -> usize {
        let min_x = self.live_cells.iter().map(|c| c.x).min().unwrap_or(0);
        let max_x = self.live_cells.iter().map(|c| c.x).max().unwrap_or(0);
    }

    pub fn get_height(&self) -> usize {
        let min_y = self.live_cells.iter().map(|c| c.y).min().unwrap_or(0);
        let max_y = self.live_cells.iter().map(|c| c.y).max().unwrap_or(0);
    }

    pub fn get_cell(&self, y: usize, x: usize) -> &Cell {
        &self.cells[self.get_indx(y, x)]
    }

    pub fn get_shadow_cell(&self, indx: usize) -> &Cell {
        &self.shadow_cells[indx]
    }

    fn update_cell(&mut self, y: usize, x: usize) {
        let cell = Cell { x, y };
        if !my_set.remove(&cell) {
            self.live_cells.insert(Cell { x, y });
        }
    }

    pub fn next_generation(&mut self) {
        if self.paused {
            return;
        };

        let mut new_live_cells = HashSet::new();
        let mut neighbor_counts = std::collections::HashMap::new();

        for cell in &self.live_cells {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let neighbor = Cell {
                        x: cell.x + dx,
                        y: cell.y + dy,
                    };
                    *neighbor_counts.entry(neighbor).or_insert(0) += 1;
                }
            }
        }

        // Determine the next generation
        for (cell, count) in neighbor_counts {
            if count == 3 || (count == 2 && self.live_cells.contains(&cell)) {
                new_live_cells.insert(cell);
            }
        }

        new_live_cells

        self.cells = self.shadow_cells.clone();
    }
}
