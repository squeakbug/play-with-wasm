use crate::{
    cell::{CellCommonProperties, CellContext, CellLike},
    space::Space,
};

pub struct LavaCell {
    pub comm_props: CellCommonProperties,
}

impl LavaCell {
    pub fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self { comm_props: cp }
    }
}

impl CellLike for LavaCell {
    fn tick(&self, cctx: CellContext, space: &mut Space) {}
}
