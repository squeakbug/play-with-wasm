use crate::cell::{Cell, CellType};

pub struct Space {
    pub width: usize,
    pub height: usize,
    pub generation: u8,
    pub cells: Vec<Cell>,
    pub shadow_cells: Vec<Cell>,
}

impl Space {
    pub fn with_witdh_height(width: usize, height: usize) -> Self {
        let empty_cell = Cell {
            cell_type: CellType::Empty,
            dx: 0.,
            dy: 0.,
            temp: 27,
        };
        Space {
            width,
            height,
            generation: 0,
            cells: vec![empty_cell; width * height],
            shadow_cells: vec![empty_cell; width * height],
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get_indx(&self, y: usize, x: usize) -> usize {
        self.height * y + x
    }

    pub fn get_shadow_cell(&self, indx: usize) -> Cell {
        self.shadow_cells[indx]
    }
}
