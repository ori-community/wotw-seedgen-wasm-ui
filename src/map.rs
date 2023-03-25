use regex::Regex;
use wasm_bindgen::prelude::*;

use wotw_seedgen::settings::{Difficulty, UniverseSettings};
use wotw_seedgen::util::NodeKind;
use wotw_seedgen::util::Position as SeedgenPosition;
use wotw_seedgen::world::graph::Node as SeedgenNode;

use wasm_bindgen_helper_macros::*;

ts_enum! {
    #[wasm_bindgen]
    #[derive(Clone, Copy)]
    /// A general distinction about the `Connection`
    pub enum ConnectionType {
        /// Anchor-to-anchor `Connection`
        Branch,
        /// Anchor-to-pickup `Connection`
        Leaf,
    }
}

#[wasm_bindgen]
/// Set of `Node`s and `Connection`s connecting those `Node`s
pub struct Graph {
    nodes: __NodeList,
    connections: __ConnectionList,
}
#[wasm_bindgen]
impl Graph {
    #[wasm_bindgen(getter)]
    pub fn connections(&self) -> ConnectionArray {
        self.connections.clone().into_js_array()
    }
    #[wasm_bindgen(getter)]
    pub fn nodes(&self) -> NodeObject {
        self.nodes.clone().into_js_object("name")
    }
}

#[wasm_bindgen]
/// Returns a `Graph` based on the given logic files
///
/// `areas` should be in the syntax usually used by `areas.wotw`, `locations` should provide csv data as usually used by `loc_data.csv`
///
/// @throws {string} Throws if the input fails to parse
pub fn graph(areas: &str, locations: &str) -> Result<Graph, JsValue> {
    let states = ""; // As long as the state data doesn't track coordinates, it isn't useful for our purpose
    let mut settings = UniverseSettings::default();
    settings.world_settings[0].difficulty = Difficulty::Unsafe; // Ensure no paths are optimized away
    let logic = wotw_seedgen::logic::parse_logic(areas, locations, states, &settings, false)?;

    let positioned_nodes = logic
        .nodes
        .iter()
        .filter_map(|node| node.map_position().map(|position| (node, position)))
        .collect::<Vec<_>>();

    let nodes = nodes(&positioned_nodes);
    let connections = connections(&logic.nodes, &positioned_nodes);

    Ok(Graph { nodes, connections })
}
fn nodes(positioned_nodes: &[(&SeedgenNode, &SeedgenPosition)]) -> __NodeList {
    let nodes = positioned_nodes
        .iter()
        .map(|(node, position)| {
            let name = node.identifier().to_owned();
            let position = Vector2::from((*position).clone());
            Node { name, position }
        })
        .collect::<Vec<_>>();
    __NodeList::from(nodes)
}
fn connections(
    nodes: &[SeedgenNode],
    positioned_nodes: &[(&SeedgenNode, &SeedgenPosition)],
) -> __ConnectionList {
    let mut node_pairs = positioned_nodes
        .iter()
        .filter_map(|(node, _)| {
            if let SeedgenNode::Anchor(anchor) = node {
                Some(anchor)
            } else {
                None
            }
        }) // Anchors
        .flat_map(|anchor| {
            anchor
                .connections
                .iter()
                .map(|connection| &nodes[connection.to]) // Target nodes
                .filter(|node| node.position().is_some()) // Only positioned targets
                .map(|target| (anchor, target)) // Pairs of anchors and target nodes
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut connections = Vec::with_capacity(node_pairs.len());
    while let Some((start, end)) = node_pairs.pop() {
        let unidirectional =
            match node_pairs
                .iter()
                .enumerate()
                .find(|(_, (other_start, other_end))| {
                    start.index == other_end.index() && end.index() == other_start.index
                }) {
                Some((reverse_connection_index, _)) => {
                    node_pairs.remove(reverse_connection_index);
                    false
                }
                None => true,
            };
        let kind = match end.node_kind() {
            NodeKind::Anchor => ConnectionType::Branch,
            _ => ConnectionType::Leaf,
        };
        let start = start.identifier.clone();
        let end = end.identifier().to_owned();
        let connection = Connection {
            start,
            end,
            unidirectional,
            kind,
        };
        connections.push(connection);
    }

    __ConnectionList::from(connections)
}

#[wasm_bindgen(typescript_custom_section)]
const NODE_MAP: &'static str = "\
export type NodeMap = {
  [name: string]: Node
}";
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "NodeMap")]
    pub type NodeMap;
}

wrapper_map! {
    #[wasm_bindgen]
    #[derive(Clone)]
    pub struct __NodeList {
        inner: IntoIter<Node>,
    }
}

#[wasm_bindgen]
/// End point of a `Connection`
#[derive(Clone)]
pub struct Node {
    #[wasm_bindgen(getter_with_clone)]
    /// The name of this `Node`
    pub name: String,
    /// The map position of this `Node`, using in-game coordinates
    pub position: Vector2,
}
#[wasm_bindgen]
impl Node {
    #[wasm_bindgen]
    /// The location of this `Node` definition in the source file
    pub fn source_location(&self, areas: &str) -> Option<SourceLocation> {
        Regex::new(&format!("^anchor +{}", self.name))
            .unwrap()
            .find(areas)
            .map(|matched| SourceLocation::from_index(areas, matched.start()))
    }
}

wrapper_list! {
    #[wasm_bindgen]
    #[derive(Clone)]
    pub struct __ConnectionList {
        inner: IntoIter<Connection>,
    }
}

#[wasm_bindgen]
/// `Connection` between two `Node`s
#[derive(Clone)]
pub struct Connection {
    #[wasm_bindgen(getter_with_clone)]
    /// Name of the `Node` this `Connection` starts at
    pub start: String,
    #[wasm_bindgen(getter_with_clone)]
    /// Name of the `Node` this `Connection` ends at
    pub end: String,
    /// `true` if this `Connection` only leads from `start` to `end`, `false` if this connection goes in both directions
    pub unidirectional: bool,
    kind: ConnectionType,
}
#[wasm_bindgen]
impl Connection {
    #[wasm_bindgen(getter, js_name = "type")]
    /// The `ConnectionType` of this `Connection`
    pub fn kind(&self) -> ConnectionTypeEnum {
        self.kind.into_js_enum()
    }
    #[wasm_bindgen]
    /// The location of this `Connection` definition in the source file
    ///
    /// If this `Connection` is not `unidirectional`, setting `inverse` to `false` will return the definition from `start` to `end`,
    /// while setting `inverse` to `true` will return the definition from `end` to `start`
    pub fn source_location(&self, areas: &str, inverse: bool) -> Option<SourceLocation> {
        let (from, to) = if inverse {
            (&self.end, &self.start)
        } else {
            (&self.start, &self.end)
        };

        let connection_pattern = match self.kind {
            ConnectionType::Branch => format!("^ +conn +{to}"),
            ConnectionType::Leaf => format!("^ +(?:pickup|quest) +{to}"),
        };

        let index = Regex::new(&connection_pattern)
            .unwrap()
            .find_iter(areas)
            .find_map(|matched| {
                let anchor = areas[..matched.start()].rfind("\nanchor")?;
                if areas[anchor + 7..]
                    .trim_start_matches(' ')
                    .starts_with(from)
                {
                    Some(matched.start())
                } else {
                    None
                }
            })?;

        Some(SourceLocation::from_index(areas, index))
    }
}
#[wasm_bindgen]
/// line and character location inside a text file
pub struct SourceLocation {
    pub line: usize,
    pub char: usize,
}
impl SourceLocation {
    fn from_index(source: &str, index: usize) -> Self {
        let line = source[..index].bytes().filter(|b| *b == b'\n').count();
        let char = source[index..].find(|c| c != ' ').unwrap_or(0);

        Self { line, char }
    }
}

#[wasm_bindgen]
/// A point in two-dimensional space
#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
impl From<SeedgenPosition> for Vector2 {
    fn from(position: SeedgenPosition) -> Vector2 {
        Vector2 {
            x: position.x.into_inner(),
            y: position.y.into_inner(),
        }
    }
}
