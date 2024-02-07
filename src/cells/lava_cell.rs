use crate::{cell::{CellCommonProperties, CellContext, CellLike}, space::Space};

struct LavaCell {
    pub comm_props: CellCommonProperties,
}

impl LavaCell {
    fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self {
            comm_props: cp,
        }
    }
}

impl CellLike for LavaCell {   
    fn tick(&self, cctx: CellContext, space: &mut Space) {
        
    }
}
