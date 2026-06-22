use std::collections::{HashMap, HashSet};

use super::helpers::*;
use super::types::Board;
use super::types::*;

/// Interconnect is a single rectanle connecting harnesses
pub fn gather_interconnects(
    all_cells: &HashMap<String, Cell>,
    all_sections: &HashMap<String, Section>,
) -> HashMap<String, Interconnect> {
    let mut all_interconnects: HashMap<String, Interconnect> = HashMap::new();
    for (cell_id, cell) in all_cells {
        let Some(ref obj) = cell.object else {
            continue;
        };
        let Some(type_n) = obj.get("type") else {
            continue;
        };
        if type_n != "interconnect" {
            continue;
        };
        let oneline_name = to_nice_string(cell.value.as_deref());
        let in_sections = determine_sections(
            &cell
                .geometry
                .clone()
                .expect("cell labled interconnect does not have geometry"),
            all_sections,
        );
        all_interconnects.insert(
            cell_id.to_string(),
            Interconnect {
                id: cell_id.to_string(),
                oneline_name,
                in_sections,
            },
        );
    }
    all_interconnects
}

pub fn gather_connectors(
    all_boards: &HashMap<String, Board>,
    all_cells: &HashMap<String, Cell>,
) -> HashMap<String, Connector> {
    let mut all_connectors: HashMap<String, Connector> = HashMap::new();
    for (group_cell_id, board) in all_boards {
        let name = all_cells
            .get(&board.main_cell_id.clone().unwrap())
            .unwrap()
            .value
            .clone();
        let striped_name = to_nice_string(name.as_deref());

        for connector_id in &board.connectors_id {
            let oneline_name = to_nice_string(
                all_cells
                    .get(&connector_id.clone())
                    .unwrap()
                    .value
                    .clone()
                    .as_deref(),
            );
            let kind = all_cells
                .get(connector_id)
                .unwrap()
                .object
                .as_ref()
                .and_then(|o| o.get("connector_kind"))
                .cloned();
            insert_unique(
                &mut all_connectors,
                connector_id.clone(),
                Connector {
                    id: connector_id.to_string(),
                    oneline_name,
                    board_id: board.main_cell_id.clone().unwrap(),
                    kind,
                },
            );
        }
    }
    all_connectors
}

/// Gathers cells marked with section_marker="<name>"
pub fn gather_sections(all_cells: &HashMap<String, Cell>) -> HashMap<String, Section> {
    let mut all_sections: HashMap<String, Section> = HashMap::new();
    for (cell_id, cell) in all_cells {
        let Some(name) = cell.object.as_ref().and_then(|o| o.get("section_marker")) else {
            continue;
        };
        let Some(ref geom) = cell.geometry else {
            panic!("section_marker cell has not geometry");
        };
        insert_unique(
            &mut all_sections,
            cell_id.to_string(),
            Section {
                id: cell_id.to_string(),
                name: name.to_string(),
                geom: geom.clone(),
            },
        );
    }
    all_sections
}

/// Gathers all harnesses
pub fn gather_harnesses(
    all_cells: &HashMap<String, Cell>,
    all_connectors: &HashMap<String, Connector>,
    all_interconnects: &HashMap<String, Interconnect>,
) -> HashMap<String, Harness> {
    let mut all_harnesses: HashMap<String, Harness> = HashMap::new();
    for (cell_id, cell) in all_cells {
        if cell.vertex_or_edge != VertexOrEdge::Edge {
            continue;
        }
        if let (Some(source_id), Some(target_id)) = (cell.source.clone(), cell.target.clone()) {
            match (
                all_connectors.contains_key(&source_id)
                    || all_interconnects.contains_key(&source_id),
                all_connectors.contains_key(&target_id)
                    || all_interconnects.contains_key(&target_id),
            ) {
                (true, true) => (),
                (true, false) | (false, true) => {
                    // println!(
                    //     "Warning: sketch contains dangling cables that are not connected on both ends"
                    // );
                    continue;
                }
                (false, false) => continue,
            }
            let length = cell.object.as_ref().and_then(|o| {
                o.get("length").map(|l| {
                    l.parse::<u32>()
                        .expect("failed to parse length into number")
                })
            });
            let kind = cell.object.as_ref().and_then(|o| o.get("harness_kind"));
            insert_unique(
                &mut all_harnesses,
                cell_id.to_string(),
                Harness {
                    id: cell_id.to_string(),
                    length,
                    kind: kind.cloned(),
                },
            );

            // if let (Some(source_c), Some(target_c)) = (all_connectors.get(source_id), all_connectors.get(target_id)) { }
        }
    }
    all_harnesses
}

/// Gathers all boards from cells
pub fn gather_boards(
    all_cells: &HashMap<String, Cell>,
    all_sections: &HashMap<String, Section>,
) -> HashMap<String, Board> {
    let mut all_boards: HashMap<String, Board> = HashMap::new();
    let cells = all_cells;

    for (id, cell) in cells.iter() {
        if cell.style.as_ref().is_none_or(|style| style != "group") {
            continue;
        }
        let Some(ref obj) = cell.object else {
            continue;
        };
        let Some(board_group) = obj.get("board_group") else {
            continue;
        };
        let Some(id) = obj.get("id") else {
            println!("warning: object without id");
            continue;
        };
        if board_group != "1" {
            continue;
        }
        // board detected, colection connector cells
        let mut connectors: HashSet<String> = HashSet::new();
        for (cell_id, cell) in cells {
            if cell.parent.as_ref().is_some_and(|p_id| p_id == id) {
                connectors.insert(cell_id.clone());
            }
        }
        // Rectangle with largest area is the board, others are the connectors
        let mut max_area: f64 = 0.0;
        let mut max_id: String = connectors
            .iter()
            .next()
            .expect("connectors empty")
            .to_string();
        for connector_id in &connectors {
            let area: f64 = cells
                .get(connector_id)
                .unwrap()
                .geometry
                .clone()
                .map(|geom| geom.area())
                .unwrap_or_default();
            if area > max_area {
                max_area = area;
                max_id = connector_id.to_string();
            }
        }
        connectors.remove(&max_id.clone());
        let oneline_name = to_nice_string(cells.get(&max_id).unwrap().value.as_deref());

        println!("name: {oneline_name:?}");
        let in_sections = determine_sections(
            cell.geometry
                .as_ref()
                .expect("board cell does not have geometry"),
            all_sections,
        );

        all_boards.insert(
            id.to_string(),
            Board {
                main_cell_id: Some(max_id),
                connectors_id: connectors,
                oneline_name,
                in_sections,
            },
        );
    }
    all_boards
}
