use crate::{cell::{CellCommonProperties, CellContext, CellLike}, space::Space};

struct WaterCell {
    pub comm_props: CellCommonProperties,
}

impl WaterCell {
    fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self {
            comm_props: cp,
        }
    }
}

impl CellLike for WaterCell {   
    fn tick(&self, cctx: CellContext, space: &mut Space) {
        
    }
}
