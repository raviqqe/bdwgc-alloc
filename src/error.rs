#[derive(Debug)]
pub struct Error {
    description: &'static str,
}

impl Error {
    pub fn new(description: &'static str) -> Error {
        Error { description }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.description)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.description
    }
}
