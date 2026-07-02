use std::error::Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SmithError {
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    #[error("UI element not found or selector invalid")]
    ElementNotFound,

    #[error("Operation cancelled by user")]
    Cancelled,

    #[error("Context error: {0}")]
    ContextError(String),

    #[error("Platform error: {0}")]
    PlatformError(String),

    /// Platform error with full error chain preserved.
    #[error("Platform error: {message}")]
    PlatformWithCause {
        message: String,
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type SmithResult<T> = Result<T, SmithError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_params_display() {
        let err = SmithError::InvalidParams("test message".into());
        assert_eq!(err.to_string(), "Invalid parameters: test message");
    }

    #[test]
    fn test_element_not_found_display() {
        let err = SmithError::ElementNotFound;
        assert_eq!(err.to_string(), "UI element not found or selector invalid");
    }

    #[test]
    fn test_cancelled_display() {
        let err = SmithError::Cancelled;
        assert_eq!(err.to_string(), "Operation cancelled by user");
    }

    #[test]
    fn test_context_error_display() {
        let err = SmithError::ContextError("something went wrong".into());
        assert_eq!(err.to_string(), "Context error: something went wrong");
    }

    #[test]
    fn test_platform_error_display() {
        let err = SmithError::PlatformError("access denied".into());
        assert_eq!(err.to_string(), "Platform error: access denied");
    }

    #[test]
    fn test_conversion_from_anyhow_error() {
        let underlying = anyhow::anyhow!("oops");
        let err: SmithError = underlying.into();
        assert!(matches!(err, SmithError::Other(_)));
        assert_eq!(err.to_string(), "oops");
    }
}
