use crate::files::FileToDelete;
use tui::backend::Backend;

#[derive(Clone)]
pub enum UiMode {
    Loading,
    Normal,
    ScreenTooSmall,
    DeleteFile(FileToDelete),
    ErrorMessage(String),
    Exiting { app_loaded: bool },
    WarningMessage(FileToDelete),
}

pub struct App<B>
where
    B: Backend,
{
    pub is_running: bool,
    pub is_loaded: bool,
    pub ui_mode: UiMode,
}
