use wasm_bindgen::prelude::*;

use crate::space::Space;

#[derive(Clone, Debug)]
pub struct CellCommonProperties {
    pub name: &'static str,
    pub cell_type: CellType,
    pub density: f64,
    pub temp_coefficient: f64,
    pub flammable: bool,
    pub dissolvable: bool,
    pub color: &'static str,
}

/// Инкапсюляция логики поведения ячейки при обновлении мира
pub trait CellLike {
    fn tick(&self, cctx: CellContext, space: &mut Space);
}

#[derive(Clone, Copy, Debug, PartialEq, std::cmp::Eq, Hash)]
#[wasm_bindgen]
pub enum CellType {
    Empty,
    Rock,
    Wood,
    Sand,
    Gunpowder,
    Water,
    Oil,
    Propane,
    Fire,
    Lava,
    Acid,
    Vapor,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub cell_type: CellType,
    pub dx: f32,
    pub dy: f32,
    pub temp: i8,
}

pub struct CellContext {
    pub x: usize,
    pub y: usize,
    pub cell_info: Cell,
}
