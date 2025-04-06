use crate::utils::{file::move_directory, zip::extract_zip};

use crate::plugin::manifest::{Arch, BinaryNodeVersion};
use anyhow::{bail, ensure, Context};
use async_zip::tokio::read::seek::ZipFileReader;
use serde::Deserialize;
use std::path::Path;
use tempfile::{env::temp_dir, tempfile};
use tokio::{
    fs::create_dir_all,
    io::{AsyncWriteExt, BufReader},
};

const NODE_DIST_BASE_URL: &str = "https://nodejs.org/dist";

#[derive(Deserialize)]
pub struct NodeDist {
    pub version: node_semver::Version,
    pub files: Vec<String>,
}

/// Request the list of available node versions from the official repository
pub async fn get_node_versions(client: &reqwest::Client) -> anyhow::Result<Vec<NodeDist>> {
    let res = client
        .get("https://nodejs.org/dist/index.json")
        .send()
        .await?;

    let res = res
        .error_for_status()
        .context("response error when requesting download")?;

    let result: Vec<NodeDist> = res.json().await?;
    Ok(result)
}

/// Get a node download URL for the windows platform
///
/// https://nodejs.org/dist/v22.13.1/node-v22.13.1-win-x64.zip
#[cfg(windows)]
fn node_download_url(version: &str, arch: Arch) -> String {
    format!(
        "{base_url}/v{version}/node-v{version}-win-{arch}.zip",
        base_url = NODE_DIST_BASE_URL,
        version = version,
        arch = arch
    )
}

/// https://nodejs.org/dist/v22.13.1/node-v22.13.1-linux-x64.tar.xz
/// https://nodejs.org/dist/v22.13.1/node-v22.13.1-darwin-x64.tar.gz
#[cfg(not(windows))]
fn node_download_url(version: &str, arch: Arch) -> String {
    todo!("platform unsupported")
}

/// Downloads the requested node version
pub async fn download_node<P: AsRef<Path>>(
    client: &reqwest::Client,
    path: P,
    version: node_semver::Version,
    arch: Arch,
) -> anyhow::Result<()> {
    let path = path.as_ref();

    if path.exists() {
        ensure!(path.is_dir(), "node output path is a file")
    } else {
        create_dir_all(path).await?;
    }

    let version = version.to_string();
    let url = node_download_url(&version, arch);

    let res = client
        .get(url)
        .send()
        .await
        .context("failed to reqeust download url")?;

    let mut res = res
        .error_for_status()
        .context("response error when requesting download")?;

    let tmp_zip_file = tempfile().context("failed to get temp file for download")?;
    let mut tmp_zip_file = tokio::fs::File::from_std(tmp_zip_file);

    while let Some(chunk) = res.chunk().await? {
        tmp_zip_file
            .write_all(&chunk)
            .await
            .context("failed to write chunk")?;
    }

    tmp_zip_file
        .flush()
        .await
        .context("failed to flush download to zip")?;

    let reader = BufReader::new(tmp_zip_file);
    let zip = ZipFileReader::with_tokio(reader)
        .await
        .context("failed to create zip reader")?;

    let temp_folder_name = format!("tilepad-node-{version}-{arch}");

    let temp_dir = temp_dir();
    let temp_dir = temp_dir.join(temp_folder_name);

    extract_zip(zip, &temp_dir)
        .await
        .context("failed to extract runtime zip")?;

    let install_folder = tokio::fs::read_dir(&temp_dir)
        .await?
        .next_entry()
        .await
        .context("failed to determine install folder")?
        .context("missing install folder from zip")?;

    let install_folder = install_folder.path();

    move_directory(&install_folder, &path).await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use node_semver::Version;

    use crate::plugin::{
        manifest::Arch,
        node::{download_node, BinaryNodeVersion},
    };

    #[tokio::test]
    async fn test_download_latest() {
        let client = reqwest::Client::new();
        let path = Path::new("runtimes/22.13.1");
        download_node(&client, path, Version::new(22, 13, 1), Arch::default())
            .await
            .unwrap();
    }
}
