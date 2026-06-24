use serde::{Deserialize, Serialize};
use serde_json::Map as JsonMap;

/// openpfe node (`id` + `type` + JSON properties).
///
/// `type` is the PFE node kind; in Grafeo it is stored as the node **label**, not duplicated
/// as a property unless callers add one.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(flatten)]
    pub properties: JsonMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default)]
pub struct NodeFilter {
    pub node_type: Option<String>,
    pub cluster_id: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
pub struct SubgraphLimits {
    pub max_depth: u32,
    pub max_nodes: usize,
}

impl Default for SubgraphLimits {
    fn default() -> Self {
        Self {
            max_depth: 3,
            max_nodes: 200,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub src_id: String,
    pub dst_id: String,
    pub edge_type: String,
    #[serde(flatten)]
    pub properties: JsonMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubgraphResult {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchHit {
    pub node_id: String,
    pub title: String,
    pub score: f64,
}

/// Draft text (and optional embedding) for `find_similar`.
#[derive(Debug, Clone)]
pub struct FindSimilarDraft {
    pub title: String,
    pub description: String,
    pub cluster_id: Option<String>,
    /// When set, enables Grafeo `hybrid_search` / `vector_search` (from `openpfe-llm`).
    pub embedding: Option<Vec<f32>>,
}

/// Ranked candidate from `find_similar` (S6+).
///
/// `match_kinds` values: `lexical`, `semantic`, `structural` — see
/// `.dev/crates/openpfe-graph/specification.md`.
#[derive(Debug, Clone, PartialEq)]
pub struct SimilarHit {
    pub node_id: String,
    pub title: String,
    pub score: f64,
    pub match_kinds: Vec<String>,
    pub snippet: Option<String>,
}
