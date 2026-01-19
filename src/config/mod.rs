use std::time::Duration;
 use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use crate::error::{ConfigError, ConfigResult};

#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub connection_string: String,
    pub app_name: Option<String>,
    pub max_pool_size: Option<u32>,
    pub min_pool_size: Option<u32>,
    pub server_selection_timeout: Duration,
    pub connect_timeout: Duration,
    pub socket_timeout: Option<Duration>,
    pub max_idle_time: Option<Duration>,
    pub retry_writes: bool,
    pub retry_reads: bool,
    pub direct_connection: bool,
    pub tls_enabled: bool,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            connection_string: String::new(),
            app_name: Some("gmsv_mongo_v2".to_string()),
            max_pool_size: Some(100),
            min_pool_size: Some(10),
            server_selection_timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            socket_timeout: None,
            max_idle_time: Some(Duration::from_secs(600)),
            retry_writes: true,
            retry_reads: true,
            direct_connection: false,
            tls_enabled: false,
        }
    }
}

impl ConnectionConfig {
    pub fn new(connection_string: impl Into<String>) -> ConfigResult<Self> {
        let conn_str = connection_string.into();

        if !conn_str.starts_with("mongodb://") && !conn_str.starts_with("mongodb+srv://") {
            return Err(ConfigError::InvalidConnectionString(
                "Connection string must start with 'mongodb://' or 'mongodb+srv://'".to_string()
            ));
        }

        Ok(Self {
            connection_string: conn_str,
            ..Default::default()
        })
    }

    pub fn with_app_name(mut self, name: impl Into<String>) -> Self {
        self.app_name = Some(name.into());
        self
    }

    pub fn with_max_pool_size(mut self, size: u32) -> Self {
        self.max_pool_size = Some(size);
        self
    }

    pub fn with_min_pool_size(mut self, size: u32) -> Self {
        self.min_pool_size = Some(size);
        self
    }

    pub fn with_server_selection_timeout(mut self, timeout: Duration) -> Self {
        self.server_selection_timeout = timeout;
        self
    }

    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    pub fn with_tls(mut self, enabled: bool) -> Self {
        self.tls_enabled = enabled;
        self
    }

    pub fn with_retry_writes(mut self, enabled: bool) -> Self {
        self.retry_writes = enabled;
        self
    }

    pub fn with_retry_reads(mut self, enabled: bool) -> Self {
        self.retry_reads = enabled;
        self
    }

    pub async fn to_client_options(&self) -> ConfigResult<ClientOptions> {
        let mut options = ClientOptions::parse(&self.connection_string)
            .await
            .map_err(|e| ConfigError::InvalidConnectionString(e.to_string()))?;

        let server_api = ServerApi::builder()
            .version(ServerApiVersion::V1)
            .build();
        options.server_api = Some(server_api);

        options.app_name = self.app_name.clone();
        options.max_pool_size = self.max_pool_size;
        options.min_pool_size = self.min_pool_size;
        options.server_selection_timeout = Some(self.server_selection_timeout);
        options.connect_timeout = Some(self.connect_timeout);
        options.max_idle_time = self.max_idle_time;
        options.retry_writes = Some(self.retry_writes);
        options.retry_reads = Some(self.retry_reads);
        options.direct_connection = Some(self.direct_connection);

        Ok(options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_connection_string() {
        let config = ConnectionConfig::new("mongodb://localhost:27017");
        assert!(config.is_ok());
    }

    #[test]
    fn test_invalid_connection_string() {
        let config = ConnectionConfig::new("invalid://localhost:27017");
        assert!(config.is_err());
    }

    #[test]
    fn test_builder_pattern() {
        let config = ConnectionConfig::new("mongodb://localhost:27017")
            .unwrap()
            .with_app_name("test_app")
            .with_max_pool_size(50)
            .with_retry_writes(false);

        assert_eq!(config.app_name, Some("test_app".to_string()));
        assert_eq!(config.max_pool_size, Some(50));
        assert_eq!(config.retry_writes, false);
    }
}
