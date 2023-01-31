use arboard::Clipboard;

use crate::cli::color::Color;

/// Copy text to clipboard. This operation cannot fail and will ignore errors
pub fn set_text(value: &str) {
    Clipboard::new()
        .and_then(|mut clipboard| clipboard.set_text(value))
        .unwrap_or_else(|e| {
            let message = format!("Failed to copy to clipboard, ignoring ({})", e);
            eprintln!("{}", Color::new(&message).yellow())
        });
}
