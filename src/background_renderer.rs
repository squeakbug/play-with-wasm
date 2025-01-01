use web_sys::CanvasRenderingContext2d;

use crate::{shared::Vec2d, world::World};

pub struct BackgroundRenderer2d {
    ctx: CanvasRenderingContext2d,
    scalex: f64,
    scaley: f64,
}

impl BackgroundRenderer2d {
    pub fn new(ctx: CanvasRenderingContext2d, world: &World) -> Self {
        let (scalex, scaley) = BackgroundRenderer2d::get_scale(&ctx, world);
        BackgroundRenderer2d { ctx, scalex, scaley }
    }

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

    pub fn resize(&mut self, world: &World) {
        let (scalex, scaley) = BackgroundRenderer2d::get_scale(&self.ctx, world);
        self.scalex = scalex;
        self.scaley = scaley;
    }

    pub fn render_world(&self, world: &World) {
        self.ctx.begin_path();
        self.render_grid(world);
        self.ctx.stroke();
    }
}

impl BackgroundRenderer2d {
    fn render_grid(&self, world: &World) {
        self.ctx.set_stroke_style_str("#111111");

        let bounding_rect = self.ctx.canvas().unwrap().get_bounding_client_rect();
        let rect_height = bounding_rect.height();
        let rect_width = bounding_rect.width();
        let width = world.get_width();
        let height = world.get_height();

        self.ctx.clear_rect(0.0, 0.0, rect_width, rect_height);

        for i in (0..width).filter(|x| x % 5 == 0) {
            let x = i as f64 * self.scalex;

            self.ctx.set_line_width(3.0);
            self.ctx.move_to(x, 0.0);
            self.ctx.line_to(x, rect_height);
        
            self.ctx.set_line_width(1.0);
            for j in 1..5 {
                let thin_x = x + j as f64 * self.scalex;
                self.ctx.move_to(thin_x, 0.0);
                self.ctx.line_to(thin_x, rect_height);
            }
        }

        for i in (0..height).filter(|x| x % 5 == 0) {
            let y = i as f64 * self.scaley;

            self.ctx.set_line_width(3.0);
            self.ctx.move_to(0.0, y);
            self.ctx.line_to(rect_width, y);
        
            self.ctx.set_line_width(1.0);
            for j in 1..5 {
                let thin_y = y + j as f64 * self.scaley;
                self.ctx.move_to(0.0, thin_y);
                self.ctx.line_to(rect_width, thin_y);
            }
        }
    }

    fn get_scale(ctx: &CanvasRenderingContext2d, world: &World) -> (f64, f64) {
        let bounding_rect = ctx.canvas().unwrap().get_bounding_client_rect();
        let rect_height = bounding_rect.height();
        let rect_width = bounding_rect.width();
        let width = world.get_width();
        let height = world.get_height();

        let scalex = rect_width / width as f64;
        let scaley = rect_height / height as f64;
        (scalex, scaley)
    }
}
