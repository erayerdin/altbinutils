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

    pub fn get_entry(&self, entry: Entry, is_root: bool) -> ApplicationResult<path::PathBuf> {
        debug!("Getting entry...");
        trace!("entry: {:?}", entry);

        let mut base_dir = match entry {
            Entry::Data(_) => self.project_dirs.data_local_dir(),
            Entry::Config(_) => self.project_dirs.config_dir(),
            Entry::Cache(_) => self.project_dirs.cache_dir(),
            Entry::Home(_) => self.user_dirs.home_dir(),
        }
        .to_path_buf();

        if is_root {
            base_dir.push(entry.get_path());
            return Ok(base_dir);
        }

        base_dir.push(format!("{}", self.app_name));
        trace!("base dir: {}", base_dir.to_string_lossy());

        base_dir.push(entry.get_path());

        Ok(base_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn appdata() -> AppData {
        AppData::new("foo").expect("Could not initialize Paths.")
    }

    mod test_appdata {
        use std::fs;

        use super::*;

        #[fixture]
        fn empty_pathbuf() -> path::PathBuf {
            path::PathBuf::from("")
        }

        #[rstest]
        #[case(true)]
        #[case(false)]
        fn test_data_dir(appdata: AppData, empty_pathbuf: path::PathBuf, #[case] is_root: bool) {
            {
                // setup
                let path = appdata
                    .get_entry(Entry::Data(empty_pathbuf.clone()), is_root)
                    .expect("Could not initialize data dir.");
                let _ = fs::remove_dir_all(path);
            }

            let path = appdata
                .get_entry(Entry::Data(empty_pathbuf), is_root)
                .expect("Could not initialize data dir.");

            if is_root {
                assert!(path.ends_with(path::PathBuf::from("altbinutils")));
            } else {
                assert!(path.ends_with(path::PathBuf::from("altbinutils/foo")));
            }
        }

        #[rstest]
        #[case(true)]
        #[case(false)]
        fn test_cache_dir(appdata: AppData, empty_pathbuf: path::PathBuf, #[case] is_root: bool) {
            {
                // setup
                let path = appdata
                    .get_entry(Entry::Cache(empty_pathbuf.clone()), is_root)
                    .expect("Could not initialize cache dir.");
                let _ = fs::remove_dir_all(path);
            }

            let path = appdata
                .get_entry(Entry::Cache(empty_pathbuf), is_root)
                .expect("Could not initialize cache dir.");

            if is_root {
                assert!(path.ends_with(path::PathBuf::from("altbinutils")));
            } else {
                assert!(path.ends_with(path::PathBuf::from("altbinutils/foo")));
            }
        }

        #[rstest]
        #[case(true)]
        #[case(false)]
        fn test_config_dir(appdata: AppData, empty_pathbuf: path::PathBuf, #[case] is_root: bool) {
            {
                // setup
                let path = appdata
                    .get_entry(Entry::Config(empty_pathbuf.clone()), is_root)
                    .expect("Could not initialize config dir.");
                let _ = fs::remove_dir_all(path);
            }

            let path = appdata
                .get_entry(Entry::Config(empty_pathbuf), is_root)
                .expect("Could not initialize config dir.");

            if is_root {
                assert!(path.ends_with(path::PathBuf::from("altbinutils")));
            } else {
                assert!(path.ends_with(path::PathBuf::from("altbinutils/foo")));
            }
        }
    }
}
