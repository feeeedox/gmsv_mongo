use crate::core::runtime::block_on;
use crate::error::{MongoError, MongoResult};
use mongodb::{bson::Document, Collection};

pub fn aggregate(
    collection: &Collection<Document>,
    pipeline: Vec<Document>,
) -> MongoResult<Vec<Document>> {
    let collection = collection.clone();
    block_on(async move {
        let mut cursor = collection
            .aggregate(pipeline)
            .await
            .map_err(|e| MongoError::Operation(format!("Aggregation failed: {}", e)))?;

        let mut results = Vec::new();

        use futures::TryStreamExt;
        while let Some(doc) = cursor
            .try_next()
            .await
            .map_err(|e| MongoError::Operation(format!("Cursor error: {}", e)))?
        {
            results.push(doc);
        }

        Ok(results)
    })
}

#[cfg(test)]
mod tests {
    use mongodb::bson::doc;

    #[test]
    fn test_pipeline_creation() {
        let pipeline = vec![
            doc! { "$match": { "status": "active" } },
            doc! { "$group": { "_id": "$category", "count": { "$sum": 1 } } },
        ];
        assert_eq!(pipeline.len(), 2);
    }
}
