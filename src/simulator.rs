use crate::{cell::{Cell, CellContext, CellDispatcher, CellLike, CellType}, space::Space};

pub struct Simulator {
    cd: CellDispatcher,
}

impl Simulator {
    pub fn with_dispatcher(cd: CellDispatcher) -> Self {
        Self {
            cd
        }
    }
    
    pub fn tick(&self, space: &mut Space) -> Result<(), i32> {
        let height = space.height();
        let width = space.width();
        for y in (0..height).rev() {
            for x in 0..width {
                let indx = space.get_indx(y, x);
                let cell = space.get_cell(indx);
                
                let cell_logic = self.cd.dispatch(cell.cell_type).map_or(Err(3), |v| Ok(v))?;
                cell_logic.tick(CellContext { x, y, cell_info: cell }, space);
            }
        }

        Ok(())
    }
}