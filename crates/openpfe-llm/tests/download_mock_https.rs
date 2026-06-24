mod common;

use std::time::Duration;

use common::{sha256_hex, spawn_http_server, wait_for, TestDirs};
use openpfe_llm::types::{CatalogEntry, LlmFile, LlmService, LlmSettings};
use openpfe_llm::LlamaLlmService;

#[test]
fn download_mock_https() {
    let _guard = common::llama_test_lock();
    let dirs = TestDirs::new();
    let payload = b"tiny-gguf-payload-for-test";
    let digest = sha256_hex(payload);
    let url = spawn_http_server(payload.to_vec());

    let file = LlmFile {
        llm: LlmSettings::default(),
        catalog: vec![CatalogEntry {
            id: "tiny".into(),
            filename: Some("tiny.gguf".into()),
            url: Some(url),
            sha256: digest.clone(),
            path: None,
        }],
    };
    openpfe_llm::config::write_config(&dirs.project, &file).expect("write config");

    let service =
        LlamaLlmService::new_with_paths(dirs.project.clone(), dirs.home.clone()).expect("service");
    let job = service.start_download("tiny").expect("start download");

    wait_for(
        || {
            matches!(
                service.download_status(&job),
                Ok(openpfe_llm::DownloadStatus::Complete)
            )
        },
        Duration::from_secs(10),
    );

    let models = service.list_models().expect("list");
    let entry = models.iter().find(|m| m.id == "tiny").expect("tiny entry");
    assert!(entry.installed);
    assert!(entry.manifest.is_some());
    assert_eq!(entry.manifest.as_ref().unwrap().sha256, digest);
}
