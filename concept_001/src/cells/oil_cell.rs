use crate::{
    cell::{CellCommonProperties, CellContext, CellLike, CellType},
    space::Space,
};

#[derive(Debug, Clone)]
pub struct OilCell {
    pub comm_props: CellCommonProperties,
}

impl OilCell {
    pub fn with_comm_props(cp: CellCommonProperties) -> Self {
        Self { comm_props: cp }
    }
}

fn saturate_sub(v: usize, a: usize, min: usize) -> usize {
    if v <= min + a {
        min
    } else {
        v - a
    }
}

fn saturate_add(v: usize, a: usize, max: usize) -> usize {
    if v >= max - a {
        max
    } else {
        v + a
    }
}

impl CellLike for OilCell {
    fn tick(&self, cctx: CellContext, space: &mut Space) {
        let (cy, cx) = (cctx.y, cctx.x);
        let ny = saturate_add(cy, 1, space.height - 1);
        let nindx = space.get_indx(ny, cx);
        if cy != space.height - 1 {
            let cindx = space.get_indx(cy, cx);
            if space.shadow_cells[nindx].cell_type == CellType::Empty {
                space.shadow_cells[nindx] = space.shadow_cells[cindx];
                space.shadow_cells[cindx].cell_type = CellType::Empty;
            } else {
                let (dy1, dx1) = (
                    saturate_add(cy, 1, space.height - 1),
                    saturate_add(cx, 1, space.width - 1),
                );
                let dindx1 = space.get_indx(dy1, dx1);
                let (dy2, dx2) = (
                    saturate_add(cy, 1, space.height - 1),
                    saturate_sub(cx, 1, 0),
                );
                let dindx2 = space.get_indx(dy2, dx2);
                let (dy3, dx3) = (
                    cy,
                    saturate_add(cx, 1, space.width - 1),
                );
                let dindx3 = space.get_indx(dy3, dx3);
                let (dy4, dx4) = (
                    cy,
                    saturate_sub(cx, 1, 0),
                );
                let dindx4 = space.get_indx(dy4, dx4);
                if space.shadow_cells[dindx1].cell_type == CellType::Empty {
                    space.shadow_cells[dindx1] = space.shadow_cells[cindx];
                    space.shadow_cells[cindx].cell_type = CellType::Empty;
                } else if space.shadow_cells[dindx2].cell_type == CellType::Empty {
                    space.shadow_cells[dindx2] = space.shadow_cells[cindx];
                    space.shadow_cells[cindx].cell_type = CellType::Empty;
                } else if space.shadow_cells[dindx3].cell_type == CellType::Empty {
                    space.shadow_cells[dindx3] = space.shadow_cells[cindx];
                    space.shadow_cells[cindx].cell_type = CellType::Empty;
                } else if space.shadow_cells[dindx4].cell_type == CellType::Empty {
                    space.shadow_cells[dindx4] = space.shadow_cells[cindx];
                    space.shadow_cells[cindx].cell_type = CellType::Empty;
                } else {
                    // Ячейка остается в прежнем состоянии
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cell::{Cell, CellCommonProperties, CellContext, CellLike, CellType}, space::Space};

    use super::OilCell;

    fn produce_oil_cell() -> OilCell {
        OilCell {
            comm_props: CellCommonProperties {
                cell_type: crate::cell::CellType::Oil,
                color: "#ffffff",
                name: "",
                density: 0.0,
                dissolvable: false,
                flammable: false,
                temp_coefficient: 0.0,
            }
        }
    }

    fn produce_empty_space() -> Space {
        let (width, height) = (5, 5);
        Space::with_witdh_height(width, height)
    }

    fn produce_cell_context() -> CellContext {
        CellContext {
            cell_info: Cell {
                cell_type: crate::cell::CellType::Oil,
                dx: 0.0,
                dy: 0.0,
                temp: 27,
            },
            x: 0,
            y: 0,
        }
    }

    fn print_cells(space: &Space) {
        for i in 0..space.height {
            for j in 0..space.width {
                let indx = space.get_indx(i, j);
                println!("space[{i},{j}] = {:?}", space.shadow_cells[indx].cell_type);
            }
        }
    }

    #[test]
    fn default_test() {
        let cell = produce_oil_cell();
        let mut space = produce_empty_space();
        let mut cctx = produce_cell_context();
        (cctx.y, cctx.x) = (4, 2);
        let indx = space.get_indx(4, 2);
        space.shadow_cells[indx] = cctx.cell_info.clone();

        cell.tick(cctx, &mut space);

        let new_indx = space.get_indx(4, 2);
        assert!(space.shadow_cells[new_indx].cell_type == CellType::Oil);
    }

    #[test]
    fn diagonal_test() {
        let cell = produce_oil_cell();
        let mut space = produce_empty_space();
        let mut cctx1 = produce_cell_context();
        (cctx1.y, cctx1.x) = (4, 2);
        let mut cctx2 = produce_cell_context();
        (cctx2.y, cctx2.x) = (3, 2);
        let indx1 = space.get_indx(4, 2);
        let indx2 = space.get_indx(3, 2);
        space.shadow_cells[indx1] = cctx1.cell_info.clone();
        space.shadow_cells[indx2] = cctx2.cell_info.clone();

        cell.tick(cctx2, &mut space);

        let new_indx = space.get_indx(4, 3);
        assert!(space.shadow_cells[new_indx].cell_type == CellType::Oil);
        assert!(space.shadow_cells[indx2].cell_type == CellType::Empty);
    }

    #[test]
    fn lowest_row_rule_test() {
        let cell = produce_oil_cell();
        let mut space = produce_empty_space();
        let mut cctx1 = produce_cell_context();
        (cctx1.y, cctx1.x) = (4, 2);
        let mut cctx2 = produce_cell_context();
        (cctx2.y, cctx2.x) = (4, 4);
        let indx1 = space.get_indx(4, 2);
        let indx2 = space.get_indx(4, 4);
        space.shadow_cells[indx1] = cctx1.cell_info.clone();
        space.shadow_cells[indx2] = cctx2.cell_info.clone();

        cell.tick(cctx1, &mut space);
        cell.tick(cctx2, &mut space);

        print_cells(&space);

        let new_indx = space.get_indx(4, 3);
        assert!(space.shadow_cells[new_indx].cell_type == CellType::Empty);
        assert!(space.shadow_cells[indx1].cell_type == CellType::Oil);
        assert!(space.shadow_cells[indx2].cell_type == CellType::Oil);
    }
}
