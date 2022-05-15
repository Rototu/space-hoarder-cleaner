use std::{collections::HashMap, path::PathBuf};

/// A file or folder indexed by the application.
#[derive(Debug)]
pub struct IndexedFile {
    pub name: String,
    /// The path to the file or folder.
    pub path: PathBuf,
    /// The size of the file or folder in bytes.
    pub size: u64,
    /// Marker to idenitfy folder.
    pub is_folder: bool,
}

/// A list of files and folders indexed by the application.
pub type IndexedFileList = Vec<IndexedFile>;

/// Collection of indexed files grouped by their file type.
pub type ExtensionHashMap<'a> = HashMap<&'a String, &'a IndexedFileList>;
