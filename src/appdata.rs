use std::{fs, path};

use directories::{ProjectDirs, UserDirs};
use log::{debug, trace};

use crate::{error::ApplicationError, exit::CommonExitCodes, result::ApplicationResult};

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

const QUALIFIER: &str = "io.github";
const ORGANIZATION: &str = "erayerdin";
const APPLICATION: &str = "altbinutils";

/// An entry in application directory.
#[derive(Debug)]
pub enum Entry {
    Data(path::PathBuf),
    Config(path::PathBuf),
    Cache(path::PathBuf),
    Home(path::PathBuf),
}

impl Entry {
    fn get_repr(&self) -> &str {
        match self {
            Entry::Data(_) => "data",
            Entry::Config(_) => "config",
            Entry::Cache(_) => "cache",
            Entry::Home(_) => "home",
        }
    }

    fn get_path(&self) -> path::PathBuf {
        match self {
            Entry::Data(p) => p.clone(),
            Entry::Config(p) => p.clone(),
            Entry::Cache(p) => p.clone(),
            Entry::Home(p) => p.clone(),
        }
    }
}

/// Paths of an app.
pub struct AppData {
    app_name: String,
    project_dirs: ProjectDirs,
    user_dirs: UserDirs,
}

impl AppData {
    pub fn new(app_name: &str) -> ApplicationResult<Self> {
        debug!("Initializing Paths for {}...", app_name);
        let app_name = app_name.to_owned();

        let project_dirs = match ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            Some(d) => d,
            None => {
                return Err(ApplicationError::InitError {
                    exit_code: CommonExitCodes::DirectoriesFailure as i32,
                    message: "Could not initialize ProjectDirs.".to_owned(),
                })
            }
        };

        let user_dirs = match UserDirs::new() {
            Some(d) => d,
            None => {
                return Err(ApplicationError::InitError {
                    exit_code: CommonExitCodes::DirectoriesFailure as i32,
                    message: "Could not initialize UserDirs.".to_owned(),
                })
            }
        };

        Ok(Self {
            app_name,
            project_dirs,
            user_dirs,
        })
    }

    pub fn get_entry(&self, entry: Entry) -> ApplicationResult<path::PathBuf> {
        debug!("Getting entry...");
        trace!("entry: {:?}", entry);

        let mut base_dir = match entry {
            Entry::Data(_) => self.project_dirs.data_local_dir(),
            Entry::Config(_) => self.project_dirs.config_dir(),
            Entry::Cache(_) => self.project_dirs.cache_dir(),
            Entry::Home(_) => self.user_dirs.home_dir(),
        }
        .to_path_buf();
        base_dir.push(format!("{}", self.app_name));
        trace!("base dir: {}", base_dir.to_string_lossy());

        match entry {
            Entry::Home(_) => (),
            _ => {
                debug!("Creating base {} directory...", entry.get_repr());
                if let Err(e) = fs::create_dir_all(base_dir.clone()) {
                    return Err(ApplicationError::InitError {
                        exit_code: CommonExitCodes::StdFsFailure as i32,
                        message: format!("Could not create base directory. {}", e),
                    });
                }
            }
        }

        base_dir.push(entry.get_path());

        Ok(base_dir)
    }
}

pub fn get_config_file(paths: &AppData, home: bool) -> ApplicationResult<path::PathBuf> {
    debug!("Getting config file...");
    trace!("home: {}", home);

    let file_name = match home {
        true => format!(".{}.config.toml", paths.app_name),
        false => format!("{}.config.toml", paths.app_name),
    };
    trace!("file name: {}", file_name);

    paths.get_entry(match home {
        true => Entry::Home(path::PathBuf::from(file_name)),
        false => Entry::Config(path::PathBuf::from(file_name)),
    })
}

pub fn get_log_file(paths: &AppData) -> ApplicationResult<path::PathBuf> {
    debug!("Getting log file...");

    paths.get_entry(Entry::Cache(path::PathBuf::from("app.log")))
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use serial_test::serial;

    use super::*;
    use rstest::*;

    #[fixture]
    fn appdata() -> AppData {
        AppData::new("foo").expect("Could not initialize Paths.")
    }

    #[rstest(
        home => [true, false]
    )]
    fn test_config_file(appdata: AppData, home: bool) {
        let config_file = get_config_file(&appdata, home).expect("Could not get config file.");
        let config_file_name = config_file.file_name();

        assert_eq!(
            config_file_name,
            match home {
                true => Some(OsStr::new(".foo.config.toml")),
                false => Some(OsStr::new("foo.config.toml")),
            }
        );
    }

    #[rstest]
    fn test_log_file(appdata: AppData) {
        let log_file = get_log_file(&appdata).expect("Could not get log file.");
        assert_eq!(log_file.file_name(), Some(OsStr::new("app.log")));
    }

    mod test_paths {
        use super::*;

        #[rstest]
        #[serial]
        fn test_data_dir(appdata: AppData) {
            {
                // setup
                let path = appdata
                    .get_entry(Entry::Data(path::PathBuf::from("")))
                    .expect("Could not initialize data dir.");
                let _ = fs::remove_dir_all(path);
            }

            let path = appdata
                .get_entry(Entry::Data(path::PathBuf::from("")))
                .expect("Could not initialize data dir.");
            assert!(path.exists());
        }

        #[rstest]
        #[serial]
        fn test_cache_dir(appdata: AppData) {
            {
                // setup
                let path = appdata
                    .get_entry(Entry::Cache(path::PathBuf::from("")))
                    .expect("Could not initialize cache dir.");
                let _ = fs::remove_dir_all(path);
            }

            let path = appdata
                .get_entry(Entry::Cache(path::PathBuf::from("")))
                .expect("Could not initialize cache dir.");
            assert!(path.exists());
        }

        #[rstest]
        #[serial]
        fn test_config_dir(appdata: AppData) {
            {
                // setup
                let path = appdata
                    .get_entry(Entry::Config(path::PathBuf::from("")))
                    .expect("Could not initialize config dir.");
                let _ = fs::remove_dir_all(path);
            }

            let path = appdata
                .get_entry(Entry::Config(path::PathBuf::from("")))
                .expect("Could not initialize config dir.");
            assert!(path.exists());
        }
    }
}
