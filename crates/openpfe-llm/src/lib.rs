//! `llm.json`, model registry, HTTPS downloads, and local inference.
//!
//! Product contract: [.dev/crates/openpfe-llm/specification.md](https://github.com/npetzall/openpfe/blob/main/.dev/crates/openpfe-llm/specification.md).

pub mod config;
pub mod download;
pub mod engine;
pub mod error;
pub mod paths;
pub mod registry;
pub mod service;
pub mod types;

pub use error::{LlmError, Result};
pub use service::LlamaLlmService;
pub use types::{
    CatalogEntry, CompleteOptions, DownloadStatus, JobId, LlmFile, LlmService, LlmSettings,
    LlmStatus, ModelEntry, ModelManifest,
};
