use std::collections::HashMap;

use crate::{brush::Tool, cell::{Cell, FilledCell}};

pub fn get_default_cells() -> HashMap<Tool, Cell> {
    let x = vec![
        (Tool::Rock, Cell::Filled(FilledCell{
            color: "#000000",
            dx: 0.,
            dy: 0.,
            density: 3.0,
            temp: 27.0,
            redox_activity: 0.0,
        })),
        (Tool::Wood, Cell::Filled(FilledCell{
            color: "#606040",
            dx: 0.0,
            dy: 0.0,
            density: 3.0,
            temp: 27.0,
            redox_activity: 0.0,
        })),
        (Tool::Sand, Cell::Filled(FilledCell{
            color: "#886611",
            dx: 0.0,
            dy: 0.0,
            density: 3.0,
            temp: 27.0,
            redox_activity: 0.0,
        })),
        (Tool::Gunpowder, Cell::Filled(FilledCell{
            color: "#666666",
            dx: 0.0,
            dy: 0.0,
            density: 3.0,
            temp: 27.0,
            redox_activity: 0.0,
        })),
        (Tool::Water, Cell::Filled(FilledCell{
            color: "#0000FF",
            dx: 0.0,
            dy: 0.0,
            density: 1.0,
            temp: 27.0,
            redox_activity: 0.0,
        })),
        (Tool::Oil, Cell::Filled(FilledCell{
            color: "#007777",
            dx: 0.0,
            dy: 0.0,
            density: 0.8,
            temp: 27.0,
            redox_activity: 0.0,
        })),
        (Tool::Propane, Cell::Filled(FilledCell{
            color: "#77FFFF",
            dx: 0.0,
            dy: 0.0,
            density: 0.1,
            temp: 27.0,
            redox_activity: 0.0,
        })),
        (Tool::Fire, Cell::Filled(FilledCell{
            color: "#FF3300",
            dx: 0.0,
            dy: 0.0,
            density: 0.01,
            temp: 27.0,
            redox_activity: 0.0,
        })),
        (Tool::Lava, Cell::Filled(FilledCell{
            color: "#993300",
            dx: 0.0,
            dy: 0.0,
            density: 3.0,
            temp: 27.0,
            redox_activity: 0.0,
        })),
        (Tool::Acid, Cell::Filled(FilledCell{
            color: "#009966",
            dx: 0.0,
            dy: 0.0,
            density: 1.2,
            temp: 27.0,
            redox_activity: 0.0,
        })),
        (Tool::Vapor, Cell::Filled(FilledCell{
            color: "#EEEEFF",
            dx: 0.0,
            dy: 0.0,
            density: 0.1,
            temp: 27.0,
            redox_activity: 0.0,
        })),
    ];

    x.into_iter().collect()
}