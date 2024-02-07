use crate::{cell::{CellCommonProperties, CellContext, CellLike}, space::Space};

struct SandCell {
    pub comm_props: CellCommonProperties,
}

impl SandCell {
    fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self {
            comm_props: cp,
        }
    }
}

impl CellLike for SandCell {   
    fn tick(&self, cctx: CellContext, space: &mut Space) {
        
    }
}
