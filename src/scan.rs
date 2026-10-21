use std::cmp::Reverse;
use std::fs;
use std::path::Path;

use anyhow::Context;

pub struct Entry {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}

/// Lists the immediate children of `root` with their total sizes, largest first.
pub fn scan(root: &Path) -> anyhow::Result<Vec<Entry>> {
    let children = fs::read_dir(root).with_context(|| format!("can't read {}", root.display()))?;

    let mut entries: Vec<Entry> = children
        .filter_map(Result::ok)
        .filter_map(|child| {
            // DirEntry::metadata doesn't follow symlinks, so we never count anything twice
            // or loop forever on a link that points back up the tree.
            let meta = child.metadata().ok()?;
            let size = if meta.is_dir() { dir_size(&child.path()) } else { meta.len() };
            Some(Entry {
                name: child.file_name().to_string_lossy().into_owned(),
                size,
                is_dir: meta.is_dir(),
            })
        })
        .collect();

    entries.sort_by_key(|e| Reverse(e.size));
    Ok(entries)
}

/// Total size of everything under `path`. Entries we can't read are skipped.
fn dir_size(path: &Path) -> u64 {
    let Ok(children) = fs::read_dir(path) else {
        return 0;
    };

    children
        .filter_map(Result::ok)
        .map(|child| match child.metadata() {
            Ok(meta) if meta.is_dir() => dir_size(&child.path()),
            Ok(meta) => meta.len(),
            Err(_) => 0,
        })
        .sum()
}
