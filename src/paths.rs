use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

use directories::{ProjectDirs, UserDirs};
use log::{debug, trace};

use crate::{
    app::{ApplicationError, ApplicationResult},
    ExitCodes,
};

// Copyright 2021 erayerdin
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

/// Paths of an app.
pub struct Paths {
    app_name: String,
    project_dirs: ProjectDirs,
    user_dirs: UserDirs,
}

impl Paths {
    pub fn new(app_name: &str) -> ApplicationResult<Self> {
        debug!("Initializing Paths for {}...", app_name);
        let app_name = app_name.to_owned();

        let project_dirs = match ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            Some(d) => d,
            None => {
                return Err(ApplicationError::InitError {
                    exit_code: ExitCodes::DirectoriesInitFailure.into(),
                    message: "Could not initialize ProjectDirs.".to_owned(),
                })
            }
        };

        let user_dirs = match UserDirs::new() {
            Some(d) => d,
            None => {
                return Err(ApplicationError::InitError {
                    exit_code: ExitCodes::DirectoriesInitFailure.into(),
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

    pub fn get_config_file(&self, home: bool, create: bool) -> ApplicationResult<PathBuf> {
        debug!("Getting config file...");
        trace!("home: {}", home);
        trace!("create: {}", create);

        let mut path = match home {
            true => self.user_dirs.home_dir().to_path_buf(),
            false => self.project_dirs.config_dir().to_path_buf(),
        };
        path.push(format!(
            "{}{}.config.toml",
            if home { "." } else { "" }, // if home, then add dot to start, to hide it in unix systems
            self.app_name
        ));
        trace!("config file path: {}", path.to_string_lossy());

        if create {
            debug!("Creating config file...");

            if !home {
                debug!("Creating parent directories...");
                let parent_path = match path.parent() {
                    Some(p) => p,
                    None => {
                        return Err(ApplicationError::InitError {
                            exit_code: ExitCodes::ConfigFileFailure.into(),
                            message: "Could not get parent directory.".to_owned(),
                        })
                    }
                };
                match create_dir_all(parent_path) {
                    Err(e) => {
                        return Err(ApplicationError::InitError {
                            exit_code: ExitCodes::ConfigFileFailure.into(),
                            message: format!("Could not create parent directories. {}", e),
                        })
                    }
                    _ => {}
                };
            }

            debug!("Creating config file...");
            match File::create(path.clone()) {
                Ok(f) => match f.sync_all() {
                    Err(e) => {
                        return Err(ApplicationError::InitError {
                            exit_code: ExitCodes::ConfigFileFailure.into(),
                            message: format!("Could not sync the config file. {}", e),
                        })
                    }
                    _ => {}
                },
                Err(e) => {
                    return Err(ApplicationError::InitError {
                        exit_code: ExitCodes::ConfigFileFailure.into(),
                        message: format!("Could not create the config file. {}", e),
                    })
                }
            }
        }

        Ok(path)
    }

    pub fn get_data_dir(&self, create: bool) -> ApplicationResult<PathBuf> {
        debug!("Getting data directory...");
        trace!("create: {}", create);

        let mut path = self.project_dirs.data_dir().to_path_buf();
        path.push(format!("{}", self.app_name));
        trace!("data dir path: {}", path.to_string_lossy());

        if create {
            debug!("Creating data directory...");

            match create_dir_all(path.clone()) {
                Err(e) => {
                    return Err(ApplicationError::InitError {
                        exit_code: ExitCodes::DataDirectoryFailure.into(),
                        message: format!("Could not create data directory. {}", e),
                    })
                }
                _ => {}
            }
        }

        Ok(path)
    }

    pub fn get_cache_dir(&self, create: bool) -> ApplicationResult<PathBuf> {
        debug!("Getting cache directory...");
        trace!("create: {}", create);

        let mut path = self.project_dirs.cache_dir().to_path_buf();
        path.push(format!("{}", self.app_name));
        trace!("cache directory path: {}", path.to_string_lossy());

        if create {
            debug!("Creating cache directory...");

            match create_dir_all(path.clone()) {
                Err(e) => {
                    return Err(ApplicationError::InitError {
                        exit_code: ExitCodes::CacheDirectoryFailure.into(),
                        message: format!("Could not create cache directory. {}", e),
                    })
                }
                _ => {}
            };
        }

        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::{
        ffi::OsStr,
        fs::{remove_dir_all, remove_file},
    };

    use super::*;
    use rstest::*;

    #[fixture]
    fn paths() -> Paths {
        Paths::new("foo").expect("Could not initialize Paths.")
    }

    #[rstest(
        home => [true, false],
        create => [true, false],
    )]
    #[serial]
    fn test_config_file(paths: Paths, home: bool, create: bool) {
        {
            // setup
            let config_file = paths
                .get_config_file(home, false)
                .expect("Could not initialize config file.");
            let _ = remove_file(config_file);
        }
        let config_file = paths
            .get_config_file(home, create)
            .expect("Could not initialize config file.");

        match home {
            true => assert_eq!(
                Some(OsStr::new(".foo.config.toml")),
                config_file.file_name()
            ),
            false => assert_eq!(Some(OsStr::new("foo.config.toml")), config_file.file_name()),
        };

        assert_eq!(config_file.exists(), create);
    }

    #[rstest(
        create => [true, false]
    )]
    #[serial]
    fn test_data_dir(paths: Paths, create: bool) {
        {
            // setup
            let data_dir = paths
                .get_data_dir(false)
                .expect("Could not initialize data dir.");
            let _ = remove_dir_all(data_dir);
        }

        let data_dir = paths
            .get_data_dir(create)
            .expect("Could not initialize data dir.");
        assert_eq!(data_dir.exists(), create);
    }

    #[rstest(
        create => [true, false]
    )]
    #[serial]
    fn test_cache_dir(paths: Paths, create: bool) {
        {
            // setup
            let cache_dir = paths
                .get_cache_dir(false)
                .expect("Could not initialize cache dir.");
            let _ = remove_dir_all(cache_dir);
        }

        let cache_dir = paths
            .get_cache_dir(create)
            .expect("Could not initialize cache dir.");
        assert_eq!(cache_dir.exists(), create);
    }
}