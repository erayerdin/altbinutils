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

pub trait Application {
    fn run(&self) -> ApplicationResult<()>;
}

#[allow(drop_bounds)]
pub fn invoke_application<A>(app: A) -> i32
where
    A: Application + Drop,
{
    debug!("Running the application...");
    match app.run() {
        Ok(_) => {
            debug!("Finished running the application successfully.");
            0
        }
        Err(e) => {
            error!("Failed to run the application.");
            e.get_exit_code()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    struct RunFailApp;
    struct SuccessfulApp;

    impl Application for RunFailApp {
        fn run(&self) -> ApplicationResult<()> {
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
        fn run(&self) -> ApplicationResult<()> {
            Ok(())
        }
    }

    impl Drop for SuccessfulApp {
        fn drop(&mut self) {
            ()
        }
    }

    #[rstest]
    fn test_run_fail() {
        let app = RunFailApp;
        let exit_code = invoke_application(app);

        assert_eq!(exit_code, 200);
    }

    #[rstest]
    fn test_successful_app() {
        let app = SuccessfulApp;
        let exit_code = invoke_application(app);

        assert_eq!(exit_code, 0);
    }
}
