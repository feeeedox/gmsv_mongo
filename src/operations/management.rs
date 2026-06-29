use crate::core::runtime::block_on;
use crate::error::{MongoError, MongoResult};
use mongodb::{bson::Document, Database};

pub fn create_collection(database: &Database, name: &str) -> MongoResult<()> {
    let database = database.clone();
    let name = name.to_string();
    block_on(async move {
        database
            .create_collection(name)
            .await
            .map_err(|e| MongoError::Operation(format!("Create collection failed: {}", e)))?;
        Ok(())
    })
}

pub fn drop_collection(database: &Database, name: &str) -> MongoResult<()> {
    let database = database.clone();
    let name = name.to_string();
    block_on(async move {
        let collection: mongodb::Collection<Document> = database.collection(&name);
        collection
            .drop()
            .await
            .map_err(|e| MongoError::Operation(format!("Drop collection failed: {}", e)))?;
        Ok(())
    })
}

pub fn list_collections(database: &Database) -> MongoResult<Vec<String>> {
    let database = database.clone();
    block_on(async move {
        database
            .list_collection_names()
            .await
            .map_err(|e| MongoError::Operation(format!("List collections failed: {}", e)))
    })
}

pub fn collection_stats(database: &Database, collection_name: &str) -> MongoResult<Document> {
    let database = database.clone();
    let collection_name = collection_name.to_string();
    block_on(async move {
        let cmd = mongodb::bson::doc! {
            "collStats": collection_name,
        };

        database
            .run_command(cmd)
            .await
            .map_err(|e| MongoError::Operation(format!("Get collection stats failed: {}", e)))
    })
}

pub fn drop_database(database: &Database) -> MongoResult<()> {
    let database = database.clone();
    block_on(async move {
        database
            .drop()
            .await
            .map_err(|e| MongoError::Operation(format!("Drop database failed: {}", e)))?;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_collection_name_validation() {
        let name = "test_collection";
        assert!(!name.is_empty());
        assert!(!name.contains('$'));
    }
}
