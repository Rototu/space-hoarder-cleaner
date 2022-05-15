use crate::hoarder_management::{IndexedFile, IndexedFileList};
use fs_extra::dir::get_size;
use jwalk::Parallelism::{RayonDefaultPool, Serial};
use jwalk::WalkDir;
use std::cmp::Ordering;
use std::path::PathBuf;

/// Index all folders and files in a given folder and return a vector with all of them in decreasing order by size.
pub fn index_folder(path: &PathBuf, should_use_multiple_threads: bool) -> IndexedFileList {
    let path = path.clone();
    let parallelism_level = if should_use_multiple_threads {
        RayonDefaultPool
    } else {
        Serial
    };
    let mut indexed_files: IndexedFileList = WalkDir::new(&path)
        .parallelism(parallelism_level)
        .skip_hidden(false)
        .follow_links(false)
        .into_iter()
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .map(|entry| {
            let path = entry.path();
            let metadata = entry.metadata().unwrap();
            let is_folder = metadata.is_dir();
            let size = get_size(&path).unwrap();
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            IndexedFile {
                name,
                path,
                size,
                is_folder,
            }
        })
        .collect::<Vec<_>>();
    indexed_files.sort_by(|file1, file2| file1.size.cmp(&file2.size).reverse());
    return indexed_files;
}
