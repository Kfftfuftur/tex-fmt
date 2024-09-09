use std::path::Path;
use std::path::PathBuf;

use crate::format_file;
use crate::fs;
use crate::logging::*;
use crate::Cli;
use colored::Colorize;
use similar::{ChangeTag, TextDiff};

fn test_file(source_file: &Path, target_file: &Path) -> bool {
    let args = Cli::new();
    let mut logs = Vec::<Log>::new();
    dbg!(source_file, target_file);
    let source_text = fs::read_to_string(source_file).unwrap();
    let target_text = fs::read_to_string(target_file).unwrap();
    let fmt_source_text =
        format_file(&source_text, source_file, &args, &mut logs);

    if fmt_source_text != target_text {
        println!(
            "{} {} -> {}",
            "fail".red().bold(),
            source_file.to_string_lossy().yellow().bold(),
            target_file.to_string_lossy().bold()
        );
        let diff = TextDiff::from_lines(&fmt_source_text, &target_text);
        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => print!(
                    "{} {}",
                    format!("@ {}:", change.old_index().unwrap()).blue().bold(),
                    format!("- {}", change).red().bold(),
                ),
                ChangeTag::Insert => print!(
                    "{} {}",
                    format!("@ {}:", change.new_index().unwrap()).blue().bold(),
                    format!("+ {}", change).green().bold(),
                ),
                ChangeTag::Equal => {}
            };
        }
    }

    fmt_source_text == target_text
}

fn read_files_from_dir(dir: &Path) -> Vec<PathBuf> {
    dir.read_dir().unwrap().map(|f| f.unwrap().path()).collect()
}

#[test]
fn test_source() {
    let source_dir = PathBuf::from("./tests/source/");
    let target_dir = PathBuf::from("./tests/target/");

    let source_files = read_files_from_dir(&source_dir);
    for source_file in source_files {
        let mut target_file = target_dir.clone();
        target_file.push(source_file.strip_prefix(&source_dir).unwrap());

        assert!(
            test_file(&source_file, &target_file),
            "File: {:?} failed",
            target_file
        );
    }
}

#[test]
fn test_target() {
    let target_dir = PathBuf::from("./tests/target/");

    let target_files = read_files_from_dir(&target_dir);
    for target_file in target_files {
        assert!(
            test_file(&target_file, &target_file),
            "File: {:?} failed",
            target_file
        );
    }
}

#[test]
#[ignore]
fn test_short() {
    let files = vec![
        //"brackets.tex",
        //"cam-thesis.cls",
        //"comments.tex",
        //"cv.tex",
        //"document.tex",
        //"environment_lines.tex",
        //"heavy_wrap.tex",
        //"higher_categories_thesis.bib",
        //"higher_categories_thesis.tex",
        //"ignore.tex",
        //"lists.tex",
        //"masters_dissertation.tex",
        //"ociamthesis.cls",
        //"phd_dissertation.tex",
        //"phd_dissertation_refs.bib",
        //"puthesis.cls",
        //"quiver.sty",
        //"readme.tex",
        "short_document.tex",
        //"tikz_network.sty",
        //"unicode.tex",
        //"verbatim.tex",
        //"wgu-cv.cls",
        //"wrap.tex",
    ];
    let mut fail = false;
    for file in files {
        if !test_file(
            &PathBuf::from(&format!("tests/source/{file}")),
            &PathBuf::from(&format!("tests/target/{file}")),
        ) {
            fail = true;
        }
    }
    assert!(!fail, "Some tests failed");
}
