use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};

pub struct BackgroundRenderer2d {
    ctx: CanvasRenderingContext2d,
}

impl BackgroundRenderer2d {
    pub fn render(&self) {
        self.ctx.begin_path();
        self.ctx.stroke();
    }
}

impl From<CanvasRenderingContext2d> for BackgroundRenderer2d {
    fn from(ctx: CanvasRenderingContext2d) -> Self {
        BackgroundRenderer2d { ctx }
    }
}
