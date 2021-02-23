use altbinutils::app::Application;
use app_rment::app::Rment;

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

#[quit::main]
fn main() {
    let mut app = Rment::new().unwrap_or_else(|e| {
        quit::with_code(e.get_exit_code());
    });

    if let Err(e) = app.run() {
        // TODO print message
        quit::with_code(e.get_exit_code());
    }
}
