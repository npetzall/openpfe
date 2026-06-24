use std::path::{Path, PathBuf};

/// Project config: `./.openpfe/llm.json` relative to process cwd.
pub fn llm_json_path(project_root: &Path) -> PathBuf {
    project_root.join(".openpfe").join("llm.json")
}

/// Shared weights root: `$HOME/.openpfe/models/`.
pub fn models_root(home: &Path) -> PathBuf {
    home.join(".openpfe").join("models")
}

/// Per-model directory: `$HOME/.openpfe/models/<id>/`.
pub fn model_dir(home: &Path, id: &str) -> PathBuf {
    models_root(home).join(id)
}
