//! Utilities for writing formatted files

use crate::fs;
use crate::logging::*;
use crate::parse::*;
use log::Level::Error;
use std::path;
use std::path::Path;

/// Write a formatted file to disk
fn write_file(file: &Path, text: &str) {
    let filepath = path::Path::new(&file).canonicalize().unwrap();
    fs::write(filepath, text).expect("Could not write the file");
}

/// Handle the newly formatted file
pub fn process_output(
    args: &Cli,
    file: &Path,
    text: &str,
    new_text: &str,
    exit_code: i32,
    logs: &mut Vec<Log>,
) -> i32 {
    let mut new_exit_code = exit_code;
    if args.print {
        println!("{}", &new_text);
    } else if args.check && text != new_text {
        record_file_log(logs, Error, file, "Incorrect formatting.");
        new_exit_code = 1;
    } else if text != new_text {
        write_file(file, new_text);
    }
    new_exit_code
}
