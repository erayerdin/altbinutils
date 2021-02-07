use directories::ProjectDirs;
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
        .expect("Could not initializer project directories.");
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
}
