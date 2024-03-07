use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum BrushType {
    SquarePencil,
    CirclePencil,
}

#[wasm_bindgen]
pub enum BrushMode {
    Create,
    Delete,
}

pub struct Brush {
    pub brush_mode: BrushMode,
    pub brush_type: BrushType,
    pub brush_size: usize,
}

impl Brush {
    pub fn new() -> Self {
        Brush {
            brush_mode: BrushMode::Create,
            brush_type: BrushType::CirclePencil,
            brush_size: 1,
        }
    }
}
