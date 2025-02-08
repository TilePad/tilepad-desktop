use crate::utils::{file::move_directory, zip::extract_zip};

use super::{arch::PlatformArch, NodeVersion};
use anyhow::{bail, Context};
use async_zip::tokio::read::seek::ZipFileReader;
use std::path::Path;
use tempfile::{env::temp_dir, tempfile};
use tokio::{
    fs::create_dir_all,
    io::{AsyncWriteExt, BufReader},
};

const NODE_DIST_BASE_URL: &str = "https://nodejs.org/dist";

/// Get a node download URL for the windows platform
///
/// https://nodejs.org/dist/v22.13.1/node-v22.13.1-win-x64.zip
/// https://nodejs.org/dist/v22.13.1/node-v22.13.1-linux-x64.tar.xz
/// https://nodejs.org/dist/v22.13.1/node-v22.13.1-darwin-x64.tar.gz
#[cfg(windows)]
fn node_download_url(version: &str, arch: PlatformArch) -> String {
    format!(
        "{base_url}/v{version}/node-v{version}-win-{arch}.zip",
        base_url = NODE_DIST_BASE_URL,
        version = version,
        arch = arch
    )
}

/// Downloads the requested node version
pub async fn download_node<P: AsRef<Path>>(
    client: &reqwest::Client,
    path: P,
    version: NodeVersion,
    arch: PlatformArch,
) -> anyhow::Result<()> {
    let path = path.as_ref();
    if path.exists() && !path.is_dir() {
        bail!("node output path is a file")
    }

    create_dir_all(path).await?;

    let version = version.0.to_string();
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

    dbg!(&install_folder);

    move_directory(&install_folder, &path).await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use node_semver::Version;

    use crate::plugin::runtime::{arch::PlatformArch, download::download_node, NodeVersion};

    #[tokio::test]
    async fn test_download_latest() {
        let client = reqwest::Client::new();
        let path = Path::new("runtimes/22.13.1");
        download_node(
            &client,
            path,
            NodeVersion(Version::new(22, 13, 1)),
            PlatformArch::default(),
        )
        .await
        .unwrap();
    }
}
