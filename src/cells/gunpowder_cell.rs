use crate::cells::common::{saturate_add, saturate_sub};
use crate::CellType;
use crate::{
    cell::{CellCommonProperties, CellContext, CellLike},
    space::Space,
};

pub struct GunpowderCell {
    pub comm_props: CellCommonProperties,
}

impl GunpowderCell {
    pub fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self { comm_props: cp }
    }
}

impl CellLike for GunpowderCell {
    fn tick(&self, cctx: CellContext, space: &mut Space) {
        let (cy, cx) = (cctx.y, cctx.x);
        let nindx = space.get_indx(saturate_add(cy, 1, space.height - 1), cx);
        if cy != space.height - 1 {
            let cindx = space.get_indx(cy, cx);
            if space.cells[nindx].cell_type == CellType::Empty {
                space.shadow_cells[nindx] = space.cells[cindx];
                space.shadow_cells[cindx].cell_type = CellType::Empty;
            } else {
                let (dy1, dx1) = (
                    saturate_add(cy, 1, space.height - 1),
                    saturate_add(cx, 1, space.width - 1),
                );
                let dindx1 = space.get_indx(dy1, dx1);
                let (dy2, dx2) = (
                    saturate_add(cy, 1, space.height - 1),
                    saturate_sub(cx, 1, 0),
                );
                let dindx2 = space.get_indx(dy2, dx2);
                if space.cells[dindx1].cell_type == CellType::Empty {
                    space.shadow_cells[dindx1] = space.cells[cindx];
                    space.shadow_cells[cindx].cell_type = CellType::Empty;
                } else if space.cells[dindx2].cell_type == CellType::Empty {
                    space.shadow_cells[dindx2] = space.cells[cindx];
                    space.shadow_cells[cindx].cell_type = CellType::Empty;
                } else {
                    // Ячейка остается в прежнем состоянии
                }
            }
        }
    }
}
