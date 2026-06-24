use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::error::{LlmError, Result};
use crate::paths::{model_dir, models_root};
use crate::types::{CatalogEntry, LlmFile, ModelEntry, ModelManifest};

pub fn list_models(home: &Path, file: &LlmFile) -> Result<Vec<ModelEntry>> {
    let mut out = Vec::with_capacity(file.catalog.len());
    for entry in &file.catalog {
        out.push(model_entry(home, entry)?);
    }
    Ok(out)
}

pub fn model_entry(home: &Path, entry: &CatalogEntry) -> Result<ModelEntry> {
    let manifest = read_manifest_if_present(home, &entry.id)?;
    let installed = manifest.is_some() || path_catalog_exists(entry);
    Ok(ModelEntry {
        id: entry.id.clone(),
        filename: entry.filename.clone(),
        url: entry.url.clone(),
        sha256: entry.sha256.clone(),
        path: entry.path.clone(),
        installed,
        manifest,
    })
}

pub fn get_model_detail(home: &Path, file: &LlmFile, id: &str) -> Result<ModelEntry> {
    let entry = catalog_entry(file, id)?;
    model_entry(home, entry)
}

fn catalog_entry<'a>(file: &'a LlmFile, id: &str) -> Result<&'a CatalogEntry> {
    file.catalog
        .iter()
        .find(|e| e.id == id)
        .ok_or_else(|| LlmError::not_found(format!("catalog id {id}")))
}

fn path_catalog_exists(entry: &CatalogEntry) -> bool {
    entry
        .path
        .as_ref()
        .is_some_and(|p| Path::new(p).is_file())
}

pub fn read_manifest_if_present(home: &Path, id: &str) -> Result<Option<ModelManifest>> {
    let path = model_dir(home, id).join("manifest.json");
    if !path.is_file() {
        return Ok(None);
    }
    let bytes = fs::read(&path)?;
    let manifest: ModelManifest = serde_json::from_slice(&bytes)
        .map_err(|e| LlmError::config(format!("invalid manifest for {id}: {e}")))?;
    Ok(Some(manifest))
}

/// Resolved `.gguf` path for loading, verifying digest when manifest or catalog sha is known.
pub fn resolve_gguf_path(home: &Path, entry: &CatalogEntry) -> Result<PathBuf> {
    if let Some(path) = &entry.path {
        let path = PathBuf::from(path);
        if !path.is_file() {
            return Err(LlmError::not_found(format!(
                "catalog path not found: {}",
                path.display()
            )));
        }
        verify_file_sha256(&path, &entry.sha256)?;
        return Ok(path);
    }

    let dir = model_dir(home, &entry.id);
    let filename = entry.filename.as_deref().ok_or_else(|| {
        LlmError::config(format!("catalog entry {} missing filename", entry.id))
    })?;
    let gguf = dir.join(filename);
    if !gguf.is_file() {
        return Err(LlmError::not_found(format!(
            "model weights not installed: {}",
            entry.id
        )));
    }
    verify_file_sha256(&gguf, &entry.sha256)?;
    Ok(gguf)
}

pub fn is_installed(home: &Path, entry: &CatalogEntry) -> bool {
    resolve_gguf_path(home, entry).is_ok()
}

pub fn write_manifest(
    home: &Path,
    id: &str,
    filename: &str,
    sha256: &str,
    url: Option<&str>,
    installed_at: &str,
) -> Result<()> {
    let dir = model_dir(home, id);
    fs::create_dir_all(&dir)?;
    let manifest = ModelManifest {
        id: id.to_string(),
        filename: filename.to_string(),
        sha256: sha256.to_string(),
        url: url.map(str::to_string),
        installed_at: installed_at.to_string(),
    };
    let path = dir.join("manifest.json");
    let json = serde_json::to_vec_pretty(&manifest)
        .map_err(|e| LlmError::config(format!("serialize manifest: {e}")))?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, json)?;
    fs::rename(tmp, path)?;
    Ok(())
}

pub fn ensure_models_root(home: &Path) -> Result<()> {
    fs::create_dir_all(models_root(home))?;
    Ok(())
}

pub fn verify_file_sha256(path: &Path, expected_hex: &str) -> Result<()> {
    let expected = normalize_sha256_hex(expected_hex)?;
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    let digest = hex::encode(hasher.finalize());
    if digest != expected {
        return Err(LlmError::verify(format!(
            "sha256 mismatch for {}: expected {expected}, got {digest}",
            path.display()
        )));
    }
    Ok(())
}

fn normalize_sha256_hex(hex_str: &str) -> Result<String> {
    let s = hex_str.trim().to_ascii_lowercase();
    if s.len() != 64 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(LlmError::verify("sha256 must be 64 hex digits"));
    }
    Ok(s)
}

mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes
            .as_ref()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }
}
