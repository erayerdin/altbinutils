use log::error;

pub mod app;
pub mod path;

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

pub enum ExitCodes {
    DirectoriesInitFailure,
    ConfigFileFailure,
}

impl From<ExitCodes> for i32 {
    fn from(e: ExitCodes) -> Self {
        match e {
            ExitCodes::DirectoriesInitFailure => -2,
            ExitCodes::ConfigFileFailure => -3,
            #[allow(unreachable_patterns)]
            _ => {
                error!("The application should not have reached to this point. ExitCodes::get_exit_code match statement");
                -1
            }
        }
    }
}
