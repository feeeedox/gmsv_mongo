use crate::core::runtime::block_on;
use crate::error::{MongoError, MongoResult};
use mongodb::{bson::Document, Collection, IndexModel};

pub fn create_index(collection: &Collection<Document>, keys: Document, unique: bool, name: Option<String>) -> MongoResult<String> {
    let collection = collection.clone();
    block_on(async move {
        let mut options = mongodb::options::IndexOptions::default();
        options.unique = Some(unique);
        options.name = name;

        let index = IndexModel::builder()
            .keys(keys)
            .options(options)
            .build();

        let result = collection
            .create_index(index)
            .await
            .map_err(|e| MongoError::IndexError(format!("Create index failed: {}", e)))?;

        Ok(result.index_name)
    })
}

pub fn create_indexes(collection: &Collection<Document>, indexes: Vec<(Document, bool, Option<String>)>) -> MongoResult<Vec<String>> {
    let collection = collection.clone();
    block_on(async move {
        let index_models: Vec<IndexModel> = indexes.into_iter()
            .map(|(keys, unique, name)| {
                let mut options = mongodb::options::IndexOptions::default();
                options.unique = Some(unique);
                options.name = name;

                IndexModel::builder()
                    .keys(keys)
                    .options(options)
                    .build()
            })
            .collect();

        let result = collection
            .create_indexes(index_models)
            .await
            .map_err(|e| MongoError::IndexError(format!("Create indexes failed: {}", e)))?;

        Ok(result.index_names)
    })
}

pub fn list_indexes(collection: &Collection<Document>) -> MongoResult<Vec<Document>> {
    let collection = collection.clone();
    block_on(async move {
        let mut cursor = collection
            .list_indexes()
            .await
            .map_err(|e| MongoError::IndexError(format!("List indexes failed: {}", e)))?;

        let mut indexes = Vec::new();

        use futures::TryStreamExt;
        while let Some(index) = cursor.try_next().await
            .map_err(|e| MongoError::IndexError(format!("Cursor error: {}", e)))? {
            if let Ok(doc) = mongodb::bson::to_document(&index) {
                indexes.push(doc);
            }
        }

        Ok(indexes)
    })
}

pub fn drop_index(collection: &Collection<Document>, index_name: &str) -> MongoResult<()> {
    let collection = collection.clone();
    let index_name = index_name.to_string();
    block_on(async move {
        collection
            .drop_index(index_name)
            .await
            .map_err(|e| MongoError::IndexError(format!("Drop index failed: {}", e)))?;
        Ok(())
    })
}

pub fn drop_all_indexes(collection: &Collection<Document>) -> MongoResult<()> {
    let collection = collection.clone();
    block_on(async move {
        collection
            .drop_indexes()
            .await
            .map_err(|e| MongoError::IndexError(format!("Drop all indexes failed: {}", e)))?;
        Ok(())
    })
}

pub fn create_text_index(collection: &Collection<Document>, fields: Vec<String>, name: Option<String>) -> MongoResult<String> {
    let mut keys_doc = Document::new();
    for field in fields {
        keys_doc.insert(field, "text");
    }

    create_index(collection, keys_doc, false, name)
}

#[cfg(test)]
mod tests {
    use mongodb::bson::doc;

    #[test]
    fn test_index_keys_creation() {
        let keys = doc! { "username": 1 };
        assert!(keys.contains_key("username"));
    }
}
