pub mod download;

pub enum HandlerError {
    Default(String),
}

pub type HandlerResult = Result<(), HandlerError>;

impl HandlerError {
    pub fn new(message: String) -> Self {
        Self::Default(message)
    }
}
