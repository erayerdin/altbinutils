use clap::Values;

use crate::app::ApplicationResult;

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

pub trait Sanitizer {
    type Target;

    fn sanitize(self, source: Values) -> ApplicationResult<Self::Target>;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use clap::{App, Arg};
    use rstest::*;

    struct EntrySanitizer;

    impl Sanitizer for EntrySanitizer {
        type Target = Vec<PathBuf>;

        fn sanitize(self, source: Values) -> ApplicationResult<Self::Target> {
            Ok(source
                .into_iter()
                .map(|v| {
                    let mut path = PathBuf::new();
                    path.push(v);
                    path
                })
                .collect())
        }
    }

    #[fixture]
    fn clapp<'a, 'b>() -> App<'a, 'b> {
        App::new("testapp").args(&[Arg::with_name("entry").multiple(true).required(true)])
    }

    #[rstest]
    fn test_sanitizer(clapp: App) {
        let matches = clapp.get_matches_from(vec!["testapp", "/foo", "/bar"]);
        let values = matches
            .values_of("entry")
            .expect("Could not get values of entry.");
        let sanitizer = EntrySanitizer;
        let paths = sanitizer.sanitize(values).expect("Could not get paths.");

        assert_eq!(paths[0].to_string_lossy(), "/foo");
        assert_eq!(paths[1].to_string_lossy(), "/bar");
    }
}
