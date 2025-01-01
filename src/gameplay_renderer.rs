use web_sys::{CanvasRenderingContext2d, WebGlRenderingContext, WebGl2RenderingContext};

use crate::{cell::Cell, world::World};

pub struct _GameplayRendererWebGL {
    gl: WebGlRenderingContext,
}

pub struct _GameplayRendererWebGL2 {
    gl: WebGl2RenderingContext,
}

pub struct GameplayRenderer2d {
    ctx: CanvasRenderingContext2d,
}

impl GameplayRenderer2d {
    pub fn render_world(&self, world: &World) {
        self.ctx.begin_path();

        let height = self.ctx.canvas().unwrap().height() as usize;
        let width = self.ctx.canvas().unwrap().width() as usize;
        for row in 0..height {
            for col in 0..width {
                let cell = world[(row, col)];
                match cell {
                    Cell::Empty => {
                        self.ctx.set_fill_style_str("#ffffff");
                        self.ctx.fill_rect(col as f64, row as f64, 1f64, 1f64);
                    },
                    Cell::Filled(cell) => {
                        self.ctx.set_fill_style_str(cell.color);
                        self.ctx.fill_rect(col as f64, row as f64, 1f64, 1f64);
                    },
                }
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
