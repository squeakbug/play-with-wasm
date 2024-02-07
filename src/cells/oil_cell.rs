use crate::{cell::{CellCommonProperties, CellContext, CellLike}, space::Space};

struct OilCell {
    pub comm_props: CellCommonProperties,
}

impl OilCell {
    fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self {
            comm_props: cp,
        }
    }
}

impl CellLike for OilCell {   
    fn tick(&self, cctx: CellContext, space: &mut Space) {
        
    }
}
