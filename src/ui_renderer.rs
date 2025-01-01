use web_sys::CanvasRenderingContext2d;

pub struct UIRenderer2d {
    ctx: CanvasRenderingContext2d,
}

impl UIRenderer2d {
    pub fn from_canvas_ctx(ctx: CanvasRenderingContext2d) -> Self {
        UIRenderer2d { ctx }
    }
    
    pub fn render(&self) {
        self.ctx.begin_path();
        self.ctx.stroke();
    }
}

impl From<CanvasRenderingContext2d> for UIRenderer2d {
    fn from(ctx: CanvasRenderingContext2d) -> Self {
        UIRenderer2d { ctx }
    }
}
