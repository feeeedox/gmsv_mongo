use crate::core::runtime::block_on;
use crate::error::{MongoError, MongoResult};
use mongodb::{bson::Document, Collection};

pub fn aggregate(collection: &Collection<Document>, pipeline: Vec<Document>) -> MongoResult<Vec<Document>> {
    let collection = collection.clone();
    block_on(async move {
        let mut cursor = collection
            .aggregate(pipeline)
            .await
            .map_err(|e| MongoError::Operation(format!("Aggregation failed: {}", e)))?;

        let mut results = Vec::new();

        use futures::TryStreamExt;
        while let Some(doc) = cursor.try_next().await
            .map_err(|e| MongoError::Operation(format!("Cursor error: {}", e)))? {
            results.push(doc);
        }

        Ok(results)
    })
}

pub fn count_aggregate(collection: &Collection<Document>, filter: Document) -> MongoResult<i64> {
    let collection = collection.clone();
    let pipeline = vec![
        mongodb::bson::doc! { "$match": filter },
        mongodb::bson::doc! { "$count": "total" },
    ];

    block_on(async move {
        let mut cursor = collection
            .aggregate(pipeline)
            .await
            .map_err(|e| MongoError::Operation(format!("Count aggregation failed: {}", e)))?;

        use futures::TryStreamExt;
        if let Some(doc) = cursor.try_next().await
            .map_err(|e| MongoError::Operation(format!("Cursor error: {}", e)))? {
            if let Ok(count) = doc.get_i64("total") {
                return Ok(count);
            }
        }

        Ok(0)
    })
}

pub fn group_by(collection: &Collection<Document>, group_field: &str, accumulator_field: Option<&str>) -> MongoResult<Vec<Document>> {
    let accumulator = if let Some(field) = accumulator_field {
        mongodb::bson::doc! { "sum": { "$sum": format!("${}", field) } }
    } else {
        mongodb::bson::doc! { "count": { "$sum": 1 } }
    };

    let pipeline = vec![
        mongodb::bson::doc! {
            "$group": {
                "_id": format!("${}", group_field),
                "values": accumulator
            }
        }
    ];

    aggregate(collection, pipeline)
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
