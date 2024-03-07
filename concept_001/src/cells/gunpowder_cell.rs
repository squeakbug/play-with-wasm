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
    fn tick(&self, cctx: CellContext, space: &mut Space) {}
}
