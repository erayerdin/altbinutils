use clap::{App as Clapp, AppSettings, ArgMatches};
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

#[async_trait]
pub trait Application {
    async fn run(
        &self,
        matches: ArgMatches<'_>,
        metadata: Metadata,
        appdata: AppData,
        config: Figment,
    ) -> ApplicationResult<()>;
    async fn clapp(&self) -> Clapp;
    async fn metadata(&self) -> ApplicationResult<Metadata> {
        debug!("Generating Metadata...");
        let r = Metadata::default();
        trace!("Metadata Result: {:?}", r);
        r
    }
    async fn appdata(&self, metadata: Metadata) -> ApplicationResult<AppData> {
        debug!("Generating AppData...");
        let r = AppData::new(Some(metadata.name));
        trace!("AppData Result: {:?}", r);
        r
    }
    async fn config(&self, metadata: Metadata, appdata: AppData) -> ApplicationResult<Figment> {
        debug!("Generating Figment...");

        let appdata_config_path = appdata
            .get_entry(Entry::Config("config.toml".into()), false)
            .await;
        let home_config_path = appdata
            .get_entry(
                Entry::Home(format!("{}.config.toml", metadata.name).into()),
                false,
            )
            .await;

        Ok(Figment::new()
            .merge(Toml::file(appdata_config_path))
            .merge(Toml::file(home_config_path)))
    }
}

pub async fn invoke_application(app: impl Application + Send + Sync) -> i32 {
    debug!("Initializing the application...");
    setup_panic!(Metadata {
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        authors: env!("CARGO_PKG_AUTHORS").into(),
        homepage: "".into()
    });

    let app_matches = app.clapp().await.get_matches();
    let metadata = match app.metadata().await {
        Ok(m) => m,
        Err(e) => return e.get_exit_code(),
    };
    let appdata = match app.appdata(metadata.clone()).await {
        Ok(a) => a,
        Err(e) => return e.get_exit_code(),
    };
    let config = match app.config(metadata.clone(), appdata.clone()).await {
        Ok(f) => f,
        Err(e) => return e.get_exit_code(),
    };

    debug!("Running the application...");
    match app.run(app_matches, metadata, appdata, config).await {
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

pub const GLOBAL_CLAP_SETTINGS: [AppSettings; 2] = [
    AppSettings::VersionlessSubcommands,
    AppSettings::DeriveDisplayOrder,
];

#[cfg(test)]
mod tests {
    use crate::error::ApplicationError;

    use super::*;
    use rstest::*;

    struct RunFailApp;
    struct SuccessfulApp;

    #[async_trait]
    impl Application for RunFailApp {
        async fn run(
            &self,
            _: ArgMatches<'_>,
            _: Metadata,
            _: AppData,
            _: Figment,
        ) -> ApplicationResult<()> {
            Err(ApplicationError::RunError {
                exit_code: 200,
                message: "run failure".to_owned(),
            })
        }

        async fn clapp(&self) -> Clapp {
            app_from_crate!()
        }
    }

    impl Drop for RunFailApp {
        fn drop(&mut self) {
            ()
        }
    }

    #[async_trait]
    impl Application for SuccessfulApp {
        async fn run(
            &self,
            _: ArgMatches<'_>,
            _: Metadata,
            _: AppData,
            _: Figment,
        ) -> ApplicationResult<()> {
            Ok(())
        }

        async fn clapp(&self) -> Clapp {
            app_from_crate!()
        }
    }

    impl Drop for SuccessfulApp {
        fn drop(&mut self) {
            ()
        }
    }

    #[rstest]
    async fn test_run_fail() {
        let app = RunFailApp;
        let exit_code = invoke_application(app).await;

        assert_eq!(exit_code, 200);
    }

    #[rstest]
    async fn test_successful_app() {
        let app = SuccessfulApp;
        let exit_code = invoke_application(app).await;

        assert_eq!(exit_code, 0);
    }
}
