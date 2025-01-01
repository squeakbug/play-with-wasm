use web_sys::{CanvasRenderingContext2d, WebGlRenderingContext, WebGl2RenderingContext};

use crate::{config::CELL_PROPERTIES, world::World};

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

        let cells = world.get_cells();
        let height = self.ctx.canvas().unwrap().height() as usize;
        let width = self.ctx.canvas().unwrap().width() as usize;
        for row in 0..height {
            for col in 0..width {
                let indx = world.get_indx(row, col);
                let cell = cells[indx];
                self.ctx.set_fill_style_str(
                    CELL_PROPERTIES[cell.cell_type as usize].color
                );
                self.ctx.fill_rect(
                   col as f64, row as f64, 1_f64, 1_f64,
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
