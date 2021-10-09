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

#[derive(Debug)]
pub enum ApplicationError {
    InitError { exit_code: i32, message: String },
    RunError { exit_code: i32, message: String },
    DestroyError { exit_code: i32, message: String },
}

impl ApplicationError {
    pub async fn get_exit_code(&self) -> i32 {
        match self {
            ApplicationError::InitError { exit_code, .. } => exit_code.clone(),
            ApplicationError::RunError { exit_code, .. } => exit_code.clone(),
            ApplicationError::DestroyError { exit_code, .. } => exit_code.clone(),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            ApplicationError::InitError { message, .. } => message.clone(),
            ApplicationError::RunError { message, .. } => message.clone(),
            ApplicationError::DestroyError { message, .. } => message.clone(),
        }
    }
}
