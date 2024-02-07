use crate::{cell::{CellCommonProperties, CellContext, CellLike}, space::Space};

struct EmptyCell {
    pub comm_props: CellCommonProperties,
}

impl EmptyCell {
    fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self {
            comm_props: cp,
        }
    }
}

impl CellLike for EmptyCell {   
    fn tick(&self, cctx: CellContext, space: &mut Space) {
        
    }
}
