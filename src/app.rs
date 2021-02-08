use clap::App;
use snafu::Snafu;

use crate::{config::Config, path::Paths};

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

#[derive(Debug, Snafu)]
pub enum ApplicationError {
    #[snafu(display("An error occured while initializing application. {}", message))]
    InitError { exit_code: i32, message: String },
    #[snafu(display("An error occured while running application. {}", message))]
    RunError { exit_code: i32, message: String },
    #[snafu(display("An error occured while destroying application. {}", message))]
    DestroyError { exit_code: i32, message: String },
}

pub type ApplicationResult<T> = Result<T, ApplicationError>;

pub type DestroyHandlerType = dyn FnMut() + 'static + Send;

pub trait Application {
    fn init(&self) -> ApplicationResult<()>;
    fn run(&self, app: App, paths: &Paths, config: Box<&dyn Config>) -> ApplicationResult<()>;
    fn destroy(
        &self,
        paths: &Paths,
        config: Box<&dyn Config>,
        sigint_handler: Option<Box<DestroyHandlerType>>,
    ) -> ApplicationResult<()>;
}
