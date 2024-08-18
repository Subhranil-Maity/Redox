use std::fs::Metadata;
use std::path::Path;
use tokio::fs::{canonicalize, metadata, read_dir, File, OpenOptions};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::schema::fs_info::ItemInfo;
use crate::utils::errors::{MyError, MyResult};

pub async fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path).await?;
    let mut s = String::new();
    match f.read_to_string(&mut s).await {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub async fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path).await?;

    f.write_all(s.as_bytes()).await
}

pub async fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn rm(path: &Path) -> io::Result<()> {
    match tokio::fs::remove_file(path).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn mv(from: &Path, to: &Path) -> io::Result<()> {
    match tokio::fs::rename(from, to).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn cp(from: &Path, to: &Path) -> io::Result<()> {
    let mut f = File::open(from).await?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;
    let mut f = File::create(to).await?;
    f.write_all(&buffer).await
}

pub async fn ls(path: &Path) -> io::Result<Vec<String>> {
    let mut entries = read_dir(path).await?;
    let mut data = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        data.push(entry.path().to_str().unwrap().to_string());
    }
    Ok(data)
}

pub async fn get_info(path: &str) -> MyResult<ItemInfo> {
    let p_path = Path::new(path);
    let _metadata = get_metadata(p_path).await?;
    let info = get_item_info(p_path).await?;
    Ok(info)
}

pub fn is_path_valid(p: &Path) -> bool {
    // let path_obj = Path::new(p);
    if !(p.is_absolute() && p.has_root()) {
        return true;
    }
    false
}

pub async fn get_metadata(p: &Path) ->  MyResult<Metadata> {
    match metadata(p).await {
        Ok(metadata) => {
            Ok(metadata)
        }
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => Err(MyError::FileOrFolderNotFound),
            io::ErrorKind::PermissionDenied => Err(MyError::AccessDeniedBySystem),
            _ => Err(MyError::UNKNOWN),
        },
    }
}

pub async fn get_file_info(path: &Path) -> MyResult<ItemInfo> {
    Ok(ItemInfo::new(
        path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        get_cleaned_canonical_path(path).await,
        get_metadata(path).await?.len(),
        path.is_symlink(),
        path.is_dir()
    ))
}
fn get_folder_name(path: &Path) -> Option<&str> {
    // let path = Path::new(folder_path);
    path.file_name().and_then(|name| name.to_str())
}

pub async fn get_cleaned_canonical_path(path: &Path) -> String {
    let canonical_path = canonicalize(path).await
        .unwrap()
        .to_string_lossy()
        .to_string();
    // Remove the \\?\ prefix from the canonical path
    if canonical_path.starts_with("\\\\?\\") {
        canonical_path[4..].to_string()
    } else {
        canonical_path
    }
}
pub async fn get_folder_info(path: &Path) -> ItemInfo {
    ItemInfo::new(
        get_folder_name(path).unwrap_or_default().to_string(),
        get_cleaned_canonical_path(path).await,
        0,
        path.is_symlink(),
        path.is_dir()
    )
}

pub async fn get_item_info(path: &Path) -> MyResult<ItemInfo> {
    if path.is_file() {
        Ok(get_file_info(path).await?)
    } else if path.is_dir() {
        return Ok(get_folder_info(path).await);
    }else {
        return Ok(ItemInfo::default());
    }
}