use std::{fs, path::PathBuf};

use crate::pbar::{get_progress_bar, print_final_report};
use crate::sanitizers;
use altbinutils::{
    app::{Application, ApplicationError, ApplicationResult},
    paths::{Entry, Paths},
    sanitizer::Sanitizer,
    CommonExitCodes, AFTER_HELP_LICENSE_TEXT,
};
use clap::{load_yaml, App};
use log::{debug, LevelFilter};

// Copyright 2021 Eray Erdin
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub struct Rment {
    paths: Paths,
}

impl Rment {
    pub fn new() -> ApplicationResult<Self> {
        debug!("Initializing rment...");
        let paths = match Paths::new("rment") {
            Ok(p) => p,
            Err(e) => {
                return Err(e);
            }
        };

        debug!("Initializing logger...");
        let logger = if cfg!(debug_assertions) {
            fern::Dispatch::new().chain(std::io::stdout())
        } else {
            let log_path = match paths.get_entry(Entry::Cache("rment.log")) {
                Ok(p) => p,
                Err(e) => {
                    return Err(e);
                }
            };

            let log_file = match fern::log_file(log_path) {
                Ok(f) => f,
                Err(e) => {
                    return Err(ApplicationError::InitError {
                        exit_code: CommonExitCodes::LogFailure as i32,
                        message: format!("An error occured while getting the log file. {}", e),
                    })
                }
            };

            fern::Dispatch::new().chain(log_file)
        };

        match logger
            .level(LevelFilter::Trace)
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{target}][{level}] {msg}",
                    target = record.target(),
                    level = record.level(),
                    msg = message
                ))
            })
            .apply()
        {
            Ok(_) => {}
            Err(e) => {
                return Err(ApplicationError::InitError {
                    exit_code: CommonExitCodes::LogFailure as i32,
                    message: format!("An error occured while initializing logger. {}", e),
                })
            }
        };

        Ok(Self { paths })
    }
}

impl Rment {
    fn root(&self, paths: Vec<PathBuf>) -> ApplicationResult<()> {
        debug!("Running rment...");

        debug!("Initializing counters...");
        let total = paths.len() as u64;
        let mut successful_file_count = 0u64;
        let mut successful_dir_count = 0u64;
        let mut failed_file_count = 0u64;
        let mut failed_dir_count = 0u64;
        let mut absent_count = 0u64;

        let pbar = get_progress_bar(total);
        // TODO multithreading
        for path in paths {
            pbar.set_message(match path.file_name() {
                Some(fname) => match fname.to_str() {
                    Some(s) => s,
                    None => "?",
                },
                None => "?",
            });
            // TODO add wildcard support, wildmatch crate
            match path.exists() {
                true => {
                    match path.is_file() {
                        true => {
                            match fs::remove_file(path.clone()) {
                                Ok(_) => {
                                    successful_file_count += 1;
                                }
                                Err(e) => {
                                    // TODO conf, fail on continue?
                                    failed_file_count += 1;
                                    pbar.println(format!(
                                        "Failed to remove file {}: {}",
                                        path.to_string_lossy(),
                                        e
                                    ));
                                }
                            }
                        }
                        false => match fs::remove_dir_all(path.clone()) {
                            Ok(_) => {
                                successful_dir_count += 1;
                            }
                            Err(e) => {
                                failed_dir_count += 1;
                                pbar.println(format!(
                                    "Failed to remove directory {}: {}",
                                    path.to_string_lossy(),
                                    e
                                ));
                            }
                        },
                    }
                }
                false => {
                    // TODO conf, fail on continue?
                    absent_count += 1;
                    pbar.println(format!("Path does not exist: {}", path.to_string_lossy()));
                }
            }
            pbar.inc(1);
        }
        pbar.set_message("OK");
        // TODO add skipped count
        print_final_report(
            &pbar,
            successful_file_count,
            successful_dir_count,
            failed_file_count,
            failed_dir_count,
            absent_count,
            0,
        );
        pbar.finish();

        Ok(())
    }
}

impl Application for Rment {
    fn run(&mut self) -> ApplicationResult<()> {
        debug!("Initializing rment...");
        let metadata = load_yaml!("../metadata.yaml");
        let app = App::from_yaml(metadata)
            .version(env!("CARGO_PKG_VERSION"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .after_help(AFTER_HELP_LICENSE_TEXT);

        debug!("Parsing matches...");
        let matches = app.get_matches();
        match matches.subcommand() {
            // TODO undo subcommand, refer: https://docs.rs/clap/2.33.3/clap/struct.ArgMatches.html#method.subcommand
            _ => {
                let sanitizer = sanitizers::root::EntrySanitizer;
                let paths = match sanitizer.sanitize(
                    matches
                        .values_of("ENTRY")
                        .expect("Could not reach ENTRY values."),
                ) {
                    Ok(p) => p,
                    Err(e) => return Err(e),
                };
                self.root(paths)
            }
        }
    }
}

impl Drop for Rment {
    fn drop(&mut self) {
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use directories::UserDirs;
    use rstest::*;
    use serial_test::serial;

    #[fixture]
    fn app() -> Rment {
        Rment::new().expect("Could not create rment instance.")
    }

    #[rstest]
    #[serial]
    fn test_root(app: Rment) {
        let paths = {
            let homedir = UserDirs::new()
                .expect("Could not get user dirs.")
                .home_dir()
                .to_path_buf();

            let file = {
                let mut path = homedir.clone();
                path.push("file1.txt");
                fs::write(
                    path.clone(),
                    "Laboriosam sapiente dolorum deleniti est dolor.",
                )
                .expect("Could not write to file1.txt");
                path
            };

            let dir = {
                let mut path = homedir.clone();
                path.push("dir");
                fs::create_dir_all(path.clone()).expect("Could not create dir.");

                let mut file = path.clone();
                file.push("file2.txt");
                fs::write(file, "Deserunt rerum quam excepturi magnam quia.")
                    .expect("Could not write to file2.txt");

                path
            };

            let missing = {
                let mut path = homedir.clone();
                path.push("missing.txt");
                path
            };

            vec![file, dir, missing]
        };

        let _ = app.root(paths.clone());

        paths.iter().for_each(|p| {
            assert!(!p.exists());

            // clean up
            if p.is_file() {
                let _ = fs::remove_file(p);
            } else {
                let _ = fs::remove_dir_all(p);
            }
        });
    }
}
