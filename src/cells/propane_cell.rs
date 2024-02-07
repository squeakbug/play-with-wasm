use crate::{cell::{CellCommonProperties, CellContext, CellLike}, space::Space};

struct PropaneCell {
    pub comm_props: CellCommonProperties,
}

impl PropaneCell {
    fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self {
            comm_props: cp,
        }
    }
}

impl CellLike for PropaneCell {   
    fn tick(&self, cctx: CellContext, space: &mut Space) {
        
    }
}
