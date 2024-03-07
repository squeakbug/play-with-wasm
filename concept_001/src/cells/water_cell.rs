use crate::{
    cell::{CellCommonProperties, CellContext, CellLike, CellType},
    space::Space,
};

pub struct WaterCell {
    pub comm_props: CellCommonProperties,
}

impl WaterCell {
    pub fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self { comm_props: cp }
    }
}

impl CellLike for WaterCell {
    fn tick(&self, cctx: CellContext, space: &mut Space) {
        let (cy, cx) = (cctx.y, cctx.x);
        let pindx = space.get_indx(cy - 1, cx);
        if cy != 0 && space.cells[pindx].cell_type != CellType::Empty {
            let cindx = space.get_indx(cy, cx);
            space.cells[cindx] = space.cells[pindx];
        }
    }
}
