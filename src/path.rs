use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

use directories::{ProjectDirs, UserDirs};
use lazy_static::lazy_static;
use log::*;

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

lazy_static! {
    static ref PROJECT_DIRS: ProjectDirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .expect("Could not initialize project directories.");
}

lazy_static! {
    static ref USER_DIRS: UserDirs =
        UserDirs::new().expect("Could not initialize user directories.");
}

/// Paths of an app.
pub struct Paths {
    app_name: String,
}

impl Paths {
    pub fn new(app_name: &str) -> Self {
        debug!("Initializing Paths for {}...", app_name);
        Self {
            app_name: app_name.to_owned(),
        }
    }

    pub fn get_config_file(&self, home: bool, create: bool) -> PathBuf {
        debug!("Getting config file...");
        trace!("home: {}", home);
        trace!("create: {}", create);

        let mut path = match home {
            true => USER_DIRS.home_dir().to_path_buf(),
            false => PROJECT_DIRS.config_dir().to_path_buf(),
        };
        path.push(format!(
            "{}{}.config.toml",
            if home { "." } else { "" }, // if home, then add dot to start, to hide it in unix systems
            self.app_name
        ));

        if create {
            debug!("Creating config file...");
            trace!("config file path: {}", path.to_string_lossy());

            if !home {
                debug!("Creating parent path...");
                let parent_path = path
                    .parent()
                    .expect("Could not get the parent path of config file.");
                create_dir_all(parent_path).expect("Could not create parent paths.");
            }

            File::create(path.clone()).expect("Could not create config file.");
        }

        path
    }
}

#[cfg(test)]
mod tests {
    use std::{ffi::OsStr, fs::remove_file};

    use super::*;
    use rstest::*;

    #[fixture]
    fn paths() -> Paths {
        Paths::new("foo")
    }

    #[rstest(
        home => [true, false],
        create => [true, false],
    )]
    fn test_config_file(paths: Paths, home: bool, create: bool) {
        let config_file = paths.get_config_file(home, create);

        match home {
            true => assert_eq!(
                Some(OsStr::new(".foo.config.toml")),
                config_file.file_name()
            ),
            false => assert_eq!(Some(OsStr::new("foo.config.toml")), config_file.file_name()),
        };

        match create {
            true => {
                assert!(config_file.exists());
                // tidy up
                remove_file(config_file).expect("Could not delete config file.");
            }
            false => assert!(!config_file.exists()),
        }
    }
}
