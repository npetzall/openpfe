use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde_json::Value;

use crate::error::ClientError;
use crate::ports::{EchoInfo, ProjectControl, ServerSpawn};

type McpHandler = dyn Fn(Value) -> Result<Value, ClientError> + Send + Sync;

/// In-memory [`ProjectControl`] for unit tests and pre-005 wiring.
#[derive(Clone, Default)]
pub struct MockProjectControl {
    inner: Arc<MockProjectControlInner>,
}

#[derive(Default)]
struct MockProjectControlInner {
    /// Successful echo URL when the server is considered up.
    http_base_url: Mutex<String>,
    /// Remaining echo failures before success (exercises lock-held wait).
    echo_failures_remaining: Mutex<usize>,
    /// When false, echo always fails with [`ClientError::Echo`].
    server_up: Mutex<bool>,
    mcp_handler: Mutex<Option<Box<McpHandler>>>,
}

impl MockProjectControl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_url(url: impl Into<String>) -> Self {
        let mock = Self::new();
        *mock.inner.http_base_url.lock().unwrap() = url.into();
        *mock.inner.server_up.lock().unwrap() = true;
        mock
    }

    pub fn set_server_up(&self, up: bool) {
        *self.inner.server_up.lock().unwrap() = up;
    }

    pub fn set_http_base_url(&self, url: impl Into<String>) {
        *self.inner.http_base_url.lock().unwrap() = url.into();
    }

    pub fn set_echo_failures_remaining(&self, count: usize) {
        *self.inner.echo_failures_remaining.lock().unwrap() = count;
    }

    pub fn set_mcp_handler<F>(&self, handler: F)
    where
        F: Fn(Value) -> Result<Value, ClientError> + Send + Sync + 'static,
    {
        *self.inner.mcp_handler.lock().unwrap() = Some(Box::new(handler));
    }
}

#[async_trait]
impl ProjectControl for MockProjectControl {
    async fn echo(&self) -> Result<EchoInfo, ClientError> {
        if !*self.inner.server_up.lock().unwrap() {
            return Err(ClientError::Echo("server not up (mock)".into()));
        }
        let mut remaining = self.inner.echo_failures_remaining.lock().unwrap();
        if *remaining > 0 {
            *remaining -= 1;
            return Err(ClientError::Echo("transient echo failure (mock)".into()));
        }
        let url = self.inner.http_base_url.lock().unwrap().clone();
        if url.is_empty() {
            return Err(ClientError::Echo(
                "no http_base_url configured (mock)".into(),
            ));
        }
        Ok(EchoInfo { http_base_url: url })
    }

    async fn shutdown(&self) -> Result<(), ClientError> {
        *self.inner.server_up.lock().unwrap() = false;
        Ok(())
    }

    async fn send_mcp(&self, payload: Value) -> Result<Value, ClientError> {
        let handler = self.inner.mcp_handler.lock().unwrap();
        match handler.as_ref() {
            Some(h) => h(payload),
            None => Ok(payload),
        }
    }
}

/// Counting [`ServerSpawn`] mock.
#[derive(Clone)]
pub struct MockServerSpawn {
    pub spawn_count: Arc<AtomicUsize>,
    on_spawn: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl std::fmt::Debug for MockServerSpawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MockServerSpawn")
            .field("spawns", &self.spawns())
            .finish()
    }
}

impl Default for MockServerSpawn {
    fn default() -> Self {
        Self::new()
    }
}

impl MockServerSpawn {
    pub fn new() -> Self {
        Self {
            spawn_count: Arc::new(AtomicUsize::new(0)),
            on_spawn: None,
        }
    }

    /// After each spawn, run `hook` (e.g. mark mock server up).
    pub fn with_on_spawn(hook: impl Fn() + Send + Sync + 'static) -> Self {
        Self {
            spawn_count: Arc::new(AtomicUsize::new(0)),
            on_spawn: Some(Arc::new(hook)),
        }
    }

    /// Spawn hook that marks `control` as running (pre-005 main wiring).
    pub fn bringing_server_up(control: MockProjectControl) -> Self {
        Self::with_on_spawn(move || control.set_server_up(true))
    }

    pub fn spawns(&self) -> usize {
        self.spawn_count.load(Ordering::SeqCst)
    }
}

impl ServerSpawn for MockServerSpawn {
    fn spawn_detached_server(&self) -> Result<(), ClientError> {
        self.spawn_count.fetch_add(1, Ordering::SeqCst);
        if let Some(hook) = &self.on_spawn {
            hook();
        }
        Ok(())
    }
}
