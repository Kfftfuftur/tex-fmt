//! Utilities for reading the command line arguments

use crate::logging::*;
use crate::regexes::*;
use clap::Parser;
use log::Level::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Command line arguments
#[allow(missing_docs)]
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(long, short, help = "Check formatting, do not modify files")]
    pub check: bool,
    #[arg(long, short, help = "Print to STDOUT, do not modify files")]
    pub print: bool,
    #[arg(long, short, help = "Keep lines, do not wrap")]
    pub keep: bool,
    #[arg(long, short, help = "Show info log messages")]
    pub verbose: bool,
    #[arg(long, short, help = "Hide warning messages")]
    pub quiet: bool,
    #[arg(long, short, help = "Show trace log messages")]
    pub trace: bool,
    #[arg(required = true)]
    pub files: Vec<PathBuf>,
}

impl Cli {
    /// Ensure the provided arguments are consistent
    pub fn resolve(&mut self) {
        if self.trace {
            self.verbose = true;
        }
    }

    #[cfg(test)]
    pub const fn new() -> Self {
        Self {
            check: false,
            print: false,
            keep: false,
            verbose: false,
            quiet: false,
            trace: false,
            files: Vec::new(),
        }
    }
}

/// Add a missing extension and read the file
pub fn read(file: &Path, logs: &mut Vec<Log>) -> Option<(PathBuf, String)> {
    // check if file has an accepted extension
    let has_ext = file
        .extension()
        .is_some_and(|extension| EXTENSIONS.iter().any(|e| extension == *e));
    // if no valid extension, try adding .tex
    let new_file = if has_ext {
        file.to_owned()
    } else {
        file.to_owned().with_extension("tex")
    };
    if let Ok(text) = fs::read_to_string(&new_file) {
        return Some((new_file, text));
    }
    if has_ext {
        record_file_log(logs, Error, file, "Could not open file.");
    } else {
        record_file_log(logs, Error, file, "File type invalid.");
    }
    None
}
