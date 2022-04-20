use crate::hoarder_management::{IndexedFile, IndexedFileList};
use jwalk::Parallelism::{RayonDefaultPool, Serial};
use jwalk::WalkDir;
use std::cmp::Ordering;
use std::path::PathBuf;

/// Index all folders and files in a given folder and return a vector with all of them in decreasing order by size.
fn index_folder(path: &PathBuf, should_use_multiple_threads: bool) -> IndexedFileList {
    let path = path.clone();
    let parallelism_level = if should_use_multiple_threads {
        RayonDefaultPool
    } else {
        Serial
    };
    let indexed_files: IndexedFileList = WalkDir::new(&path)
        .parallelism(parallelism_level)
        .skip_hidden(false)
        .follow_links(false)
        .process_read_dir(|_depth, _path, _read_dir_state, children| {
            children.sort_by(|a, b| match (a, b) {
                (Ok(a), Ok(b)) => a
                    .metadata()
                    .unwrap()
                    .len()
                    .cmp(&b.metadata().unwrap().len())
                    .reverse(),
                (Ok(_), Err(_)) => Ordering::Less,
                (Err(_), Ok(_)) => Ordering::Greater,
                (Err(_), Err(_)) => Ordering::Equal,
            });
        })
        .into_iter()
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .map(|entry| {
            let path = entry.path();
            let metadata = entry.metadata().unwrap();
            let is_folder = metadata.is_dir();
            let size = metadata.len();
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            IndexedFile {
                name,
                path,
                size,
                is_folder,
            }
        })
        .collect::<Vec<_>>();
    return indexed_files;
}
