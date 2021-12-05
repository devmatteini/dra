use indicatif::{ProgressBar, ProgressStyle};

// NOTE: spinner ticks and duration are from
// https://github.com/sindresorhus/cli-spinners/blob/00de8fbeee16fa49502fa4f687449f70f2c8ca2c/spinners.json#L2-L16
const TICKS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const TICK_DURATION: u64 = 80;

pub struct DownloadSpinner {
    pb: ProgressBar,
    asset_name: String,
}

impl DownloadSpinner {
    pub fn new(asset_name: &str) -> Self {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(TICKS)
                .template("{spinner:.blue} {msg}"),
        );
        pb.set_message(format!("Downloading {}", asset_name));
        Self {
            pb,
            asset_name: asset_name.to_string(),
        }
    }

    pub fn start(&self) {
        self.pb.enable_steady_tick(TICK_DURATION);
    }

    pub fn stop(&self) {
        self.pb.finish_and_clear();
        println!("Download {} completed!", &self.asset_name);
    }
}
