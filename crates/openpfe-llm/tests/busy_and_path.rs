mod common;

use std::thread;
use std::time::Duration;

use common::{sha256_hex, touch_file, TestDirs};
use openpfe_llm::error::LlmError;
use openpfe_llm::types::{CatalogEntry, CompleteOptions, LlmFile, LlmService, LlmSettings};
use openpfe_llm::LlamaLlmService;

#[test]
fn busy_rejects_second_complete() {
    let _guard = common::llama_test_lock();
    let dirs = TestDirs::new();
    openpfe_llm::config::write_config(&dirs.project, &LlmFile::default()).expect("write");

    let service = LlamaLlmService::new_for_test(
        dirs.project,
        dirs.home,
        Some(Duration::from_millis(300)),
    )
    .expect("service");

    let svc = std::sync::Arc::new(service);
    let a = std::sync::Arc::clone(&svc);
    let b = std::sync::Arc::clone(&svc);

    let t1 = thread::spawn(move || a.complete("hello", CompleteOptions::default()));
    thread::sleep(Duration::from_millis(30));
    let second = b.complete("world", CompleteOptions::default());
    assert!(matches!(second, Err(LlmError::Busy)));
    let _ = t1.join().expect("thread");
}

#[test]
fn path_catalog_skips_download() {
    let _guard = common::llama_test_lock();
    let dirs = TestDirs::new();
    let payload = b"path-model";
    let digest = sha256_hex(payload);
    let gguf = dirs.home.join("existing.gguf");
    touch_file(&gguf, payload);

    let file = LlmFile {
        llm: LlmSettings::default(),
        catalog: vec![CatalogEntry {
            id: "local".into(),
            filename: None,
            url: None,
            sha256: digest,
            path: Some(gguf.display().to_string()),
        }],
    };
    openpfe_llm::config::write_config(&dirs.project, &file).expect("write");

    let service =
        LlamaLlmService::new_with_paths(dirs.project, dirs.home).expect("service");
    let err = service.start_download("local").unwrap_err();
    assert!(matches!(err, LlmError::InvalidRequest(_)));

    let models = service.list_models().expect("list");
    let entry = models.iter().find(|m| m.id == "local").expect("local");
    assert!(entry.installed);
}
