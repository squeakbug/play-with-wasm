use crate::{cell::{CellCommonProperties, CellContext, CellLike}, space::Space};

struct RockCell {
    pub comm_props: CellCommonProperties,
}

impl RockCell {
    fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self {
            comm_props: cp,
        }
    }
}

impl CellLike for RockCell {   
    fn tick(&self, cctx: CellContext, space: &mut Space) {
        
    }
}
