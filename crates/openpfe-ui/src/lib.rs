//! HTTP API for humans — Web UI and TUI.
//!
//! Product contract: [.dev/crates/openpfe-ui/specification.md](https://github.com/npetzall/openpfe/blob/main/.dev/crates/openpfe-ui/specification.md).

mod error;
mod router;
mod routes;
mod state;

pub use error::ApiError;
pub use router::api_router;
pub use state::AppState;
