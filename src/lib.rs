mod brush;
mod cell;
mod cells;
mod config;
mod simulator;
mod space;
mod world;
mod shared;
mod ui_renderer;
mod gameplay_renderer;
mod background_renderer;

use wasm_bindgen::prelude::*;
use derive_builder::Builder;
use gloo_utils::format::JsValueSerdeExt;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use shared::Vec2d;
use world::World;
use brush::Brush;
use cell::CellType;
use ui_renderer::UIRenderer2d;
use background_renderer::BackgroundRenderer2d;
use gameplay_renderer::GameplayRenderer2d;

#[wasm_bindgen]
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct WorldVM {
    world: World,
    brush: Brush,
    ui_renderer: UIRenderer2d,
    gameplay_renderer: GameplayRenderer2d,
    background_renderer: BackgroundRenderer2d,
}

#[wasm_bindgen]
impl WorldVM {
    pub fn poll(&mut self) {
        self.world.tick();
    }

    pub fn tap_on_grid(&mut self, y: f64, x: f64) {
        let world_coord = self.background_renderer
            .apply_reverse_frame_matrix(Vec2d { x, y });
        self.brush.set_position(world_coord.x, world_coord.y);
        self.brush.apply(&mut self.world);
    }

    pub fn hover_on_grid(&mut self, y: f64, x: f64) {
        self.brush.set_position(x as usize, y as usize);
    }

    pub fn pause_simulation(&mut self) {
        self.world.pause();
    }

    pub fn resume_simultaion(&mut self) {
        self.world.resume();
    }

    pub fn set_brush(&mut self, brush: Brush) {
        self.brush = brush;
    }

    pub fn reset_world(&mut self) {
        self.world.reset();
    }

    pub fn render(&self) {
        self.background_renderer.render();
        self.gameplay_renderer.render_world(&self.world);
        self.ui_renderer.render();
    }
}

#[wasm_bindgen]
pub fn create_world(
    ui_canvas: HtmlCanvasElement,
    gameplay_canvas: HtmlCanvasElement,
    background_canvas: HtmlCanvasElement,
) -> WorldVM {
    let ui_ctx = ui_canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let gm_ctx = gameplay_canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let bg_ctx_options = JsValue::from_serde(&serde_json::json!({
        "alpha": false,
    })).unwrap();
    let bg_ctx = background_canvas
        .get_context_with_context_options("2d", &bg_ctx_options)
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    WorldVMBuilder::default()
        .ui_renderer(ui_ctx.into())
        .gameplay_renderer(gm_ctx.into())
        .background_renderer(bg_ctx.into())
        .build()
        .unwrap()
}
