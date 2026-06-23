use serde::Serialize;
use std::collections::{HashMap, HashSet};

/// describes position and apperance of object (e.g. rectangle)
#[derive(Debug, Serialize, Clone)]
pub struct RectGeometry {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
impl RectGeometry {
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
    pub fn overlaps(&self, other: &RectGeometry) -> bool {
        let overlaps = self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y;
        // println!("overlaps: {overlaps}");
        // println!("{self:?}");
        // println!("{other:?}");
        overlaps
    }
}

/// Any object that has a human readable, human set name
trait Named {
    fn nice_name(&self) -> String;
}

/// Drawio Vertex: board, connector
/// Drawio Edge:
#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub enum VertexOrEdge {
    Vertex,
    Edge,
    Other,
}

/// xml element: mxCell
#[derive(Debug, Clone, Serialize)]
pub struct Cell {
    /// unique id of drawio element
    pub id: u32,
    /// html that renders to text visible in drawio
    pub value: Option<String>,
    pub oneline_name: Option<String>,
    pub style: Option<String>,
    pub vertex_or_edge: VertexOrEdge,
    /// edge connects to cell with source_id
    pub source_id: Option<u32>,
    /// edge connects to cell with target_id
    pub target_id: Option<u32>,
    pub parent_id: Option<u32>,
    pub geometry: Option<RectGeometry>,
    /// Attribues of the surrounding object tag e.g.  section_marker="recovery" id="2"
    pub object: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct Board {
    /// unique id of drawio element
    pub main_cell_id: Option<u32>,
    /// visible text without linebreaks
    pub oneline_name: String,
    // id -> connector_id
    pub connectors_id: HashSet<u32>,
    pub in_sections: HashSet<u32>,
}

pub struct Section {
    /// unique id of drawio element
    pub id: u32,
    pub name: String,
    pub geom: RectGeometry,
}

pub struct Connector {
    /// unique id of drawio element
    pub id: u32,
    /// visible text without linebreaks
    pub oneline_name: String,
    pub kind: Option<String>,
    pub board_id: u32,
}

pub struct Harness {
    /// unique id of drawio element
    pub id: u32,
    /// length in mm
    pub length: Option<u32>,
    /// harness_kind
    pub kind: Option<String>,
}

pub struct Interconnect {
    /// unique id of drawio element
    pub id: u32,
    /// visible text without linebreaks
    pub oneline_name: String,
    pub in_sections: HashSet<u32>,
}

pub struct Sensor {
    /// unique id of drawio element
    pub id: u32,
    /// visible text without linebreaks
    pub oneline_name: String,
    pub in_section: HashSet<u32>,
}

pub struct Actor {
    /// unique id of drawio element
    pub id: u32,
    /// visible text without linebreaks
    pub oneline_name: String,
    pub in_section: HashSet<u32>,
}
