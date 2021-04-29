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

/// Common exit codes that can be arisen from any applications.
///
/// **1. Conventions**
///
/// All exit codes should end with `Failure` suffix and all exit codes must
/// be negative `i32`. Positives are meant to be used by applications.
///
/// **1.1 Exit Code Ranges**
///
/// Exit code `-1` is reserved by `clap`. After that, each -100 chunk will be
/// for each dependency and their submodules, except the first, which is between
/// -2 and -99.
///
/// No positive integer is and will be used by CommonExitCodes enum. Positive `i32`
/// exit codes are reserved for any application utilizing this.
pub enum CommonExitCodes {
    // std failures
    StdFsFailure = -2,

    // directories failures
    // TODO split to ProjectDirs and UserDirs
    DirectoriesFailure = -100,
}
