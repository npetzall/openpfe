use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use openpfe_llm::error::LlmError;
use openpfe_llm::types::{
    CompleteOptions, DownloadStatus, JobId, LlmFile, LlmService, LlmStatus, ModelEntry,
};
use openpfe_ui::api_router;
use tower::ServiceExt;

mod common;

#[test]
fn get_llm_config_defaults() {
    let (state, _dir) = common::test_state(MockLlmService::default());
    let app = api_router(state);

    let response = tokio_test(async {
        app.oneshot(
            Request::builder()
                .uri("/api/v1/llm/config")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response")
    });

    assert_eq!(response.status(), StatusCode::OK);
    let body = tokio_test(body_json(response));
    assert!(body.get("llm").is_some());
    assert!(body.get("catalog").is_some());
}

#[test]
fn put_llm_active() {
    let mock = MockLlmService::default();
    let (state, _dir) = common::test_state(mock.clone());
    let app = api_router(state);

    let response = tokio_test(async {
        app.oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/v1/llm/active")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"model":"test-model"}"#))
                .expect("request"),
        )
        .await
        .expect("response")
    });

    assert_eq!(response.status(), StatusCode::OK);
    let body = tokio_test(body_json(response));
    assert_eq!(body["model"], "test-model");
    assert_eq!(mock.active_model(), Some("test-model".into()));
}

#[test]
fn complete_returns_text() {
    let mock = MockLlmService::default();
    let (state, _dir) = common::test_state(mock);
    let app = api_router(state);

    let response = tokio_test(async {
        app.oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/llm/complete")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"prompt":"hello"}"#))
                .expect("request"),
        )
        .await
        .expect("response")
    });

    assert_eq!(response.status(), StatusCode::OK);
    let body = tokio_test(body_json(response));
    assert_eq!(body["text"], "mocked response");
}

#[test]
fn complete_busy() {
    let mock = MockLlmService::default().with_complete_error(LlmError::Busy);
    let (state, _dir) = common::test_state(mock);
    let app = api_router(state);

    let response = tokio_test(async {
        app.oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/llm/complete")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"prompt":"hello"}"#))
                .expect("request"),
        )
        .await
        .expect("response")
    });

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = tokio_test(body_json(response));
    assert_eq!(body["error"]["code"], "inference_busy");
}

#[test]
fn model_not_loaded() {
    let mock = MockLlmService::default()
        .with_complete_error(LlmError::engine("no model loaded"));
    let (state, _dir) = common::test_state(mock);
    let app = api_router(state);

    let response = tokio_test(async {
        app.oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/llm/complete")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"prompt":"hello"}"#))
                .expect("request"),
        )
        .await
        .expect("response")
    });

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = tokio_test(body_json(response));
    assert_eq!(body["error"]["code"], "model_not_loaded");
}

fn tokio_test<F: std::future::Future>(fut: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(fut)
}

async fn body_json(response: axum::response::Response) -> serde_json::Value {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    serde_json::from_slice(&bytes).expect("json")
}

#[derive(Clone, Default)]
struct MockLlmService {
    inner: Arc<MockLlmInner>,
}

#[derive(Clone, Copy, Default)]
enum MockCompleteMode {
    #[default]
    Ok,
    Busy,
    NotLoaded,
}

#[derive(Default)]
struct MockLlmInner {
    active_model: Mutex<Option<String>>,
    complete_mode: Mutex<MockCompleteMode>,
}

impl MockLlmService {
    fn with_complete_error(self, err: LlmError) -> Self {
        let mode = match err {
            LlmError::Busy => MockCompleteMode::Busy,
            _ => MockCompleteMode::NotLoaded,
        };
        *self.inner.complete_mode.lock().expect("lock") = mode;
        self
    }

    fn active_model(&self) -> Option<String> {
        self.inner.active_model.lock().expect("lock").clone()
    }
}

impl LlmService for MockLlmService {
    fn load_config(&self) -> openpfe_llm::Result<LlmFile> {
        Ok(LlmFile::default())
    }

    fn write_config(&self, _file: &LlmFile) -> openpfe_llm::Result<()> {
        Ok(())
    }

    fn list_models(&self) -> openpfe_llm::Result<Vec<ModelEntry>> {
        Ok(vec![])
    }

    fn start_download(&self, _id: &str) -> openpfe_llm::Result<JobId> {
        Ok(JobId::new_v4())
    }

    fn download_status(&self, _job_id: &JobId) -> openpfe_llm::Result<DownloadStatus> {
        Ok(DownloadStatus::Queued)
    }

    fn set_active_model(&self, id: &str) -> openpfe_llm::Result<()> {
        *self.inner.active_model.lock().expect("lock") = Some(id.to_string());
        Ok(())
    }

    fn reload_engine(&self) -> openpfe_llm::Result<()> {
        Ok(())
    }

    fn status(&self) -> LlmStatus {
        LlmStatus::default()
    }

    fn complete(&self, _prompt: &str, _opts: CompleteOptions) -> openpfe_llm::Result<String> {
        match *self.inner.complete_mode.lock().expect("lock") {
            MockCompleteMode::Ok => Ok("mocked response".into()),
            MockCompleteMode::Busy => Err(LlmError::Busy),
            MockCompleteMode::NotLoaded => Err(LlmError::engine("no model loaded")),
        }
    }
}
