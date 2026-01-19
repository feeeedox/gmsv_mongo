/// Connection pooling and management
///
/// Manages MongoDB client connections with proper pooling and lifecycle
use mongodb::{Client, Database, Collection};
use mongodb::bson::Document;
use std::sync::Arc;
use crate::config::ConnectionConfig;
use crate::error::{MongoError, MongoResult};
use crate::core::runtime::block_on;

#[derive(Clone)]
pub struct MongoConnection {
    client: Arc<Client>,
    connection_string: String,
}

impl MongoConnection {
    pub fn new(config: ConnectionConfig) -> MongoResult<Self> {
        let connection_string = config.connection_string.clone();

        let client = block_on(async move {
            let options = config.to_client_options()
                .await
                .map_err(|e| MongoError::Connection(e.to_string()))?;

            Client::with_options(options)
                .map_err(|e| MongoError::Connection(e.to_string()))
        })?;

        Ok(Self {
            client: Arc::new(client),
            connection_string,
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

    pub fn database(&self, name: &str) -> Database {
        self.client.database(name)
    }

    pub fn collection(&self, database: &str, collection: &str) -> Collection<Document> {
        self.client.database(database).collection(collection)
    }

    pub fn list_databases(&self) -> MongoResult<Vec<String>> {
        let client = Arc::clone(&self.client);
        block_on(async move {
            client
                .list_database_names()
                .await
                .map_err(|e| MongoError::Operation(e.to_string()))
        })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn connection_string(&self) -> &str {
        &self.connection_string
    }
}

pub fn database_exists(client: &Client, db_name: &str) -> MongoResult<bool> {
    let client = client.clone();
    let db_name = db_name.to_string();
    block_on(async move {
        let names = client
            .list_database_names()
            .await
            .map_err(|e| MongoError::Operation(e.to_string()))?;
        Ok(names.contains(&db_name))
    })
}

pub fn collection_exists(database: &Database, collection_name: &str) -> MongoResult<bool> {
    let database = database.clone();
    let collection_name = collection_name.to_string();
    block_on(async move {
        let names = database
            .list_collection_names()
            .await
            .map_err(|e| MongoError::Operation(e.to_string()))?;
        Ok(names.contains(&collection_name))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_string_storage() {
        let config = ConnectionConfig::new("mongodb://localhost:27017").unwrap();
        let conn = MongoConnection::new(config);

        if let Ok(connection) = conn {
            assert!(!connection.connection_string().is_empty());
        }
    }
}
