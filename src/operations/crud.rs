use mongodb::{Collection, bson::Document};
use crate::core::runtime::block_on;
use crate::error::{MongoError, MongoResult};

pub fn insert_one(collection: Collection<Document>, document: Document) -> MongoResult<String> {
    block_on(async move {
        let result = collection
            .insert_one(document)
            .await
            .map_err(|e| MongoError::Operation(format!("Insert failed: {}", e)))?;

        Ok(result.inserted_id.to_string())
    })
}

pub fn insert_many(collection: Collection<Document>, documents: Vec<Document>) -> MongoResult<Vec<String>> {
    block_on(async move {
        let result = collection
            .insert_many(documents)
            .await
            .map_err(|e| MongoError::Operation(format!("Insert many failed: {}", e)))?;

        Ok(result.inserted_ids
            .values()
            .map(|id| id.to_string())
            .collect())
    })
}

pub fn find(collection: Collection<Document>, filter: Document, limit: Option<i64>) -> MongoResult<Vec<Document>> {
    block_on(async move {
        let mut cursor = collection
            .find(filter)
            .await
            .map_err(|e| MongoError::Operation(format!("Find failed: {}", e)))?;

        let mut documents = Vec::new();
        let mut count = 0i64;

        use futures::TryStreamExt;
        while let Some(doc) = cursor.try_next().await
            .map_err(|e| MongoError::Operation(format!("Cursor error: {}", e)))? {
            documents.push(doc);
            count += 1;

            if let Some(limit_val) = limit {
                if count >= limit_val {
                    break;
                }
            }
        }

        Ok(documents)
    })
}

pub fn find_one(collection: Collection<Document>, filter: Document) -> MongoResult<Option<Document>> {
    block_on(async move {
        collection
            .find_one(filter)
            .await
            .map_err(|e| MongoError::Operation(format!("Find one failed: {}", e)))
    })
}

pub fn update_one(collection: Collection<Document>, filter: Document, update: Document, upsert: bool) -> MongoResult<i64> {
    block_on(async move {
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(upsert)
            .build();

        let result = collection
            .update_one(filter, update)
            .with_options(options)
            .await
            .map_err(|e| MongoError::Operation(format!("Update failed: {}", e)))?;

        Ok(result.modified_count as i64)
    })
}

pub fn update_many(collection: Collection<Document>, filter: Document, update: Document, upsert: bool) -> MongoResult<i64> {
    block_on(async move {
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(upsert)
            .build();

        let result = collection
            .update_many(filter, update)
            .with_options(options)
            .await
            .map_err(|e| MongoError::Operation(format!("Update many failed: {}", e)))?;

        Ok(result.modified_count as i64)
    })
}

pub fn replace_one(collection: Collection<Document>, filter: Document, replacement: Document, upsert: bool) -> MongoResult<i64> {
    block_on(async move {
        let options = mongodb::options::ReplaceOptions::builder()
            .upsert(upsert)
            .build();

        let result = collection
            .replace_one(filter, replacement)
            .with_options(options)
            .await
            .map_err(|e| MongoError::Operation(format!("Replace failed: {}", e)))?;

        Ok(result.modified_count as i64)
    })
}

pub fn delete_one(collection: Collection<Document>, filter: Document) -> MongoResult<i64> {
    block_on(async move {
        let result = collection
            .delete_one(filter)
            .await
            .map_err(|e| MongoError::Operation(format!("Delete failed: {}", e)))?;

        Ok(result.deleted_count as i64)
    })
}

pub fn delete_many(collection: Collection<Document>, filter: Document) -> MongoResult<i64> {
    block_on(async move {
        let result = collection
            .delete_many(filter)
            .await
            .map_err(|e| MongoError::Operation(format!("Delete many failed: {}", e)))?;

        Ok(result.deleted_count as i64)
    })
}

pub fn count_documents(collection: Collection<Document>, filter: Document) -> MongoResult<i64> {
    block_on(async move {
        let count = collection
            .count_documents(filter)
            .await
            .map_err(|e| MongoError::Operation(format!("Count failed: {}", e)))?;

        Ok(count as i64)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::doc;

    #[test]
    fn test_document_creation() {
        let doc = doc! {
            "name": "test",
            "value": 42
        };
        assert_eq!(doc.len(), 2);
    }
}
