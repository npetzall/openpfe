//! Run with a real tiny `.gguf` on disk:
//! `OPENPFE_LLM_GGUF=/path/to/model.gguf cargo test -p openpfe-llm --test inference_manual -- --ignored`

mod common;

use std::path::PathBuf;

use common::{sha256_hex, TestDirs};
use openpfe_llm::types::{CatalogEntry, CompleteOptions, LlmFile, LlmService, LlmSettings};
use openpfe_llm::{registry, LlamaLlmService};

#[test]
#[ignore = "requires local GGUF at OPENPFE_LLM_GGUF"]
fn inference_manual() {
    let _guard = common::llama_test_lock();
    let gguf_path = std::env::var("OPENPFE_LLM_GGUF").expect("OPENPFE_LLM_GGUF");
    let path = PathBuf::from(&gguf_path);
    let bytes = std::fs::read(&path).expect("read gguf");
    let digest = sha256_hex(&bytes);

    let dirs = TestDirs::new();

    let file = LlmFile {
        llm: LlmSettings {
            model: Some("manual".into()),
            n_ctx: 512,
            max_tokens: 16,
            ..Default::default()
        },
        catalog: vec![CatalogEntry {
            id: "manual".into(),
            filename: None,
            url: None,
            sha256: digest,
            path: Some(path.display().to_string()),
        }],
    };
    openpfe_llm::config::write_config(&dirs.project, &file).expect("write");

    let service =
        LlamaLlmService::new_with_paths(dirs.project, dirs.home).expect("service");
    assert!(service.status().loaded, "engine should load path catalog entry");

    let out = service
        .complete("Say hi in one word.", CompleteOptions::default())
        .expect("complete");
    assert!(!out.trim().is_empty());

    registry::verify_file_sha256(&path, &file.catalog[0].sha256).expect("sha256");
}
