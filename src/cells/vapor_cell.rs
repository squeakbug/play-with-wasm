use crate::{
    cell::{CellCommonProperties, CellContext, CellLike},
    space::Space,
};

pub struct VaporCell {
    pub comm_props: CellCommonProperties,
}

impl VaporCell {
    pub fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self { comm_props: cp }
    }
}

impl CellLike for VaporCell {
    fn tick(&self, cctx: CellContext, space: &mut Space) {}
}
