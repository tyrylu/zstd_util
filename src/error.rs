#[derive(Debug, thiserror::Error)]
/// The various error conditions which can occur.
pub enum Error {
    /// The ZSTD library returned an error.
    #[error("The Zstd library returned an error: {message}")]
    ZSTDError {
        message: &'static str,
        error_code: usize,
    },
}

impl From<usize> for Error {
    fn from(val: usize) -> Error {
        Error::ZSTDError {
            message: zstd_safe::get_error_name(val),
            error_code: val,
        }
    }
}
