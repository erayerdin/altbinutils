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
pub mod paths;
pub mod sanitizer;

pub const AFTER_HELP_LICENSE_TEXT: &str =
    "This software is licensed under the terms of Apache License 2.0.
To read the details, refer to: https://www.apache.org/licenses/LICENSE-2.0";

pub enum ExitCodes {
    DirectoriesFailure = -2,
    PathsFailure = -3,
    LogFailure = -4,
}
