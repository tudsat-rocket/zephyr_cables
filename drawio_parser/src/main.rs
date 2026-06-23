use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

mod gather;
mod helpers;
mod types;

use helpers::*;
use types::*;

use crate::gather::{
    gather_all_actors, gather_all_sensors, gather_boards, gather_connectors, gather_harnesses,
    gather_interconnects, gather_sections,
};

fn attr_value(e: &BytesStart, key: &[u8]) -> Option<String> {
    e.attributes()
        .flatten()
        .find(|a| a.key.as_ref() == key)
        .and_then(|a| String::from_utf8(a.value.into_owned()).ok())
}

fn parse_geometry(e: &BytesStart) -> RectGeometry {
    fn parse_f64(value: Option<String>) -> Option<f64> {
        value.and_then(|v| v.parse::<f64>().ok())
    }
    RectGeometry {
        x: parse_f64(attr_value(e, b"x")).unwrap_or_default(), // .expect("mx geometry wrong: x"),
        y: parse_f64(attr_value(e, b"y")).unwrap_or_default(), // expect("mx geometry wrong: y"),
        width: parse_f64(attr_value(e, b"width")).expect("mx geometry wrong: width"),
        height: parse_f64(attr_value(e, b"height")).expect("mx geometry wrong: height"),
    }
}

/// If cell has not id tag, fallback_id will be used
fn parse_cell(e: &BytesStart, fallback_id: Option<u32>) -> Cell {
    let vertex = attr_value(e, b"vertex").as_deref() == Some("1");
    let edge = attr_value(e, b"edge").as_deref() == Some("1");

    let maybe_id: Option<u32> =
        attr_value(e, b"id").map(|id| id.parse().expect("cell id is not number"));
    let id: u32 = match (maybe_id, fallback_id) {
        (None, None) => panic!("no id for cell found"),
        (Some(id), None) => id,
        (None, Some(id)) => id,
        (Some(..), Some(..)) => panic!("fallback id and id provided when trying to parse cell"),
    };
    let value = attr_value(e, b"value");
    let oneline_name = value.clone().map(|v| to_nice_string(Some(v.as_ref())));

    Cell {
        id,
        value,
        oneline_name,
        style: attr_value(e, b"style"),
        vertex_or_edge: match (vertex, edge) {
            (true, _) => VertexOrEdge::Vertex,
            (_, true) => VertexOrEdge::Edge,
            _ => VertexOrEdge::Other,
        },
        source_id: attr_value(e, b"source").map(|id| id.parse().expect("source not a number")),
        target_id: attr_value(e, b"target").map(|id| id.parse().expect("target not a number")),
        parent_id: attr_value(e, b"parent").map(|id| id.parse().expect("parent not a number")),
        geometry: None,
        object: None,
    }
}
fn insert_unique_cell(map: &mut HashMap<u32, Cell>, id: u32, cell: Cell) {
    if map.contains_key(&id) {
        panic!("tried to insert cell with same id twice");
    }
    map.insert(id, cell);
}

pub fn parse_diagram_cells(xml_str: &str) -> HashMap<u32, Cell> {
    let mut reader = Reader::from_str(xml_str);
    let mut buf = Vec::new();

    let mut cells: HashMap<u32, Cell> = HashMap::new();
    let mut current_cell: Option<Cell> = None;
    let mut current_object: Option<HashMap<String, String>> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"object" => {
                if current_object.is_some() {
                    panic!("open object tag discovered, while other still open");
                }
                current_object = Some(
                    e.attributes()
                        .filter_map(|a| a.ok())
                        .map(|a| {
                            let key = String::from_utf8_lossy(a.key.as_ref()).into_owned();
                            let value = a.unescape_value().unwrap_or_default().into_owned();
                            (key, value)
                        })
                        .collect(),
                );
            }

            Ok(Event::Start(ref e)) if e.name().as_ref() == b"mxCell" => {
                if current_cell.is_some() {
                    panic!("open mxCell tag discovered, while other still open");
                }
                let mut cell = parse_cell(
                    e,
                    current_object.as_ref().and_then(|o| {
                        o.get("id")
                            .map(|id| id.parse().expect("obj id is not a number"))
                    }),
                );
                cell.object = current_object.clone();
                if current_object.is_some() {}
                current_cell = Some(cell);
            }

            Ok(Event::Empty(ref e)) if e.name().as_ref() == b"mxCell" => {
                let mut cell = parse_cell(
                    e,
                    current_object.as_ref().and_then(|o| {
                        o.get("id")
                            .map(|id| id.parse().expect("object id not a number"))
                    }),
                );
                cell.object = current_object.clone();
                if cell.id != 0 && cell.id != 1 {
                    panic!("empty mxCell other than id=\"0\" or id=\"1\"");
                }
                let id = cell.id.clone();
                insert_unique_cell(&mut cells, id, cell);
            }

            Ok(Event::Empty(ref e)) if e.name().as_ref() == b"mxGeometry" => {
                if let Some(cell) = current_cell.as_mut() {
                    if cell.vertex_or_edge == VertexOrEdge::Vertex {
                        cell.geometry = Some(parse_geometry(e));
                    }
                } else {
                    panic!("mxGeometry outside of cell");
                }
            }

            Ok(Event::End(ref e)) if e.name().as_ref() == b"mxCell" => {
                if let Some(cell) = current_cell.take() {
                    insert_unique_cell(&mut cells, cell.id, cell);
                }
                current_cell = None;
            }

            Ok(Event::End(ref e)) if e.name().as_ref() == b"object" => {
                current_object = None;
            }

            Ok(Event::Eof) => break,
            Err(e) => {
                panic!("XML Error: {}", e);
            }
            _ => {}
        }

        buf.clear();
    }

    cells
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let xml = std::fs::read_to_string(filename).unwrap_or_else(|e| {
        eprintln!("Error reading file '{}': {}", filename, e);
        std::process::exit(1);
    });

    let all_cells: HashMap<u32, Cell> = parse_diagram_cells(&xml);

    let all_sections = gather_sections(&all_cells);
    let all_boards = gather_boards(&all_cells, &all_sections);
    let all_connectors = gather_connectors(&all_boards, &all_cells);
    let all_interconnects = gather_interconnects(&all_cells, &all_sections);
    let all_actors = gather_all_actors(&all_cells, &all_sections);
    let all_sensors = gather_all_sensors(&all_cells, &all_sections);
    let all_harnesses = gather_harnesses(
        &all_cells,
        &all_connectors,
        &all_interconnects,
        &all_boards,
        &all_sensors,
        &all_actors,
    );

    println!("\n\n\n\n\n\n\n");

    // print for debugging

    for (board_id, board) in &all_boards {
        let mut connector_names: Vec<String> = Vec::new();
        for conn_id in &board.connectors_id {
            connector_names.push(
                all_connectors
                    .get(&conn_id.clone())
                    .unwrap()
                    .oneline_name
                    .clone(),
            );
        }
        let mut section_names: Vec<String> = Vec::new();
        for sect_id in &board.in_sections {
            section_names.push(all_sections.get(&sect_id.clone()).unwrap().name.clone());
        }
        println!(
            "B({:?}) S({:?}): connectors: {:?}",
            board.oneline_name, section_names, &connector_names
        );
    }
    let mut all_interconnects: HashMap<String, Harness> = HashMap::new();

    for (id, section) in all_sections {
        println!("s({})", section.name);
    }
    println!("\n\n");

    for (id, harness) in all_harnesses {
        println!(
            "h({}), kind: {:?} len: {:?}",
            harness.id, harness.kind, harness.length
        );
    }
}
