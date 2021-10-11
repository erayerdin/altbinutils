use std::fmt;

use log::{debug, trace};
use semver::Version;

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

#[macro_export]
macro_rules! metadata {
    () => {{
        (|| {
            use semver::Version;

            let name = env!("CARGO_PKG_NAME").to_owned();
            let version = match Version::parse(env!("CARGO_PKG_VERSION")) {
                Ok(v) => v,
                Err(e) => {
                    return Err(ApplicationError::InitError {
                        exit_code: crate::exit::CommonExitCodes::SemverVersionParseFailure as i32,
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
            Ok(Metadata {
                name,
                version,
                description,
                authors,
            })
        })()
    }};
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub authors: Vec<String>,
}

impl Metadata {
    pub fn new<S: Into<String> + fmt::Display + fmt::Debug>(
        name: S,
        version: Version,
        description: S,
        authors: Vec<S>,
    ) -> Self {
        debug!("Initializing metadata...");
        trace!("name: {}", name);
        trace!("version: {}", version);
        trace!("description: {}", description);
        trace!("authors: {:?}", authors);

        Self {
            name: name.into(),
            version,
            description: description.into(),
            authors: authors.into_iter().map(|v| v.into()).collect(),
        }
    }
}
