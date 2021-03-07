use crate::ExitCodes;
use std::{env::current_dir, path::PathBuf};

use altbinutils::{
    app::{ApplicationError, ApplicationResult},
    sanitizer::Sanitizer,
};
use log::{debug, trace};
use path_dedot::ParseDot;

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

pub struct EntrySanitizer;

impl Sanitizer for EntrySanitizer {
    type Target = Vec<PathBuf>;

    fn sanitize(self, source: clap::Values) -> ApplicationResult<Self::Target> {
        debug!("Sanitizing ENTRY values...");

        let cwd = match current_dir() {
            Ok(p) => p,
            Err(e) => {
                return Err(ApplicationError::RunError {
                    exit_code: ExitCodes::CwdFailure as i32,
                    message: format!("Failed to get current working directory. {}", e),
                })
            }
        };
        trace!("cwd: {}", cwd.to_string_lossy());
        Ok(source
            .map(|v| {
                let mut path = cwd.clone();
                path.push(v);
                let final_path = match path.parse_dot() {
                    Ok(p) => p.to_path_buf(),
                    Err(_) => path,
                };
                trace!("path: {}", final_path.to_string_lossy());
                final_path
            })
            .collect())
    }
}
