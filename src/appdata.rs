use std::path;

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
    async fn get_path(&self) -> path::PathBuf {
        match self {
            Entry::Data(p) => p.clone(),
            Entry::Config(p) => p.clone(),
            Entry::Cache(p) => p.clone(),
            Entry::Home(p) => p.clone(),
        }
    }
}

/// Paths of an app.
#[derive(Debug, Clone)]
pub struct AppData {
    app_name: String,
    project_dirs: ProjectDirs,
    user_dirs: UserDirs,
}

impl AppData {
    pub fn new<S: Into<String>>(app_name: Option<S>) -> ApplicationResult<Self> {
        let app_name: String = match app_name {
            Some(s) => s.into(),
            None => env!("CARGO_PKG_NAME").into(),
        };

        debug!("Initializing AppData paths for {}...", app_name);
        let app_name = app_name.to_owned();

        let project_dirs = match ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            Some(d) => d,
            None => {
                return Err(ApplicationError::InitError {
                    exit_code: CommonExitCodes::DirectoriesProjectDirsFailure as i32,
                    message: "Could not initialize ProjectDirs.".to_owned(),
                })
            }
        };

        let user_dirs = match UserDirs::new() {
            Some(d) => d,
            None => {
                return Err(ApplicationError::InitError {
                    exit_code: CommonExitCodes::DirectoriesUserDirsFailure as i32,
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

    /// Gets a specific entry from appdata directories.
    ///
    /// - **entry**: The type of entry.
    /// - **is_root**: If `is_root`, then `altbinutils` directory will be returned, otherwise
    /// the application's appdata directory will be returned.
    pub async fn get_entry(&self, entry: Entry, is_root: bool) -> path::PathBuf {
        debug!("Getting entry...");
        trace!("entry: {:?}", entry);
        trace!("is root: {:?}", is_root);

        let mut base_dir = match entry {
            Entry::Data(_) => self.project_dirs.data_local_dir(),
            Entry::Config(_) => self.project_dirs.config_dir(),
            Entry::Cache(_) => self.project_dirs.cache_dir(),
            Entry::Home(_) => self.user_dirs.home_dir(),
        }
        .to_path_buf();

        base_dir.push(entry.get_path().await);

        if is_root {
            return base_dir;
        }

        base_dir.push(format!("{}", self.app_name));
        trace!("base dir: {}", base_dir.to_string_lossy());

        base_dir.push(entry.get_path().await);

        base_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::logger;
    use rstest::*;
    use std::fs;

    #[fixture]
    fn appdata() -> AppData {
        AppData::new(Some("foo")).expect("Could not initialize Paths.")
    }

    #[fixture]
    fn empty_pathbuf() -> path::PathBuf {
        path::PathBuf::from("")
    }

    #[rstest]
    #[case(true)]
    #[case(false)]
    async fn test_data_dir(
        #[allow(unused_variables)] logger: bool,
        appdata: AppData,
        empty_pathbuf: path::PathBuf,
        #[case] is_root: bool,
    ) {
        {
            // setup
            let path = appdata
                .get_entry(Entry::Data(empty_pathbuf.clone()), is_root)
                .await;
            let _ = fs::remove_dir_all(path);
        }

        let path = appdata.get_entry(Entry::Data(empty_pathbuf), is_root).await;

        if is_root {
            let terminal = if cfg!(target_os = "windows") {
                path.parent()
                    .expect("Could not get parent of path.")
                    .file_name()
                    .expect("Could not get terminal of path.")
            } else {
                path.file_name().expect("Could not get terminal of path.")
            };
            assert_eq!(
                terminal,
                if cfg!(target_os = "macos") {
                    "io.github.erayerdin.altbinutils"
                } else {
                    "altbinutils"
                }
            );
        } else {
            let (terminal, parent) = {
                let parent = if cfg!(target_os = "windows") {
                    path.parent()
                        .expect("Could not get parent of path.")
                        .parent()
                        .expect("Could not get parent of path.")
                } else {
                    path.parent().expect("Could not get parent of path.")
                };
                (
                    path.file_name().expect("Could not get terminal of path."),
                    parent.file_name().expect("Could not get terminal of path."),
                )
            };
            assert_eq!(terminal, "foo");
            assert_eq!(
                parent,
                if cfg!(target_os = "macos") {
                    "io.github.erayerdin.altbinutils"
                } else {
                    "altbinutils"
                }
            );
        }
    }

    #[rstest]
    #[case(true)]
    #[case(false)]
    async fn test_cache_dir(
        #[allow(unused_variables)] logger: bool,
        appdata: AppData,
        empty_pathbuf: path::PathBuf,
        #[case] is_root: bool,
    ) {
        {
            // setup
            let path = appdata
                .get_entry(Entry::Cache(empty_pathbuf.clone()), is_root)
                .await;
            let _ = fs::remove_dir_all(path);
        }

        let path = appdata
            .get_entry(Entry::Cache(empty_pathbuf), is_root)
            .await;

        if is_root {
            let terminal = if cfg!(target_os = "windows") {
                path.parent()
                    .expect("Could not get parent of path.")
                    .file_name()
                    .expect("Could not get terminal of path.")
            } else {
                path.file_name().expect("Could not get terminal of path.")
            };
            assert_eq!(
                terminal,
                if cfg!(target_os = "macos") {
                    "io.github.erayerdin.altbinutils"
                } else {
                    "altbinutils"
                }
            );
        } else {
            let (terminal, parent) = {
                let parent = if cfg!(target_os = "windows") {
                    path.parent()
                        .expect("Could not get parent of path.")
                        .parent()
                        .expect("Could not get parent of path.")
                } else {
                    path.parent().expect("Could not get parent of path.")
                };
                (
                    path.file_name().expect("Could not get terminal of path."),
                    parent.file_name().expect("Could not get terminal of path."),
                )
            };
            assert_eq!(terminal, "foo");
            assert_eq!(
                parent,
                if cfg!(target_os = "macos") {
                    "io.github.erayerdin.altbinutils"
                } else {
                    "altbinutils"
                }
            );
        }
    }

    #[rstest]
    #[case(true)]
    #[case(false)]
    async fn test_config_dir(
        #[allow(unused_variables)] logger: bool,
        appdata: AppData,
        empty_pathbuf: path::PathBuf,
        #[case] is_root: bool,
    ) {
        {
            // setup
            let path = appdata
                .get_entry(Entry::Config(empty_pathbuf.clone()), is_root)
                .await;
            let _ = fs::remove_dir_all(path);
        }

        let path = appdata
            .get_entry(Entry::Config(empty_pathbuf), is_root)
            .await;

        if is_root {
            let terminal = if cfg!(target_os = "windows") {
                path.parent()
                    .expect("Could not get parent of path.")
                    .file_name()
                    .expect("Could not get terminal of path.")
            } else {
                path.file_name().expect("Could not get terminal of path.")
            };
            assert_eq!(
                terminal,
                if cfg!(target_os = "macos") {
                    "io.github.erayerdin.altbinutils"
                } else {
                    "altbinutils"
                }
            );
        } else {
            let (terminal, parent) = {
                let parent = if cfg!(target_os = "windows") {
                    path.parent()
                        .expect("Could not get parent of path.")
                        .parent()
                        .expect("Could not get parent of path.")
                } else {
                    path.parent().expect("Could not get parent of path.")
                };
                (
                    path.file_name().expect("Could not get terminal of path."),
                    parent.file_name().expect("Could not get terminal of path."),
                )
            };
            assert_eq!(terminal, "foo");
            assert_eq!(
                parent,
                if cfg!(target_os = "macos") {
                    "io.github.erayerdin.altbinutils"
                } else {
                    "altbinutils"
                }
            );
        }
    }
}
