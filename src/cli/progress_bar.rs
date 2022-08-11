use std::path::Path;

use indicatif::ProgressStyle;

use crate::cli;
use crate::cli::color::Color;

pub struct ProgressBar {
    pb: indicatif::ProgressBar,
    end_message: String,
}

impl ProgressBar {
    pub fn new(message: String, end_message: String) -> Self {
        let pb = indicatif::ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(cli::spinner::TICKS)
                .template("{spinner:.blue} {msg} {percent}% ({eta})")
                .unwrap(),
        );
        pb.set_message(message);
        Self { pb, end_message }
    }

    pub fn start(&self) {
        self.pb.enable_steady_tick(cli::spinner::TICK_DURATION);
    }

    pub fn stop(&self) {
        self.pb.finish_and_clear();
        println!("{}", &self.end_message);
    }

    pub fn set_max_progress(&self, progress: u64) {
        self.pb.set_length(progress);
    }

    pub fn update_progress(&self, progress: u64) {
        self.pb.set_position(progress);
    }

    pub fn download(download_asset: &str, output_path: &Path) -> ProgressBar {
        ProgressBar::new(
            format!("Downloading {}", Color::new(download_asset).bold()),
            format!(
                "Saved to: {}",
                Color::new(&format!("{}", output_path.display())).bold()
            ),
        )
    }
}
