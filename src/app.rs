use human_panic::setup_panic;
use log::{debug, error};

use crate::result::ApplicationResult;

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

pub trait Application<'a> {
    fn run(&self) -> ApplicationResult<()>;
}

pub fn invoke_application<'a, A>(app: A) -> i32
where
    A: Application<'a>,
{
    debug!("Initializing the application...");
    setup_panic!(Metadata {
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        authors: env!("CARGO_PKG_AUTHORS").into(),
        homepage: "".into()
    });

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
    use crate::error::ApplicationError;

    use super::*;
    use rstest::*;

    struct RunFailApp;
    struct SuccessfulApp;

    impl<'a> Application<'a> for RunFailApp {
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

    impl<'a> Application<'a> for SuccessfulApp {
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
