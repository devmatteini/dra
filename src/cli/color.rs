use dialoguer::console::{StyledObject, style};
use std::fmt::{Display, Formatter};

pub struct Color<'a> {
    inner: StyledObject<&'a str>,
}

impl<'a> Color<'a> {
    pub fn new(value: &'a str) -> Self {
        Color {
            inner: style(value),
        }
    }

    pub fn bold(self) -> Self {
        Color {
            inner: self.inner.bold(),
        }
    }

    pub fn red(self) -> Self {
        Color {
            inner: self.inner.red(),
        }
    }

    pub fn green(self) -> Self {
        Color {
            inner: self.inner.green(),
        }
    }
}

impl Display for Color<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.inner))
    }
}
