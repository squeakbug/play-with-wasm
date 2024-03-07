use crate::{
    cell::{CellCommonProperties, CellContext, CellLike},
    space::Space,
};

pub struct AcidCell {
    pub comm_props: CellCommonProperties,
}

impl AcidCell {
    pub fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self { comm_props: cp }
    }
}

impl CellLike for AcidCell {
    fn tick(&self, cctx: CellContext, space: &mut Space) {}
}
