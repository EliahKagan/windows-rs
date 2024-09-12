/// A specialized [`Result`] type that provides compiler error information.
pub type Result<T> = std::result::Result<T, Error>;

/// An error object consists of both an error message and file and line information.
#[derive(Default, Debug)]
pub struct Error {
    message: String,
    path: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(fmt, "error: {}", self.message)?;
        if !self.path.is_empty() {
            writeln!(fmt, "  --> {}", self.path)?;
        }
        Ok(())
    }
}

impl Error {
    pub(crate) fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            ..Self::default()
        }
    }

    pub(crate) fn with_path(self, path: &str) -> Self {
        Self {
            path: path.to_string(),
            ..self
        }
    }
}
