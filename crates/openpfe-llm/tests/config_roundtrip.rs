mod common;

use common::TestDirs;
use openpfe_llm::config::{load_config, write_config};
use openpfe_llm::types::{CatalogEntry, LlmFile, LlmSettings};

#[test]
fn config_defaults_when_missing() {
    let dirs = TestDirs::new();
    let loaded = load_config(&dirs.project).expect("load defaults");
    assert_eq!(loaded, LlmFile::default());

    let file = LlmFile {
        llm: LlmSettings {
            model: Some("demo".into()),
            n_ctx: 2048,
            ..Default::default()
        },
        catalog: vec![CatalogEntry {
            id: "demo".into(),
            filename: Some("demo.gguf".into()),
            url: Some("https://example.com/demo.gguf".into()),
            sha256: "0".repeat(64),
            path: None,
        }],
    };
    write_config(&dirs.project, &file).expect("write config");
    let again = load_config(&dirs.project).expect("reload");
    assert_eq!(again, file);
}
