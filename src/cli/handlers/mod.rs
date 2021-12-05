pub mod download;

pub enum HandlerError {
    Default(String),
    OperationCancelled(String),
}

pub type HandlerResult = Result<(), HandlerError>;

impl HandlerError {
    pub fn new(message: String) -> Self {
        Self::Default(message)
    }

    pub fn op_cancelled(message: String) -> Self {
        Self::OperationCancelled(message)
    }
}
