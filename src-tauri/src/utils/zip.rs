use std::path::{Path, PathBuf};

use anyhow::Context;
use async_zip::tokio::read::seek::ZipFileReader;
use tokio::{
    fs::{OpenOptions, create_dir_all},
    io::{AsyncBufRead, AsyncSeek},
};
use tokio_util::compat::FuturesAsyncReadCompatExt;

/// Create a zip reader from the tokio readable type
pub async fn create_zip_reader<R>(reader: R) -> anyhow::Result<ZipFileReader<R>>
where
    R: AsyncBufRead + AsyncSeek + Unpin,
{
    ZipFileReader::with_tokio(reader)
        .await
        .context("failed to create zip reader")
}

/// Extracts the provided zip file reader to the provided `out_dir`
pub async fn extract_zip<R>(mut zip: ZipFileReader<R>, out_dir: &Path) -> anyhow::Result<()>
where
    R: AsyncBufRead + AsyncSeek + Unpin,
{
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

/// Attempts to extract the contents of a file named `file_name` from the provided
/// `zip` zip file, returns the bytes of the file if one was found
pub async fn extract_zip_file<R>(
    mut zip: ZipFileReader<R>,
    file_name: &str,
) -> anyhow::Result<Option<Vec<u8>>>
where
    R: AsyncBufRead + AsyncSeek + Unpin,
{
    // Find the file entry index of the file
    let file_index = zip.file().entries().iter().position(|entry| {
        entry
            .filename()
            .as_str()
            .is_ok_and(|value| value == file_name)
    });

    let file_index = match file_index {
        Some(value) => value,
        None => return Ok(None),
    };

    // Create a reader for the manifest file
    let mut file_reader = zip
        .reader_without_entry(file_index)
        .await
        .context("failed to read file from zip")?
        .compat();

    // Read the manifest file
    let mut data = Vec::new();
    tokio::io::copy(&mut file_reader, &mut data)
        .await
        .context("failed to read manifest file")?;

    Ok(Some(data))
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
