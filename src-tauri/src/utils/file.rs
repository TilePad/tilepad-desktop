use std::{collections::VecDeque, path::Path};

/// Moves a directory from one path to another
///
/// This is safe for cross volume moving unlike [std::fs::rename]
#[allow(unused)]
pub async fn move_directory<Src: AsRef<Path>, Dst: AsRef<Path>>(
    src: Src,
    dest: Dst,
) -> anyhow::Result<()> {
    let src = src.as_ref();
    let dest = dest.as_ref();

    // Copy all contents
    copy_dir_all(src, dest).await?;

    // Remove old contents
    tokio::fs::remove_dir_all(src).await?;

    Ok(())
}

/// Copies all files and directories from one path to another
#[allow(unused)]
pub async fn copy_dir_all(src: &Path, dst: &Path) -> anyhow::Result<()> {
    // Create destination path
    tokio::fs::create_dir_all(dst).await?;

    // Create queue for copying
    let mut queue = VecDeque::new();
    queue.push_back((src.to_path_buf(), dst.to_path_buf()));

    while let Some((src_dir, dst_dir)) = queue.pop_front() {
        let mut entries = tokio::fs::read_dir(&src_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let src_path = entry.path();
            let dst_path = dst_dir.join(entry.file_name());
            let file_type = entry.file_type().await?;

            if file_type.is_dir() {
                tokio::fs::create_dir_all(&dst_path).await?;
                queue.push_back((src_path, dst_path));
            } else {
                tokio::fs::copy(src_path, dst_path).await?;
            }
        }
    }

    Ok(())
}
