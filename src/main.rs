mod app;
mod files;
mod hoarder_management;

use anyhow;
use clap::Parser;
use crossterm::event::Event as BackEvent;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::path::PathBuf;
use std::process;
use std::{env, io};
use tui::backend::Backend;
use tui::backend::CrosstermBackend;

#[cfg(not(test))]
const SHOULD_SHOW_LOADING_ANIMATION: bool = true;
#[cfg(test)]
const SHOULD_SHOW_LOADING_ANIMATION: bool = false;
#[cfg(not(test))]
const SHOULD_HANDLE_WIN_CHANGE: bool = true;
#[cfg(test)]
const SHOULD_HANDLE_WIN_CHANGE: bool = false;
#[cfg(not(test))]
const SHOULD_SCAN_HD_FILES_IN_MULTIPLE_THREADS: bool = true;
#[cfg(test)]
const SHOULD_SCAN_HD_FILES_IN_MULTIPLE_THREADS: bool = false;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Folder path to scan for space hoarders
    #[clap(short, long, parse(from_os_str), value_name = "PATH")]
    path: Option<PathBuf>,
}

fn main() {
    if let Err(err) = try_main() {
        println!("Error: {}", err);
        process::exit(2);
    }
}

fn get_stdout() -> io::Result<io::Stdout> {
    Ok(io::stdout())
}

fn try_main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match get_stdout() {
        Ok(stdout) => {
            enable_raw_mode()?;
            let terminal_backend = CrosstermBackend::new(stdout);
            // let terminal_events = TerminalEvents {};
            let folder = match cli.path {
                Some(folder) => folder,
                None => env::current_dir()?,
            };
            if !folder.as_path().is_dir() {
                anyhow::bail!("Folder '{}' does not exist", folder.to_string_lossy())
            }
            // start(
            //     terminal_backend,
            //     Box::new(terminal_events),
            //     folder,
            //     opts.apparent_size,
            //     opts.disable_delete_confirmation,
            // );
        }
        Err(_) => anyhow::bail!("Failed to get stdout: are you trying to pipe 'shc'?"),
    }
    disable_raw_mode()?;
    Ok(())
}

pub fn start<B>(
    terminal_backend: B,
    terminal_events: Box<dyn Iterator<Item = BackEvent> + Send>,
    path: PathBuf,
    show_apparent_size: bool,
    disable_delete_confirmation: bool,
) where
    B: Backend + Send + 'static,
{
}
