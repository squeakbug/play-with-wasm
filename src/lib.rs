mod utils;
mod cell;
mod space;
mod simulator;
mod cells;
mod config;
mod world;
mod brush;

use brush::{Brush, BrushMode, BrushType};
use cell::{Cell, CellType};
use wasm_bindgen::prelude::*;
use world::World;
use web_sys::{ Document, Window, HtmlElement, HtmlCanvasElement, CanvasRenderingContext2d, MouseEvent, TouchEvent };
use crate::config::CELL_PROPERTIES;

/// Модель представления мира
/// 
/// Здесь живет состояние интерфейса
/// Представление (JS frontend) может только читать модель и отправлять команды
#[wasm_bindgen]
pub struct WorldVM {
    world: World,
    paused: bool,
    cell_type: CellType,
    brush: Brush
}

#[wasm_bindgen]
impl WorldVM {
    pub fn with_world(world: World) -> Self {
        WorldVM {
            world,
            paused: true,
            cell_type: CellType::Rock,
            brush: Brush::new(),
        }
    }

    pub fn tick(&mut self) {
        //self.world.tick();
    }

    pub fn tap_on_grid(&mut self, y: usize, x: usize) {
        self.world.tap_on_grid(y, x, self.cell_type, &self.brush);
    }

    pub fn pause_simulation(&mut self) {
        if !self.paused {
            self.world.pause();
        }
    }

    pub fn resume_simultaion(&mut self) {
        if self.paused {
            self.world.resume();
        }
    }

    pub fn set_cell_type(&mut self, ct: CellType) {
        self.cell_type = ct;
    }

    pub fn set_brush_size(&mut self, bs: u32) {
        self.brush.brush_size = bs;
    }

    pub fn set_brush_mode(&mut self, bm: BrushMode) {
        self.brush.brush_mode = bm;
    }

    pub fn set_brush_type(&mut self, bt: BrushType) {
        self.brush.brush_type = bt;
    }

    pub fn reset_world(&mut self) {
        self.world.reset();
    }

    pub fn render_cell_on_canvas(&self, canvas: HtmlCanvasElement) {
        let cells = self.world.get_cells();
        let ctx = canvas
            .get_context("2d").unwrap().unwrap()
            .dyn_into::<CanvasRenderingContext2d>().unwrap();
        ctx.begin_path();

        let height = self.world.get_height();
        let width = self.world.get_width();
        for row in 0..height {
            for col in 0..width {
                let indx = self.world.get_indx(row, col);
                let cell = cells[indx];

                ctx.set_fill_style(&JsValue::from(CELL_PROPERTIES[cell.cell_type as usize].color));
                ctx.fill_rect(
                    (col * (5 + 1) + 1) as f64,
                    (row * (5 + 1) + 1) as f64,
                    5 as f64,
                    5 as f64
                );
            }
        }

        ctx.stroke();
    }
}

#[wasm_bindgen]
pub fn create_world() -> WorldVM {
    let world = World::new();
    WorldVM::with_world(world)
}
