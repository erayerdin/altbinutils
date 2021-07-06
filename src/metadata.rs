use log::{debug, trace};
use semver::Version;

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

#[derive(Debug, Clone)]
pub struct Metadata {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub authors: Vec<String>,
}

impl Metadata {
    pub fn new(name: String, version: Version, description: String, authors: Vec<String>) -> Self {
        debug!("Initializing metadata...");
        trace!("name: {}", name);
        trace!("version: {}", version);
        trace!("description: {}", description);
        trace!("authors: {:?}", authors);

        Self {
            name,
            version,
            description,
            authors,
        }
    }

    pub fn default() -> ApplicationResult<Self> {
        debug!("Initializing metadata from defaults...");

        let name = env!("CARGO_PKG_NAME").to_owned();
        let version = match Version::parse(env!("CARGO_PKG_VERSION")) {
            Ok(v) => v,
            Err(e) => {
                return Err(ApplicationError::InitError {
                    exit_code: CommonExitCodes::SemverVersionParseFailure as i32,
                    message: format!("Could not parse version of application. {}", e),
                });
            }
        };
        let description = env!("CARGO_PKG_DESCRIPTION").to_owned();
        let authors: Vec<String> = env!("CARGO_PKG_AUTHORS")
            .split(";")
            .into_iter()
            .map(|s| s.trim().to_owned())
            .collect();

        trace!("name: {}", name);
        trace!("version: {}", version);
        trace!("description: {}", description);
        trace!("authors: {:?}", authors);

        Ok(Self {
            name,
            version,
            description,
            authors,
        })
    }
}
