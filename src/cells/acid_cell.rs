use crate::{cell::{CellCommonProperties, CellContext, CellLike}, space::Space};

struct AcidCell {
    pub comm_props: CellCommonProperties,
}

impl AcidCell {
    fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self {
            comm_props: cp,
        }
    }
}

impl CellLike for AcidCell {   
    fn tick(&self, cctx: CellContext, space: &mut Space) {
        
    }
}
