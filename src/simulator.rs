use crate::{
    cell::{CellContext, CellLike},
    space::Space,
};

pub struct Simulator {
    pub cell_logics: Vec<Box<dyn CellLike>>,
}

impl Simulator {
    pub fn with_logics(cell_logics: Vec<Box<dyn CellLike>>) -> Self {
        Simulator { cell_logics }
    }

    pub fn tick(&self, space: &mut Space) -> Result<(), i32> {
        let height = space.height();
        let width = space.width();
        for y in (0..height).rev() {
            for x in 0..width {
                let indx = space.get_indx(y, x);
                let cell = space.get_shadow_cell(indx);

                let cell_logic = &self.cell_logics[cell.cell_type as usize];
                cell_logic.tick(
                    CellContext {
                        x,
                        y,
                        cell_info: cell,
                    },
                    space,
                );
            }
        }

        Ok(())
    }
}
