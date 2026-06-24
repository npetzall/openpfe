use std::num::NonZeroU32;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::AddBos;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::sampling::LlamaSampler;

use crate::error::{LlmError, Result};

struct LoadedModel {
    backend: LlamaBackend,
    model: LlamaModel,
    n_ctx: u32,
    n_threads: u32,
}

pub struct InferenceEngine {
    loaded: Mutex<Option<LoadedModel>>,
    busy: AtomicBool,
}

impl InferenceEngine {
    pub fn new() -> Result<Self> {
        Ok(Self {
            loaded: Mutex::new(None),
            busy: AtomicBool::new(false),
        })
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded.lock().expect("engine lock").is_some()
    }

    pub fn try_acquire_busy(&self) -> bool {
        self.busy
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }

    pub fn release_busy(&self) {
        self.busy.store(false, Ordering::SeqCst);
    }

    pub fn is_busy(&self) -> bool {
        self.busy.load(Ordering::SeqCst)
    }

    pub fn unload(&self) {
        let mut guard = self.loaded.lock().expect("engine lock");
        *guard = None;
    }

    pub fn load(&self, path: &Path, n_ctx: u32, n_threads: u32) -> Result<()> {
        let backend = LlamaBackend::init().map_err(|e| LlmError::engine(format!("backend: {e}")))?;
        let model = LlamaModel::load_from_file(&backend, path, &LlamaModelParams::default())
            .map_err(|e| LlmError::engine(format!("load model: {e}")))?;
        let mut guard = self.loaded.lock().expect("engine lock");
        *guard = Some(LoadedModel {
            backend,
            model,
            n_ctx,
            n_threads,
        });
        Ok(())
    }

    pub fn complete(&self, prompt: &str, temperature: f32, max_tokens: u32) -> Result<String> {
        let guard = self.loaded.lock().expect("engine lock");
        let loaded = guard
            .as_ref()
            .ok_or_else(|| LlmError::engine("no model loaded"))?;

        let n_ctx = NonZeroU32::new(loaded.n_ctx.max(512))
            .ok_or_else(|| LlmError::engine("n_ctx must be > 0"))?;
        let mut ctx_params = LlamaContextParams::default().with_n_ctx(Some(n_ctx));
        if loaded.n_threads > 0 {
            let threads = i32::try_from(loaded.n_threads).unwrap_or(i32::MAX);
            ctx_params = ctx_params.with_n_threads(threads);
            ctx_params = ctx_params.with_n_threads_batch(threads);
        }

        let mut ctx = loaded
            .model
            .new_context(&loaded.backend, ctx_params)
            .map_err(|e| LlmError::engine(format!("create context: {e}")))?;

        let tokens = loaded
            .model
            .str_to_token(prompt, AddBos::Always)
            .map_err(|e| LlmError::engine(format!("tokenize: {e}")))?;

        if tokens.is_empty() {
            return Ok(String::new());
        }

        let max_tokens = max_tokens.max(1) as i32;
        let mut batch = LlamaBatch::new(512, 1);
        let last = (tokens.len() - 1) as i32;
        for (i, token) in (0_i32..).zip(tokens.iter().copied()) {
            batch
                .add(token, i, &[0], i == last)
                .map_err(|e| LlmError::engine(format!("batch add: {e}")))?;
        }
        ctx.decode(&mut batch)
            .map_err(|e| LlmError::engine(format!("decode prompt: {e}")))?;

        let mut sampler = if temperature <= 0.0 {
            LlamaSampler::chain_simple([LlamaSampler::greedy()])
        } else {
            LlamaSampler::chain_simple([
                LlamaSampler::temp(temperature),
                LlamaSampler::dist(1234),
            ])
        };

        let mut out = String::new();
        let mut decoder = encoding_rs::UTF_8.new_decoder();
        let mut n_cur = batch.n_tokens();
        let mut generated = 0_i32;

        while generated < max_tokens {
            let token = sampler.sample(&ctx, batch.n_tokens() - 1);
            sampler.accept(token);
            if loaded.model.is_eog_token(token) {
                break;
            }
            let piece = loaded
                .model
                .token_to_piece(token, &mut decoder, true, None)
                .map_err(|e| LlmError::engine(format!("token to piece: {e}")))?;
            out.push_str(&piece);

            batch.clear();
            batch
                .add(token, n_cur, &[0], true)
                .map_err(|e| LlmError::engine(format!("batch add: {e}")))?;
            n_cur += 1;
            ctx.decode(&mut batch)
                .map_err(|e| LlmError::engine(format!("decode: {e}")))?;
            generated += 1;
        }

        Ok(out)
    }
}

impl Default for InferenceEngine {
    fn default() -> Self {
        Self::new().expect("llama backend init")
    }
}
