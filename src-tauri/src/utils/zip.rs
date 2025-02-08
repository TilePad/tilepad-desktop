use std::path::{Path, PathBuf};

use anyhow::Context;
use async_zip::tokio::read::seek::ZipFileReader;
use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::BufReader,
};
use tokio_util::compat::FuturesAsyncReadCompatExt;

/// Extracts the provided zip file reader to the provided `out_dir`
pub async fn extract_zip(
    mut zip: ZipFileReader<BufReader<File>>,
    out_dir: &Path,
) -> anyhow::Result<()> {
    for index in 0..zip.file().entries().len() {
        let entry = zip
            .file()
            .entries()
            .get(index)
            .context("failed to get entry")?;

        let file_name = entry.filename().as_str().context("invalid file name")?;
        let path = out_dir.join(sanitize_file_path(file_name));
        let entry_is_dir = entry.dir().context("failed to detect entry is dir")?;

        if entry_is_dir {
            if !path.exists() {
                create_dir_all(&path)
                    .await
                    .context("failed to create extracted directory")?;
            }

            continue;
        }

        let mut entry_reader = zip
            .reader_without_entry(index)
            .await
            .context("failed to read ZipEntry")?
            .compat();

        let parent = path
            .parent()
            .context("file entry should have parent directories")?;

        if !parent.is_dir() {
            create_dir_all(parent)
                .await
                .context("failed to create parent directories")?;
        }

        let mut writer = OpenOptions::new()
            .write(true)
            .create_new(true)
            .truncate(true)
            .open(&path)
            .await
            .context("Failed to create extracted file")?;

        tokio::io::copy(&mut entry_reader, &mut writer)
            .await
            .context("failed to copy to extracted file")?;
    }

    Ok(())
}

/// Returns a relative path without reserved names, redundant separators, ".", or "..".
fn sanitize_file_path(path: &str) -> PathBuf {
    // Replaces backwards slashes
    path.replace('\\', "/")
        // Sanitizes each component
        .split('/')
        .map(sanitize_filename::sanitize)
        .collect()
}
