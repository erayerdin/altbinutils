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

pub mod app;
pub mod appdata;
pub mod error;
pub mod exit;
pub mod result;

#[cfg(test)]
pub mod tests {
    use rstest::*;
    use simple_logger;

    #[fixture]
    pub fn logger() -> bool {
        match simple_logger::SimpleLogger::new().init() {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
