#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Empty,
    Physical(PhysicalCell),
    Conway(ConwayCell)
}

#[derive(Clone, Copy, Debug)]
pub struct ConwayCell {
    pub color: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub enum VonNeumanCellState {
    
}

#[derive(Clone, Copy, Debug)]
pub struct VonNeumanCell {
    pub color: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct PhysicalCellBuilder {
    pub color: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct PhysicalCell {
    pub color: &'static str,
    pub density: f64,
    pub temp: f64,
    pub redox_activity: f64,
}

impl PhysicalCellBuilder {
    pub fn build(&self) -> Cell {
        Cell::Physical(PhysicalCell {
            color: self.color,
            density: 0f64, 
            temp: 0f64, 
            redox_activity: 0f64,
        })
    }
}

impl Default for PhysicalCellBuilder {
    fn default() -> Self {
        PhysicalCellBuilder { color: "TODO" }
    }
}
