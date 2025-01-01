#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Empty,
    Filled(FilledCell),
}

#[derive(Clone, Copy, Debug)]
pub struct FilledCell {
    pub color: &'static str,

    pub dx: f32,
    pub dy: f32,
    pub density: f64,

    pub temp: f32,

    pub redox_activity: f64,
}

impl FilledCell {
    pub fn get_kinetic_e(&self) -> f32 {
        let density = self.density;
        let (dx, dy) = (self.dx, self.dy);
        ((dx * dx + dy * dy) * density as f32) / 2.0
    }
}
