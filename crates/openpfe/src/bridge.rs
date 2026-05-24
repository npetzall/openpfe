use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::error::ClientError;
use crate::ports::ProjectControl;

/// Stdio line-delimited JSON-RPC ↔ IPC MCP bridge (FR-3; logs on stderr).
pub async fn run_stdio_bridge(
    control: &dyn ProjectControl,
    verbose: bool,
) -> Result<(), ClientError> {
    let mut lines = BufReader::new(tokio::io::stdin()).lines();
    let mut stdout = tokio::io::stdout();
    while let Some(line) = lines
        .next_line()
        .await
        .map_err(|e| ClientError::Bridge(e.to_string()))?
    {
        if line.trim().is_empty() {
            continue;
        }
        let payload: Value = serde_json::from_str(&line)
            .map_err(|e| ClientError::Bridge(format!("invalid json: {e}")))?;
        if verbose {
            eprintln!("openpfe: mcp forwarding request");
        }
        let response = control.send_mcp(payload).await?;
        let out = serde_json::to_string(&response)
            .map_err(|e| ClientError::Bridge(format!("encode response: {e}")))?;
        stdout
            .write_all(out.as_bytes())
            .await
            .map_err(|e| ClientError::Bridge(e.to_string()))?;
        stdout
            .write_all(b"\n")
            .await
            .map_err(|e| ClientError::Bridge(e.to_string()))?;
        stdout
            .flush()
            .await
            .map_err(|e| ClientError::Bridge(e.to_string()))?;
    }
    Ok(())
}
