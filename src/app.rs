use clap::{App as Clapp, ArgMatches};
use figment::{
    providers::{Format, Toml},
    Figment,
};
use human_panic::setup_panic;
use log::{debug, error};

use crate::{
    appdata::{AppData, Entry},
    metadata::Metadata,
    result::ApplicationResult,
};

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

pub trait Application {
    fn run(
        &self,
        matches: ArgMatches,
        metadata: Metadata,
        appdata: AppData,
        config: Figment,
    ) -> ApplicationResult<()>;
    fn clapp(&self) -> Clapp;
    fn metadata(&self) -> ApplicationResult<Metadata> {
        debug!("Generating Metadata...");
        let r = Metadata::default();
        trace!("Metadata Result: {:?}", r);
        r
    }
    fn appdata(&self, metadata: Metadata) -> ApplicationResult<AppData> {
        debug!("Generating AppData...");
        let r = AppData::new(Some(metadata.name));
        trace!("AppData Result: {:?}", r);
        r
    }
    fn config(&self, metadata: Metadata, appdata: AppData) -> ApplicationResult<Figment> {
        debug!("Generating Figment...");

        let appdata_config_path = appdata.get_entry(Entry::Config("config.toml".into()), false);
        let home_config_path = appdata.get_entry(
            Entry::Home(format!("{}.config.toml", metadata.name).into()),
            false,
        );

        Ok(Figment::new()
            .merge(Toml::file(appdata_config_path))
            .merge(Toml::file(home_config_path)))
    }
}

pub fn invoke_application<A>(app: A) -> i32
where
    A: Application,
{
    debug!("Initializing the application...");
    setup_panic!(Metadata {
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        authors: env!("CARGO_PKG_AUTHORS").into(),
        homepage: "".into()
    });

    let app_matches = app.clapp().get_matches();
    let metadata = match app.metadata() {
        Ok(m) => m,
        Err(e) => return e.get_exit_code(),
    };
    let appdata = match app.appdata(metadata.clone()) {
        Ok(a) => a,
        Err(e) => return e.get_exit_code(),
    };
    let config = match app.config(metadata.clone(), appdata.clone()) {
        Ok(f) => f,
        Err(e) => return e.get_exit_code(),
    };

    debug!("Running the application...");
    match app.run(app_matches, metadata, appdata, config) {
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

    impl Application for RunFailApp {
        fn run(&self, _: ArgMatches, _: Metadata, _: AppData, _: Figment) -> ApplicationResult<()> {
            Err(ApplicationError::RunError {
                exit_code: 200,
                message: "run failure".to_owned(),
            })
        }

        fn clapp(&self) -> Clapp {
            app_from_crate!()
        }
    }

    impl Drop for RunFailApp {
        fn drop(&mut self) {
            ()
        }
    }

    impl Application for SuccessfulApp {
        fn run(&self, _: ArgMatches, _: Metadata, _: AppData, _: Figment) -> ApplicationResult<()> {
            Ok(())
        }

        fn clapp(&self) -> Clapp {
            app_from_crate!()
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
