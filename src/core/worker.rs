use crate::core::runtime::MONGO_RUNTIME;
use once_cell::sync::Lazy;
use std::sync::atomic::AtomicUsize;
use std::sync::Mutex;

pub type LuaReference = i32;
pub const LUA_REGISTRYINDEX: i32 = -10000;

static CALLBACKS_PENDING: AtomicUsize = AtomicUsize::new(0);
static HOOK_REGISTERED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub static JOB_QUEUE: Lazy<tokio::sync::mpsc::UnboundedSender<Job>> = Lazy::new(|| {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    spawn_worker(rx);
    tx
});

pub static CALLBACK_QUEUE: Lazy<Mutex<(std::sync::mpsc::Sender<Job>, std::sync::mpsc::Receiver<Job>)>> =
    Lazy::new(|| Mutex::new(std::sync::mpsc::channel()));

#[derive(Debug)]
pub struct Job {
    pub operation: Operation,
    pub callback: Option<LuaReference>,
    pub result: Option<JobResult>,
}

#[derive(Debug)]
pub enum Operation {
    InsertOne {
        collection: mongodb::Collection<mongodb::bson::Document>,
        document: mongodb::bson::Document,
    },
    InsertMany {
        collection: mongodb::Collection<mongodb::bson::Document>,
        documents: Vec<mongodb::bson::Document>,
    },
    Find {
        collection: mongodb::Collection<mongodb::bson::Document>,
        filter: mongodb::bson::Document,
        limit: Option<i64>,
    },
    FindOne {
        collection: mongodb::Collection<mongodb::bson::Document>,
        filter: mongodb::bson::Document,
    },
    UpdateOne {
        collection: mongodb::Collection<mongodb::bson::Document>,
        filter: mongodb::bson::Document,
        update: mongodb::bson::Document,
        upsert: bool,
    },
    UpdateMany {
        collection: mongodb::Collection<mongodb::bson::Document>,
        filter: mongodb::bson::Document,
        update: mongodb::bson::Document,
        upsert: bool,
    },
    DeleteOne {
        collection: mongodb::Collection<mongodb::bson::Document>,
        filter: mongodb::bson::Document,
    },
    DeleteMany {
        collection: mongodb::Collection<mongodb::bson::Document>,
        filter: mongodb::bson::Document,
    },
    CountDocuments {
        collection: mongodb::Collection<mongodb::bson::Document>,
        filter: mongodb::bson::Document,
    },
    Aggregate {
        collection: mongodb::Collection<mongodb::bson::Document>,
        pipeline: Vec<mongodb::bson::Document>,
    },
}

#[derive(Debug)]
pub enum JobResult {
    InsertOne(Result<String, String>),
    InsertMany(Result<Vec<String>, String>),
    Find(Result<Vec<mongodb::bson::Document>, String>),
    FindOne(Result<Option<mongodb::bson::Document>, String>),
    UpdateOne(Result<i64, String>),
    UpdateMany(Result<i64, String>),
    DeleteOne(Result<i64, String>),
    DeleteMany(Result<i64, String>),
    CountDocuments(Result<i64, String>),
    Aggregate(Result<Vec<mongodb::bson::Document>, String>),
}

pub fn submit_job(job: Job) -> Result<(), String> {
    if job.callback.is_some() {
        CALLBACKS_PENDING.fetch_add(1, std::sync::atomic::Ordering::Release);
    }

    JOB_QUEUE.send(job)
        .map_err(|e| format!("Failed to submit job: {:?}", e))
}

async fn process_job(mut job: Job) {
    let result = match &job.operation {
        Operation::InsertOne { collection, document } => {
            let result = collection
                .insert_one(document.clone())
                .await
                .map(|r| r.inserted_id.to_string())
                .map_err(|e| e.to_string());
            JobResult::InsertOne(result)
        }
        Operation::InsertMany { collection, documents } => {
            let result = collection
                .insert_many(documents.clone())
                .await
                .map(|r| r.inserted_ids.values().map(|id| id.to_string()).collect())
                .map_err(|e| e.to_string());
            JobResult::InsertMany(result)
        }
        Operation::Find { collection, filter, limit } => {
            use futures::TryStreamExt;

            let result = async {
                let mut cursor = collection.find(filter.clone()).await
                    .map_err(|e| e.to_string())?;

                let mut documents = Vec::new();
                let mut count = 0i64;

                while let Some(doc) = cursor.try_next().await.map_err(|e| e.to_string())? {
                    documents.push(doc);
                    count += 1;

                    if let Some(limit_val) = limit {
                        if count >= *limit_val {
                            break;
                        }
                    }
                }

                Ok(documents)
            }.await;

            JobResult::Find(result)
        }
        Operation::FindOne { collection, filter } => {
            let result = collection
                .find_one(filter.clone())
                .await
                .map_err(|e| e.to_string());
            JobResult::FindOne(result)
        }
        Operation::UpdateOne { collection, filter, update, upsert } => {
            let options = mongodb::options::UpdateOptions::builder()
                .upsert(*upsert)
                .build();

            let result = collection
                .update_one(filter.clone(), update.clone())
                .with_options(options)
                .await
                .map(|r| r.modified_count as i64)
                .map_err(|e| e.to_string());
            JobResult::UpdateOne(result)
        }
        Operation::UpdateMany { collection, filter, update, upsert } => {
            let options = mongodb::options::UpdateOptions::builder()
                .upsert(*upsert)
                .build();

            let result = collection
                .update_many(filter.clone(), update.clone())
                .with_options(options)
                .await
                .map(|r| r.modified_count as i64)
                .map_err(|e| e.to_string());
            JobResult::UpdateMany(result)
        }
        Operation::DeleteOne { collection, filter } => {
            let result = collection
                .delete_one(filter.clone())
                .await
                .map(|r| r.deleted_count as i64)
                .map_err(|e| e.to_string());
            JobResult::DeleteOne(result)
        }
        Operation::DeleteMany { collection, filter } => {
            let result = collection
                .delete_many(filter.clone())
                .await
                .map(|r| r.deleted_count as i64)
                .map_err(|e| e.to_string());
            JobResult::DeleteMany(result)
        }
        Operation::CountDocuments { collection, filter } => {
            let result = collection
                .count_documents(filter.clone())
                .await
                .map(|c| c as i64)
                .map_err(|e| e.to_string());
            JobResult::CountDocuments(result)
        }
        Operation::Aggregate { collection, pipeline } => {
            use futures::TryStreamExt;

            let result = async {
                let mut cursor = collection
                    .aggregate(pipeline.clone())
                    .await
                    .map_err(|e| e.to_string())?;

                let mut documents = Vec::new();
                while let Some(doc) = cursor.try_next().await.map_err(|e| e.to_string())? {
                    documents.push(doc);
                }

                Ok(documents)
            }.await;

            JobResult::Aggregate(result)
        }
    };

    if job.callback.is_some() {
        job.result = Some(result);
        if let Ok(guard) = CALLBACK_QUEUE.lock() {
            guard.0.send(job).ok();
        }
    }
}

fn spawn_worker(mut rx: tokio::sync::mpsc::UnboundedReceiver<Job>) {
    MONGO_RUNTIME.spawn(async move {
        while let Some(job) = rx.recv().await {
            tokio::task::spawn(process_job(job));
        }
    });
}

pub fn get_callbacks_pending() -> usize {
    CALLBACKS_PENDING.load(std::sync::atomic::Ordering::Acquire)
}

pub fn decrease_callbacks_pending(count: usize) {
    CALLBACKS_PENDING.fetch_sub(count, std::sync::atomic::Ordering::Release);
}

pub fn should_register_hook() -> bool {
    !HOOK_REGISTERED.swap(true, std::sync::atomic::Ordering::AcqRel)
}

pub fn mark_hook_unregistered() {
    HOOK_REGISTERED.store(false, std::sync::atomic::Ordering::Release);
}

