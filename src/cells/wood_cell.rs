use crate::{
    cell::{CellCommonProperties, CellContext, CellLike},
    space::Space,
};

pub struct WoodCell {
    pub comm_props: CellCommonProperties,
}

impl WoodCell {
    pub fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self { comm_props: cp }
    }
}

impl CellLike for WoodCell {
    fn tick(&self, _: CellContext, _: &mut Space) {}
}
