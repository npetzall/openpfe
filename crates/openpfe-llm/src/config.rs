use std::fs;
use std::path::Path;

use crate::error::{LlmError, Result};
use crate::paths::llm_json_path;
use crate::types::{validate_catalog_entry, LlmFile};

pub fn load_config(project_root: &Path) -> Result<LlmFile> {
    let path = llm_json_path(project_root);
    if !path.exists() {
        return Ok(LlmFile::default());
    }
    let bytes = fs::read(&path)?;
    let file: LlmFile = serde_json::from_slice(&bytes)
        .map_err(|e| LlmError::config(format!("invalid llm.json: {e}")))?;
    for entry in &file.catalog {
        validate_catalog_entry(entry)?;
    }
    Ok(file)
}

pub fn write_config(project_root: &Path, file: &LlmFile) -> Result<()> {
    for entry in &file.catalog {
        validate_catalog_entry(entry)?;
    }
    let path = llm_json_path(project_root);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_vec_pretty(file)
        .map_err(|e| LlmError::config(format!("serialize llm.json: {e}")))?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, json)?;
    fs::rename(tmp, path)?;
    Ok(())
}
