//! openpfe ↔ Grafeo mapping.
//!
//! Normative product schema: [specification.md](../../../.dev/crates/openpfe-graph/specification.md).
//! Narrative: [grafeo/specification.md](../../../.dev/crates/openpfe-graph/grafeo/specification.md).

/// PFE node `type` → Grafeo **node label** (same string).
pub mod label {
    pub const PROBLEM: &str = "problem";
    pub const CLUSTER: &str = "cluster";
    pub const COMPONENT: &str = "component";
    pub const CONTRACT: &str = "contract";
}

/// PFE edge `type` → Grafeo **relationship name** (same string).
pub mod rel {
    pub const DEPENDS_ON: &str = "depends_on";
    pub const MEMBER_OF: &str = "member_of";
    pub const INTERFACES: &str = "interfaces";
}

/// Node / edge **properties** stored on Grafeo entities (JSON-compatible `Value`s).
///
/// The stable openpfe id is **not** the Grafeo internal `NodeId`; it lives in [`PROP_ID`].
pub mod prop {
    /// Canonical UUID string (openpfe node id). Required on every node.
    pub const ID: &str = "id";
    pub const TITLE: &str = "title";
    pub const DESCRIPTION: &str = "description";
    pub const STATUS: &str = "status";
    /// Denormalized filter helper; must agree with `member_of` edges.
    pub const CLUSTER_ID: &str = "cluster_id";
    /// Optional `f32` embedding from `openpfe-llm` for vector / hybrid search.
    pub const EMBEDDING: &str = "embedding";

    pub const CONTRACT_BODY: &str = "contract_body";
    pub const VERSION: &str = "version";
    pub const CONSUMER_ID: &str = "consumer_id";
    pub const PROVIDER_ID: &str = "provider_id";
}

/// Default embedding width when creating the vector index (override when known from `openpfe-llm`).
pub const DEFAULT_EMBEDDING_DIMENSIONS: usize = 384;

/// Minimum BM25 score to keep a lexical hit (filters tiny-corpus noise).
pub const MIN_BM25_SCORE: f64 = 0.01;

/// Scale factor applied to mean Jaccard structural scores in `find_similar`.
pub const STRUCTURAL_SCORE_SCALE: f64 = 4.0;

/// Edge types traversed by bounded `subgraph` (MCP context shield).
pub const SUBGRAPH_EDGE_TYPES: &[&str] = &[rel::DEPENDS_ON, rel::MEMBER_OF, rel::INTERFACES];
