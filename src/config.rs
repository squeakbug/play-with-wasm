use crate::{
    cell::{CellCommonProperties, CellLike, CellType},
    cells::{
        AcidCell, EmptyCell, FireCell, GunpowderCell, LavaCell, OilCell, PropaneCell, RockCell,
        SandCell, WaterCell, WoodCell,
    },
};

pub static CELL_PROPERTIES: [CellCommonProperties; 11] = [
    CellCommonProperties {
        name: "Empty",
        cell_type: CellType::Empty,
        density: 0.0,
        temp_coefficient: 0.1,
        flammable: false,
        dissolvable: false,
        color: "#FFFFFF",
    },
    CellCommonProperties {
        name: "Rock",
        cell_type: CellType::Rock,
        density: 3.0,
        temp_coefficient: 0.1,
        flammable: false,
        dissolvable: true,
        color: "#000000",
    },
    CellCommonProperties {
        name: "Wood",
        cell_type: CellType::Wood,
        density: 3.0,
        temp_coefficient: 0.8,
        flammable: true,
        dissolvable: true,
        color: "#606040",
    },
    CellCommonProperties {
        name: "Sand",
        cell_type: CellType::Sand,
        density: 3.0,
        temp_coefficient: 0.1,
        flammable: false,
        dissolvable: true,
        color: "#886611",
    },
    CellCommonProperties {
        name: "Gunpowder",
        cell_type: CellType::Gunpowder,
        density: 3.0,
        temp_coefficient: 20.0,
        flammable: true,
        dissolvable: false,
        color: "#666666",
    },
    CellCommonProperties {
        name: "Water",
        cell_type: CellType::Water,
        density: 1.0,
        temp_coefficient: 0.4,
        flammable: false,
        dissolvable: false,
        color: "#0000FF",
    },
    CellCommonProperties {
        name: "Oil",
        cell_type: CellType::Oil,
        density: 0.8,
        temp_coefficient: 10.0,
        flammable: true,
        dissolvable: false,
        color: "#007777",
    },
    CellCommonProperties {
        name: "Propane",
        cell_type: CellType::Propane,
        density: 0.1,
        temp_coefficient: 200.0,
        flammable: true,
        dissolvable: false,
        color: "#77FFFF",
    },
    CellCommonProperties {
        name: "Fire",
        cell_type: CellType::Fire,
        density: 0.01,
        temp_coefficient: 1.0,
        flammable: false,
        dissolvable: false,
        color: "#FF3300",
    },
    CellCommonProperties {
        name: "Lava",
        cell_type: CellType::Lava,
        density: 3.0,
        temp_coefficient: 100.0,
        flammable: false,
        dissolvable: true,
        color: "#993300",
    },
    CellCommonProperties {
        name: "Acid",
        cell_type: CellType::Acid,
        density: 1.2,
        temp_coefficient: 0.1,
        flammable: false,
        dissolvable: false,
        color: "#009966",
    },
];

pub fn get_cell_logics() -> Vec<Box<dyn CellLike>> {
    vec![
        Box::new(EmptyCell::with_comm_props(
            CELL_PROPERTIES[CellType::Empty as usize].clone(),
        )),
        Box::new(RockCell::with_comm_props(
            CELL_PROPERTIES[CellType::Rock as usize].clone(),
        )),
        Box::new(WoodCell::with_comm_props(
            CELL_PROPERTIES[CellType::Wood as usize].clone(),
        )),
        Box::new(SandCell::with_comm_props(
            CELL_PROPERTIES[CellType::Sand as usize].clone(),
        )),
        Box::new(GunpowderCell::with_comm_props(
            CELL_PROPERTIES[CellType::Gunpowder as usize].clone(),
        )),
        Box::new(WaterCell::with_comm_props(
            CELL_PROPERTIES[CellType::Water as usize].clone(),
        )),
        Box::new(OilCell::with_comm_props(
            CELL_PROPERTIES[CellType::Oil as usize].clone(),
        )),
        Box::new(PropaneCell::with_comm_props(
            CELL_PROPERTIES[CellType::Propane as usize].clone(),
        )),
        Box::new(FireCell::with_comm_props(
            CELL_PROPERTIES[CellType::Fire as usize].clone(),
        )),
        Box::new(LavaCell::with_comm_props(
            CELL_PROPERTIES[CellType::Lava as usize].clone(),
        )),
        Box::new(AcidCell::with_comm_props(
            CELL_PROPERTIES[CellType::Acid as usize].clone(),
        )),
    ]
}
