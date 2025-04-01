pub mod download;

use std::path::{Path, PathBuf};

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};

use super::manifest::NodeVersion;

/// Node runtime
pub struct NodeRuntime {
    /// Version of the runtime
    version: NodeVersion,
    /// Path to the runtime directory
    path: PathBuf,
}

/// Searches within the provided directory for available runtimes
pub async fn discover_runtimes<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<NodeRuntime>> {
    let path = path.as_ref();

    if !path.exists() {
        bail!("runtime directory missing");
    }

    let mut paths = tokio::fs::read_dir(path)
        .await
        .context("failed to search runtimes directory")?;

    let runtimes = Vec::new();

    while let Some(entry) = paths
        .next_entry()
        .await
        .context("failed to get next directory entry")?
    {
        let metadata = entry
            .metadata()
            .await
            .context("failed to get entry metadata")?;

        // Skip non directories
        if !metadata.is_dir() {
            continue;
        }

        let entry_path = entry.path();
    }

    Ok(runtimes)
}

/// Install a Node package
pub async fn download_node_runtime<P: AsRef<Path>>(output: P) -> anyhow::Result<()> {
    todo!();
}
