use crate::{
    cell::{CellCommonProperties, CellContext, CellLike},
    space::Space,
};

pub struct EmptyCell {
    pub comm_props: CellCommonProperties,
}

impl EmptyCell {
    pub fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self { comm_props: cp }
    }
}

impl CellLike for EmptyCell {
    fn tick(&self, _: CellContext, _: &mut Space) {}
}
