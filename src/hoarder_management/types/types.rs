use std::{collections::HashMap, path::PathBuf};

/// A file or folder indexed by the application.
pub struct IndexedFile {
    name: String,
    /// The path to the file or folder.
    path: PathBuf,
    /// The size of the file or folder in bytes.
    size: u64,
    /// Marker to idenitfy folder.
    is_folder: bool,
}

/// A list of files and folders indexed by the application.
pub type IndexedFileList = Vec<IndexedFile>;

/// Collection of indexed files grouped by their file type.
pub type ExtensionHashMap<'a> = HashMap<&'a String, &'a IndexedFileList>;
