mod common;
pub mod completion;
pub mod download;
pub mod untag;

pub enum HandlerError {
    Default(String),
    OperationCancelled(String),
}

pub type HandlerResult = Result<(), HandlerError>;

impl HandlerError {
    pub fn new(message: String) -> Self {
        Self::Default(message)
    }

    pub fn op_cancelled(message: &str) -> Self {
        Self::OperationCancelled(message.to_string())
    }
}
