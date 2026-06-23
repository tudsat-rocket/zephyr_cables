use std::collections::{HashMap, HashSet};
use std::sync::mpsc::TrySendError;

use super::helpers::*;
use super::types::Board;
use super::types::*;

/// Interconnect is a single rectanle connecting harnesses
pub fn gather_interconnects(
    all_cells: &HashMap<u32, Cell>,
    all_sections: &HashMap<u32, Section>,
) -> HashMap<u32, Interconnect> {
    let mut all_interconnects: HashMap<u32, Interconnect> = HashMap::new();
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
            *cell_id,
            Interconnect {
                id: *cell_id,
                oneline_name,
                in_sections,
            },
        );
    }
    all_interconnects
}

pub fn gather_all_sensors(
    all_cells: &HashMap<u32, Cell>,
    all_sections: &HashMap<u32, Section>,
) -> HashMap<u32, Sensor> {
    let sensor_cells = gather_one_type_in_section("sensor", all_cells, all_sections);
    let mut sensors: HashMap<u32, Sensor> = HashMap::new();
    for (id, in_section) in sensor_cells {
        let oneline_name = all_cells
            .get(&id)
            .unwrap()
            .oneline_name
            .clone()
            .unwrap_or_else(|| "Unnamed".to_string());

        insert_unique(
            &mut sensors,
            id,
            Sensor {
                id,
                oneline_name,
                in_section,
            },
        );
    }
    sensors
}

pub fn gather_all_actors(
    all_cells: &HashMap<u32, Cell>,
    all_sections: &HashMap<u32, Section>,
) -> HashMap<u32, Actor> {
    let actor_cells = gather_one_type_in_section("actor", all_cells, all_sections);
    let mut actors: HashMap<u32, Actor> = HashMap::new();
    for (id, in_section) in actor_cells {
        let oneline_name = all_cells
            .get(&id)
            .unwrap()
            .oneline_name
            .clone()
            .unwrap_or_else(|| "Unnamed".to_string());

        insert_unique(
            &mut actors,
            id,
            Actor {
                id,
                oneline_name,
                in_section,
            },
        );
    }
    actors
}

pub fn gather_one_type_in_section(
    type_str: &str,
    all_cells: &HashMap<u32, Cell>,
    all_sections: &HashMap<u32, Section>,
) -> HashMap<u32, HashSet<u32>> {
    let mut found: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (cell_id, cell) in all_cells {
        let Some(ref obj) = cell.object else {
            continue;
        };
        let Some(type_n) = obj.get("type") else {
            continue;
        };
        if type_n != type_str {
            continue;
        };
        let in_sections = determine_sections(
            &cell
                .geometry
                .clone()
                .expect("cell labled interconnect does not have geometry"),
            all_sections,
        );
        found.insert(*cell_id, in_sections);
    }
    found
}

pub fn gather_connectors(
    all_boards: &HashMap<u32, Board>,
    all_cells: &HashMap<u32, Cell>,
) -> HashMap<u32, Connector> {
    let mut all_connectors: HashMap<u32, Connector> = HashMap::new();
    for (group_cell_id, board) in all_boards {
        // let name = all_cells
        //     .get(&board.main_cell_id.clone().unwrap())
        //     .unwrap()
        //     .value
        //     .clone();
        // let name = to_nice_string(name.as_deref());

        for connector_id in &board.connectors_id {
            let oneline_name = to_nice_string(
                all_cells
                    .get(connector_id)
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
                *connector_id,
                Connector {
                    id: *connector_id,
                    oneline_name,
                    board_id: board.main_cell_id.unwrap(),
                    kind,
                },
            );
        }
    }
    all_connectors
}

/// Gathers cells marked with section_marker="<name>"
pub fn gather_sections(all_cells: &HashMap<u32, Cell>) -> HashMap<u32, Section> {
    let mut all_sections: HashMap<u32, Section> = HashMap::new();
    for (cell_id, cell) in all_cells {
        let Some(name) = cell.object.as_ref().and_then(|o| o.get("section_marker")) else {
            continue;
        };
        let Some(ref geom) = cell.geometry else {
            panic!("section_marker cell has not geometry");
        };
        insert_unique(
            &mut all_sections,
            *cell_id,
            Section {
                id: *cell_id,
                name: name.to_string(),
                geom: geom.clone(),
            },
        );
    }
    all_sections
}

/// Gathers all harnesses
pub fn gather_harnesses(
    all_cells: &HashMap<u32, Cell>,
    all_connectors: &HashMap<u32, Connector>,
    all_interconnects: &HashMap<u32, Interconnect>,
    all_boards: &HashMap<u32, Board>,
    all_sensors: &HashMap<u32, Sensor>,
    all_actors: &HashMap<u32, Actor>,
) -> HashMap<u32, Harness> {
    let mut all_harnesses: HashMap<u32, Harness> = HashMap::new();
    for (cell_id, cell) in all_cells {
        if cell.vertex_or_edge != VertexOrEdge::Edge {
            continue;
        }
        if let (Some(source_id), Some(target_id)) = (cell.source_id, cell.target_id) {
            match (
                all_connectors.contains_key(&source_id)
                    || all_interconnects.contains_key(&source_id)
                    || all_sensors.contains_key(&source_id)
                    || all_actors.contains_key(&source_id)
                    || all_boards.contains_key(&source_id),
                all_connectors.contains_key(&target_id)
                    || all_interconnects.contains_key(&target_id)
                    || all_sensors.contains_key(&source_id)
                    || all_actors.contains_key(&source_id)
                    || all_boards.contains_key(&target_id),
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
                *cell_id,
                Harness {
                    id: *cell_id,
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
    all_cells: &HashMap<u32, Cell>,
    all_sections: &HashMap<u32, Section>,
) -> HashMap<u32, Board> {
    let mut all_boards: HashMap<u32, Board> = HashMap::new();
    let cells = all_cells;

    for (id, cell) in cells.iter() {
        if cell.style.as_ref().is_none_or(|style| style != "group") {
            continue;
        }
        let Some(ref obj) = cell.object else {
            println!("WARN: group style cell has no parent object");
            continue;
        };
        let Some(board_group) = obj.get("board_group") else {
            continue;
        };
        if board_group != "1" {
            continue;
        }
        // board detected, colection connector cells
        let mut connectors: HashSet<u32> = HashSet::new();
        for (cell_id, cell) in cells {
            if cell.parent_id.as_ref().is_some_and(|p_id| *p_id == *id) {
                connectors.insert(*cell_id);
            }
        }
        // Rectangle with largest area is the board, others are the connectors
        let mut max_area: f64 = 0.0;
        let mut max_id: u32 = *connectors.iter().next().expect("connectors empty");

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
                max_id = *connector_id;
            }
        }
        connectors.remove(&max_id);
        let oneline_name = to_nice_string(cells.get(&max_id).unwrap().value.as_deref());

        println!("name for board: {oneline_name:?}");
        let in_sections = determine_sections(
            cell.geometry
                .as_ref()
                .expect("board cell does not have geometry"),
            all_sections,
        );

        all_boards.insert(
            *id,
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
