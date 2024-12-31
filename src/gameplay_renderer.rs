use web_sys::{window, HtmlCanvasElement};
use web_sys::{CanvasRenderingContext2d, WebGlRenderingContext, WebGl2RenderingContext};

pub struct GameplayRendererWebGL {
    gl: WebGlRenderingContext,
}

pub struct GameplayRendererWebGL2 {
    gl: WebGl2RenderingContext,
}

pub struct GameplayRenderer2d {
    ctx: CanvasRenderingContext2d,
}

impl GameplayRenderer2d {
    pub fn render(&self) {
        self.ctx.begin_path();

        let render_data = self.world.get_cells();
        let height = self.world.get_height();
        let width = self.world.get_width();
        for row in 0..height {
            for col in 0..width {
                let indx = self.world.get_indx(row, col);
                let cell = cells[indx];
 
                ctx.set_fill_style(&JsValue::from(
                    CELL_PROPERTIES[cell.cell_type as usize].color,
                ));
                ctx.fill_rect(
                    (col * (5 + 1) + 1) as f64,
                    (row * (5 + 1) + 1) as f64,
                    5_f64,
                    5_f64,
                );
            }
        }
 
        self.ctx.stroke();
    }
}

impl From<CanvasRenderingContext2d> for GameplayRenderer2d {
    fn from(ctx: CanvasRenderingContext2d) -> Self {
        GameplayRenderer2d { ctx }
    }
}
