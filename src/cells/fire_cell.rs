use crate::{cell::{CellCommonProperties, CellContext, CellLike}, space::Space};

struct FireCell {
    pub comm_props: CellCommonProperties,
}

impl FireCell {
    fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self {
            comm_props: cp,
        }
    }
}

impl CellLike for FireCell {   
    fn tick(&self, cctx: CellContext, space: &mut Space) {
        
    }
}
