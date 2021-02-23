use std::process;

use clap::ArgMatches;
use fern::Dispatch;
use log::{debug, error};

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
    pub fn get_exit_code(&self) -> i32 {
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

pub type ApplicationResult<T> = Result<T, ApplicationError>;

type InvokeReturn = (i32, String);

fn fail_invoke(step: &str, err: ApplicationError) -> InvokeReturn {
    let exit_code = err.get_exit_code();
    let message = err.get_message();

    error!("Failed to {} the application.", step);
    error!("{}", message);

    if cfg!(not(test)) {
        process::exit(exit_code);
    }

    (exit_code, message)
}

pub trait Application {
    fn init(&self, logger: Dispatch) -> ApplicationResult<ArgMatches>;
    fn run(&self, matches: ArgMatches) -> ApplicationResult<()>;
}

pub fn invoke_application<A>(app: A) -> InvokeReturn
where
    A: Application + Drop,
{
    let logger = if cfg!(debug_assertions) {
        Dispatch::new().format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                record.level(),
                message
            ))
        })
    } else {
        Dispatch::new().format(|out, message, _| {
            // TODO append level to anything except info level
            out.finish(format_args!("{}", message))
        })
    };

    debug!("Invoking the application...");

    debug!("Initializing the application...");
    match app.init(logger) {
        Ok(m) => {
            debug!("Finished the initialization of application successfully.");
            debug!("Running the application...");

            match app.run(m) {
                Ok(_) => {
                    debug!("Finished the running of application successfully.");
                    (0, "".to_owned())
                }
                Err(e) => fail_invoke("run", e),
            }
        }
        Err(e) => fail_invoke("initialize", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::App;
    use rstest::*;

    struct InitFailApp;
    struct RunFailApp;
    struct SuccessfulApp;

    impl Application for InitFailApp {
        fn init(&self, _: Dispatch) -> ApplicationResult<ArgMatches> {
            Err(ApplicationError::InitError {
                exit_code: 100,
                message: "init failure".to_owned(),
            })
        }

        fn run(&self, _: ArgMatches) -> ApplicationResult<()> {
            Ok(())
        }
    }

    impl Drop for InitFailApp {
        fn drop(&mut self) {
            ()
        }
    }

    impl Application for RunFailApp {
        fn init(&self, _: Dispatch) -> ApplicationResult<ArgMatches> {
            Ok(App::new("runfailapp").get_matches())
        }

        fn run(&self, _: ArgMatches) -> ApplicationResult<()> {
            Err(ApplicationError::RunError {
                exit_code: 200,
                message: "run failure".to_owned(),
            })
        }
    }

    impl Drop for RunFailApp {
        fn drop(&mut self) {
            ()
        }
    }

    impl Application for SuccessfulApp {
        fn init(&self, _: Dispatch) -> ApplicationResult<ArgMatches> {
            Ok(App::new("successfulapp").get_matches())
        }

        fn run(&self, _: ArgMatches) -> ApplicationResult<()> {
            Ok(())
        }
    }

    impl Drop for SuccessfulApp {
        fn drop(&mut self) {
            ()
        }
    }

    #[rstest]
    fn test_init_fail() {
        let app = InitFailApp;
        let (exit_code, message) = invoke_application(app);

        assert_eq!(message, "init failure");
        assert_eq!(exit_code, 100);
    }

    #[rstest]
    fn test_run_fail() {
        let app = RunFailApp;
        let (exit_code, message) = invoke_application(app);

        assert_eq!(message, "run failure");
        assert_eq!(exit_code, 200);
    }

    #[rstest]
    fn test_successful_app() {
        let app = SuccessfulApp;
        let (exit_code, message) = invoke_application(app);

        assert_eq!(exit_code, 0);
        assert_eq!(message, "");
    }
}
