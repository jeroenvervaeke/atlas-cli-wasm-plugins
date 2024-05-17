use anyhow::{Context, Result};
use std::path::Path;
use tokio::fs::read;
use wit_component::ComponentEncoder;

pub async fn adapt_wasm_bytes(wasm_path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let adapter_wasm_bytes = wasmcloud_component_adapters::WASI_PREVIEW1_REACTOR_COMPONENT_ADAPTER;
    let wasm_bytes = read(&wasm_path).await.with_context(|| {
        format!(
            "failed to read wasm file from path [{}]",
            wasm_path.as_ref().display()
        )
    })?;

    // Build a component encoder
    let mut encoder = ComponentEncoder::default()
        .validate(true)
        .module(&wasm_bytes)
        .with_context(|| {
            format!(
                "failed to encode wasm component @ [{}]",
                wasm_path.as_ref().display()
            )
        })?;

    // Adapt the module
    encoder = encoder
        .adapter("wasi_snapshot_preview1", adapter_wasm_bytes.as_ref())
        .context("failed to set adapter during encoding")?;

    // Return the encoded module bytes
    encoder
        .encode()
        .context("failed to serialize encoded component")
}
