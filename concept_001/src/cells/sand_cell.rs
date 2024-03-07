use crate::{
    cell::{CellCommonProperties, CellContext, CellLike},
    space::Space,
};

pub struct SandCell {
    pub comm_props: CellCommonProperties,
}

impl SandCell {
    pub fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self { comm_props: cp }
    }
}

impl CellLike for SandCell {
    fn tick(&self, cctx: CellContext, space: &mut Space) {}
}
