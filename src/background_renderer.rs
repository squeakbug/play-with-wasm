use web_sys::CanvasRenderingContext2d;

use crate::{shared::Vec2d, world::World};

pub struct BackgroundRenderer2d {
    ctx: CanvasRenderingContext2d,
    scalex: f64,
    scaley: f64,
}

impl BackgroundRenderer2d {
    // TODO: use rayon
    // TODO: apply canvas API for scaling
    pub fn apply_reverse_frame_matrix(&self, client: Vec2d<f64>) -> Vec2d<usize> {
        let row = client.x / self.scalex;
        let col = client.y / self.scaley;
        Vec2d { y: row as usize, x: col as usize }
    }

    pub fn apply_frame_matrix(&self, client: Vec2d<usize>) -> Vec2d<f64> {
        let row = client.x as f64 * self.scalex;
        let col = client.y as f64 * self.scaley;
        Vec2d { y: row, x: col }
    }

    fn render_grid(&self) {
        self.ctx.set_stroke_style_str("#eeeeee");

        let height = self.ctx.canvas().unwrap().height();
        let width = self.ctx.canvas().unwrap().width();

        for i in (0..width).filter(|x| x % 5 == 0) {
            let x = i as f64 * self.scalex;

            self.ctx.set_line_width(2.0);
            self.ctx.move_to(x, 0.0);
            self.ctx.line_to(x, height as f64);
        
            self.ctx.set_line_width(1.0);
            for j in 1..4 {
                let x = (i + j) as f64 * self.scalex;
                self.ctx.move_to(x, 0.0);
                self.ctx.line_to(x, height as f64);
            }
        }

        for i in (0..height).filter(|x| x % 5 == 0) {
            let y = i as f64 * self.scaley;

            self.ctx.set_line_width(2.0);
            self.ctx.move_to(0.0, y);
            self.ctx.line_to(height as f64, y);
        
            self.ctx.set_line_width(1.0);
            for j in 1..4 {
                let y = (i + j) as f64 * self.scaley;
                self.ctx.move_to(0.0, y);
                self.ctx.line_to(height as f64, y);
            }
        }
    }

    pub fn resize(&mut self, world: &World) {
        let bounding_rect = self.ctx.canvas().unwrap().get_bounding_client_rect();
        let rect_height = bounding_rect.height();
        let rect_width = bounding_rect.width();
        let width = world.get_width();
        let height = world.get_height();

        self.scalex = rect_width / width as f64;
        self.scaley = rect_height / height as f64;
    }

    pub fn render(&self) {
        self.ctx.begin_path();
        self.render_grid();
        self.ctx.stroke();
    }
}

impl From<CanvasRenderingContext2d> for BackgroundRenderer2d {
    fn from(ctx: CanvasRenderingContext2d) -> Self {
        BackgroundRenderer2d { ctx, scalex: 1.0, scaley:1.0 }
    }
}
