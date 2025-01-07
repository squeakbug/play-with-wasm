mod brush;
mod cell;
mod render;
mod rle;
mod physicslife;
mod quicklife;

use glam::USizeVec2;
use wasm_bindgen::prelude::*;
use derive_builder::Builder;
use gloo_utils::format::JsValueSerdeExt;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use brush::Brush;
use render::{
    background_renderer::BackgroundRenderer2d,
    gameplay_renderer::GameplayRenderer2d,
    ui_renderer::UIRenderer2d,
};
use crate::physicslife::World;

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

    pub fn tap_on_grid(&mut self, y: usize, x: usize) {
        let world_coord = self.background_renderer
            .apply_reverse_frame_matrix(USizeVec2::new(x, y));
        self.brush.set_position(world_coord.x, world_coord.y);
        self.brush.apply(&mut self.world);
    }

    pub fn hover_on_grid(&mut self, y: f32, x: f32) {
        self.brush.set_position(x as usize, y as usize);
    }

    pub fn toggle_simulation(&mut self) {
        self.world.toggle_pause();
    }

    pub fn set_brush(&mut self, brush: Brush) {
        self.brush = brush;
    }

    pub fn reset_world(&mut self) {
        self.world.reset();
    }

    pub fn render(&mut self) {
        self.background_renderer.render_world(&self.world);
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
    let world = World::with_size(USizeVec2::new(50, 50));

    let ui_ctx = ui_canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let gp_ctx = gameplay_canvas
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
    let bg_renderer = BackgroundRenderer2d::new(bg_ctx, &world);

    WorldVMBuilder::default()
        .world(world)
        .brush(Brush::new())
        .ui_renderer(ui_ctx.into())
        .gameplay_renderer(gp_ctx.into())
        .background_renderer(bg_renderer)
        .build()
        .unwrap()
}
