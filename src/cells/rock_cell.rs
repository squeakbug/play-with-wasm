use crate::{
    cell::{CellCommonProperties, CellContext, CellLike},
    space::Space,
};

pub struct RockCell {
    pub comm_props: CellCommonProperties,
}

impl RockCell {
    pub fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self { comm_props: cp }
    }
}

impl CellLike for RockCell {
    fn tick(&self, _: CellContext, _: &mut Space) {}
}
