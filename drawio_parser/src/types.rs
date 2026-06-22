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
        println!("overlaps: {overlaps}");
        println!("{self:?}");
        println!("{other:?}");
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
    pub id: String,
    pub value: Option<String>,
    pub style: Option<String>,
    pub vertex_or_edge: VertexOrEdge,
    pub source: Option<String>,
    pub target: Option<String>,
    pub parent: Option<String>,
    pub geometry: Option<RectGeometry>,
    pub object: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct Board {
    pub main_cell_id: Option<String>,
    pub oneline_name: String,
    // id -> connector_id
    pub connectors_id: HashSet<String>,
    pub in_sections: HashSet<String>,
}

pub struct Section {
    pub id: String,
    pub name: String,
    pub geom: RectGeometry,
}

pub struct Connector {
    pub id: String,
    pub oneline_name: String,
    pub kind: Option<String>,
    pub board_id: String,
}

pub struct Harness {
    pub id: String,
    /// length in mm
    pub length: Option<u32>,
    pub kind: Option<String>,
}

pub struct Interconnect {
    pub id: String,
    pub oneline_name: String,
    pub in_sections: HashSet<String>,
}
