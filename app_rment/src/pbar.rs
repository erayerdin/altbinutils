use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

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

pub fn get_progress_bar(total: u64) -> ProgressBar {
    let pbar = ProgressBar::new(total);
    pbar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} ({pos:>7}/{len:7}) {msg}")
            .progress_chars("█▎ "),
    );
    pbar
}

pub fn print_final_report(
    pbar: &ProgressBar,
    successful_file_count: u64,
    successful_dir_count: u64,
    failed_file_count: u64,
    failed_dir_count: u64,
    absent_count: u64,
    skipped_count: u64,
) {
    pbar.println(format!(
        "{}: {sf}/{sd}/{ff}/{fd}/{ab}/{sk} | {}/{}/{}/{}/{}/{}", // TODO add skipped count
        "Final Report".bold(),
        "sfile".green(),
        "sdir".green(),
        "ffile".red(),
        "fdir".red(),
        "missing".magenta(),
        "skipped".yellow(),
        sf = successful_file_count.to_string().green().bold(),
        sd = successful_dir_count.to_string().green().bold(),
        ff = failed_file_count.to_string().red().bold(),
        fd = failed_dir_count.to_string().red().bold(),
        ab = absent_count.to_string().magenta().bold(),
        sk = skipped_count.to_string().yellow()
    ));
}
