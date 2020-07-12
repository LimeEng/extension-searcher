use std::path::Path;
use walkdir::WalkDir;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg};

const CLI_SUB_CMD_SEARCH: &str = "search";
const CLI_SUB_CMD_SEARCH_ARG_DIR: &str = "dir";
const CLI_SUB_CMD_SEARCH_ARG_EXTENSIONS: &str = "extensions";

#[derive(Debug)]
pub enum CliError {
    IoError(std::io::Error),
    ValidationError(String),
}

impl From<std::io::Error> for CliError {
    fn from(io_error: std::io::Error) -> Self {
        CliError::IoError(io_error)
    }
}

fn main() -> Result<(), CliError> {
    let matches = App::new("extension-searcher")
        // .version(crate_version!())
        .long_version(crate_version!())
        .version_short("v")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            App::new(CLI_SUB_CMD_SEARCH)
                .about("Recursively searches the specified directory after files, with the specified extensions")
                .arg(
                    Arg::with_name(CLI_SUB_CMD_SEARCH_ARG_DIR)
                        .help("The root directory")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name(CLI_SUB_CMD_SEARCH_ARG_EXTENSIONS)
                        .help("The list of extensions that will be searched after")
                        .index(2)
                        .multiple(true)
                        .required(true),
                )
        )
        .get_matches();
    match matches.subcommand() {
        (CLI_SUB_CMD_SEARCH, Some(matches)) => {
            let root_dir = matches.value_of(CLI_SUB_CMD_SEARCH_ARG_DIR).unwrap();
            let extensions: Vec<_> = matches
                .values_of(CLI_SUB_CMD_SEARCH_ARG_EXTENSIONS)
                .unwrap()
                .collect();
            search_directory(root_dir.to_string(), |path| {
                let extension = path.extension().and_then(|s| s.to_str());
                if let Some(ext) = extension {
                    if extensions
                        .iter()
                        .any(|&item| item.to_lowercase() == ext.to_owned().to_lowercase())
                    {
                        return true;
                    }
                }
                return false;
            });
            Ok(())
        }
        _ => Err(CliError::ValidationError(String::from(
            "Invalid subcommand",
        ))),
    }
}

fn search_directory<F: Fn(&Path) -> bool>(dir: String, should_print: F) {
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if should_print(&entry.path()) {
            println!("{}", entry.path().display())
        }
    }
}
