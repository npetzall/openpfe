use std::path::Path;

use serde_json::Map as JsonMap;

use crate::error::Result;
use crate::types::{
    FindSimilarDraft, Node, NodeFilter, SearchHit, SimilarHit, SubgraphLimits, SubgraphResult,
};

/// Product graph API — hides Grafeo types from HTTP/MCP layers.
///
/// All methods are synchronous; HTTP layers should call via `spawn_blocking`.
pub trait GraphStore {
    /// Open or create the graph store directory (e.g. `./.openpfe/graph/store/`).
    ///
    /// The adapter opens `{store_dir}/store.grafeo` and creates `store_dir` when missing.
    fn open(store_dir: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;

    fn close(self) -> Result<()>;

    fn upsert_node(
        &mut self,
        node_type: &str,
        id: &str,
        properties: JsonMap<String, serde_json::Value>,
    ) -> Result<()>;

    fn get_node(&self, id: &str) -> Result<Option<Node>>;

    fn list_nodes(&self, filter: &NodeFilter) -> Result<Vec<Node>>;

    fn delete_node(&mut self, id: &str) -> Result<bool>;

    fn create_edge(
        &mut self,
        src_id: &str,
        dst_id: &str,
        edge_type: &str,
        properties: JsonMap<String, serde_json::Value>,
    ) -> Result<()>;

    fn delete_edge(&mut self, src_id: &str, dst_id: &str, edge_type: &str) -> Result<bool>;

    fn neighbors(
        &self,
        id: &str,
        edge_types: Option<&[&str]>,
        outgoing: bool,
    ) -> Result<Vec<String>>;

    fn subgraph(&self, cluster_id: &str, limits: SubgraphLimits) -> Result<SubgraphResult>;

    fn validate_acyclic_deps(&self) -> Result<Vec<Vec<String>>>;

    fn search_problems(&self, query: &str, k: usize) -> Result<Vec<SearchHit>>;

    fn find_similar(&self, draft: &FindSimilarDraft, k: usize) -> Result<Vec<SimilarHit>>;

    fn backup_full(&self, backup_dir: &Path) -> Result<()>;

    /// Rebuild BM25 indexes after bulk import (not needed for steady-state upserts).
    fn rebuild_text_indexes(&self) -> Result<()>;

    /// Rebuild HNSW index after bulk embedding backfill.
    fn rebuild_vector_index(&self) -> Result<()>;
}
