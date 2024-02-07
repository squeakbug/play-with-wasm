use crate::{cell::{CellCommonProperties, CellContext, CellLike}, space::Space};

struct WoodCell {
    pub comm_props: CellCommonProperties,
}

impl WoodCell {
    fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self {
            comm_props: cp,
        }
    }
}

impl CellLike for WoodCell {   
    fn tick(&self, cctx: CellContext, space: &mut Space) {
        
    }
}
