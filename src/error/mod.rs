use thiserror::Error;

#[derive(Error, Debug)]
pub enum MongoError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Database operation failed: {0}")]
    Operation(String),

    #[error("Index error: {0}")]
    IndexError(String),

    #[error("MongoDB driver error: {0}")]
    DriverError(#[from] mongodb::error::Error),
}

#[derive(Error, Debug)]
pub enum LuaError {
    #[error("Invalid argument at position {position}: {message}")]
    InvalidArgument { position: usize, message: String },

    #[error("Type conversion error: expected {expected}, got {actual}")]
    TypeConversion { expected: String, actual: String },

    #[error("Lua table conversion error: {0}")]
    TableConversion(String),

    #[error("Invalid userdata: {0}")]
    InvalidUserdata(String),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid connection string: {0}")]
    InvalidConnectionString(String),
}

/// Result type alias for MongoDB operations
pub type MongoResult<T> = Result<T, MongoError>;

/// Result type alias for Lua operations
pub type LuaResult<T> = Result<T, LuaError>;

/// Result type alias for configuration operations
pub type ConfigResult<T> = Result<T, ConfigError>;

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
