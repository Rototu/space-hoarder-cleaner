use std::ffi::OsString;

pub enum FileType {
    File,
    Folder,
}

pub struct FileMetadata {
    pub name: OsString,
    pub size: u128,
    pub descendants: Option<u64>,
    pub percentage: f64, // 1.0 is 100% (0.5 is 50%, etc.)
    pub file_type: FileType,
}
