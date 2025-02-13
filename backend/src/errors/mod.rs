mod api_error;

pub use api_error::ApiError;

pub type Result<T> = std::result::Result<T, ApiError>;
