use crate::config::ConnectionConfig;
use crate::core::runtime::block_on;
use crate::error::{MongoError, MongoResult};
/// Connection pooling and management
///
/// Manages MongoDB client connections with proper pooling and lifecycle
use mongodb::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct MongoConnection {
    client: Arc<Client>,
}

impl MongoConnection {
    pub fn new(config: ConnectionConfig) -> MongoResult<Self> {
        let client = block_on(async move {
            let options = config
                .to_client_options()
                .await
                .map_err(|e| MongoError::Connection(e.to_string()))?;

            Client::with_options(options).map_err(|e| MongoError::Connection(e.to_string()))
        })?;

        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub fn test_connection(&self) -> MongoResult<()> {
        let client = Arc::clone(&self.client);
        block_on(async move {
            client
                .database("admin")
                .run_command(mongodb::bson::doc! {"ping": 1})
                .await
                .map_err(|e| MongoError::Connection(format!("Ping failed: {}", e)))?;
            Ok(())
        })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
