//! Embedded problem graph for openpfe ([Grafeo](https://github.com/GrafeoDB/grafeo) engine).
//!
//! Product schema and API: [.dev/crates/openpfe-graph/specification.md](../../.dev/crates/openpfe-graph/specification.md).
//! Grafeo mapping: [schema](schema) and [.dev/crates/openpfe-graph/grafeo/specification.md](../../.dev/crates/openpfe-graph/grafeo/specification.md).

pub mod error;
pub mod graph_store;
pub mod grafeo_store;
pub mod schema;
pub mod types;

pub use error::{GraphError, Result};
pub use graph_store::GraphStore;
pub use grafeo_store::GrafeoGraphStore;
pub use types::{
    Edge, FindSimilarDraft, Node, NodeFilter, SearchHit, SimilarHit, SubgraphLimits,
    SubgraphResult,
};
