use thiserror::Error;

#[derive(Error, Debug)]
pub enum MongoError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Database operation failed: {0}")]
    Operation(String),

    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    #[error("Collection not found: {0}")]
    CollectionNotFound(String),

    #[error("Database not found: {0}")]
    DatabaseNotFound(String),

    #[error("Document validation failed: {0}")]
    ValidationError(String),

    #[error("Index error: {0}")]
    IndexError(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),

    #[error("Timeout: operation took longer than {0}s")]
    Timeout(u64),

    #[error("MongoDB driver error: {0}")]
    DriverError(#[from] mongodb::error::Error),
}

#[derive(Error, Debug)]
pub enum LuaError {
    #[error("Invalid argument at position {position}: {message}")]
    InvalidArgument { position: usize, message: String },

    #[error("Type conversion error: expected {expected}, got {actual}")]
    TypeConversion { expected: String, actual: String },

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Lua table conversion error: {0}")]
    TableConversion(String),

    #[error("Invalid userdata: {0}")]
    InvalidUserdata(String),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid connection string: {0}")]
    InvalidConnectionString(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Missing required configuration: {0}")]
    MissingConfig(String),
}

/// Result type alias for MongoDB operations
pub type MongoResult<T> = Result<T, MongoError>;

/// Result type alias for Lua operations
pub type LuaResult<T> = Result<T, LuaError>;

/// Result type alias for configuration operations
pub type ConfigResult<T> = Result<T, ConfigError>;

/// Convert any error to a C-compatible string for Lua error reporting
pub fn to_lua_error_string(error: impl std::fmt::Display) -> String {
    format!("MongoDB Error: {}", error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = MongoError::Connection("Failed to connect".to_string());
        assert_eq!(err.to_string(), "Connection error: Failed to connect");
    }

    #[test]
    fn test_lua_error_display() {
        let err = LuaError::InvalidArgument {
            position: 1,
            message: "Expected string".to_string(),
        };
        assert!(err.to_string().contains("position 1"));
    }
}
