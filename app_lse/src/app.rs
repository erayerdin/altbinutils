use altbinutils::{
    app::{Application, Clapp},
    appdata::AppData,
    clap::{
        app_from_crate, crate_authors, crate_description, crate_name, crate_version, ArgMatches,
    },
    figment::Figment,
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

pub struct LseApplication;

#[async_trait]
impl Application for LseApplication {
    async fn run(
        &self,
        matches: ArgMatches<'_>,
        metadata: Metadata,
        appdata: AppData,
        config: Figment,
    ) -> ApplicationResult<()> {
        println!("Not implemented this app yet, but here's some info.");
        println!("metadata: {:?}", metadata);
        println!("appdata: {:?}", appdata);
        println!("config: {:?}", config);
        Ok(())
    }
    async fn clapp(&self) -> Clapp {
        app_from_crate!()
    }
}
