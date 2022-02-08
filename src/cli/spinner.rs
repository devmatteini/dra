use crate::cli::color::Color;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

// NOTE: spinner ticks and duration are from
// https://github.com/sindresorhus/cli-spinners/blob/00de8fbeee16fa49502fa4f687449f70f2c8ca2c/spinners.json#L2-L16
const TICKS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const TICK_DURATION: u64 = 80;

pub struct Spinner {
    pb: ProgressBar,
    end_message: String,
}

impl Spinner {
    pub fn new(message: String, end_message: String) -> Self {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(TICKS)
                .template("{spinner:.blue} {msg}"),
        );
        pb.set_message(message);
        Self { pb, end_message }
    }

    pub fn start(&self) {
        self.pb.enable_steady_tick(TICK_DURATION);
    }

    pub fn stop(&self) {
        self.pb.finish_and_clear();
        println!("{}", &self.end_message);
    }

    pub fn download(download_asset: &str, output_path: &Path) -> Spinner {
        Spinner::new(
            format!("Downloading {}", Color::new(download_asset).bold()),
            format!(
                "Saved to: {}",
                Color::new(&format!("{}", output_path.display())).bold()
            ),
        )
    }
}
