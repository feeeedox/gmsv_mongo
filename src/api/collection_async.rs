use crate::api::callbacks::listen;
use crate::core::worker::{should_register_hook, submit_job, Job, Operation, LUA_REGISTRYINDEX};
use crate::types::lua_table_to_bson;
use crate::utils::{push_error, read_userdata};
use log::error;
use mongodb::{bson::Document, Collection};
use rglua::lua::LuaState;
use rglua::prelude::*;

fn maybe_register_hook(l: LuaState) {
    if should_register_hook() {
        unsafe { listen(l); }
    }
}

/// Async version of insert_one with callback
#[lua_function]
pub extern "C" fn insert_one_async(l: LuaState) -> i32 {
    unsafe {
        let collection: Collection<Document> = match read_userdata(l, 1) {
            Ok(c) => c,
            Err(e) => return push_error(l, e),
        };

        let document = match lua_table_to_bson(l, 2) {
            Ok(doc) => doc,
            Err(e) => {
                error!("Failed to convert document: {}", e);
                lua_pushboolean(l, 0);
                return 1;
            }
        };

        let callback = if lua_isfunction(l, 3) {
            maybe_register_hook(l);
            lua_pushvalue(l, 3);
            Some(luaL_ref(l, LUA_REGISTRYINDEX))
        } else {
            None
        };

        let job = Job {
            operation: Operation::InsertOne {
                collection: collection.clone(),
                document,
            },
            callback,
            result: None,
        };

        match submit_job(job) {
            Ok(_) => lua_pushboolean(l, 1),
            Err(e) => {
                error!("Failed to submit job: {}", e);
                lua_pushboolean(l, 0);
            }
        }

        1
    }
}

/// Async version of insert_many with callback
#[lua_function]
pub extern "C" fn insert_many_async(l: LuaState) -> i32 {
    unsafe {
        let collection: Collection<Document> = match read_userdata(l, 1) {
            Ok(c) => c,
            Err(e) => return push_error(l, e),
        };

        if !lua_istable(l, 2) {
            error!("Expected table of documents");
            lua_pushboolean(l, 0);
            return 1;
        }

        let mut documents = Vec::new();
        let mut index = 1;

        loop {
            lua_rawgeti(l, 2, index);
            if lua_isnil(l, -1) {
                lua_pop(l, 1);
                break;
            }

            match lua_table_to_bson(l, -1) {
                Ok(doc) => documents.push(doc),
                Err(e) => {
                    error!("Failed to convert document at index {}: {}", index, e);
                    lua_pop(l, 1);
                    lua_pushboolean(l, 0);
                    return 1;
                }
            }

            lua_pop(l, 1);
            index += 1;
        }

        let callback = if lua_isfunction(l, 3) {
            maybe_register_hook(l);
            lua_pushvalue(l, 3);
            Some(luaL_ref(l, LUA_REGISTRYINDEX))
        } else {
            None
        };

        let job = Job {
            operation: Operation::InsertMany {
                collection: collection.clone(),
                documents,
            },
            callback,
            result: None,
        };

        match submit_job(job) {
            Ok(_) => lua_pushboolean(l, 1),
            Err(e) => {
                error!("Failed to submit job: {}", e);
                lua_pushboolean(l, 0);
            }
        }

        1
    }
}

/// Async version of find with callback
#[lua_function]
pub extern "C" fn find_async(l: LuaState) -> i32 {
    unsafe {
        let collection: Collection<Document> = match read_userdata(l, 1) {
            Ok(c) => c,
            Err(e) => return push_error(l, e),
        };

        let filter = match lua_table_to_bson(l, 2) {
            Ok(doc) => doc,
            Err(e) => {
                error!("Failed to convert filter: {}", e);
                lua_pushboolean(l, 0);
                return 1;
            }
        };

        let limit = if lua_isnumber(l, 3) != 0 {
            Some(lua_tonumber(l, 3) as i64)
        } else {
            None
        };

        let callback = if lua_isfunction(l, 4) {
            maybe_register_hook(l);
            lua_pushvalue(l, 4);
            Some(luaL_ref(l, LUA_REGISTRYINDEX))
        } else {
            None
        };

        let job = Job {
            operation: Operation::Find {
                collection: collection.clone(),
                filter,
                limit,
            },
            callback,
            result: None,
        };

        match submit_job(job) {
            Ok(_) => lua_pushboolean(l, 1),
            Err(e) => {
                error!("Failed to submit job: {}", e);
                lua_pushboolean(l, 0);
            }
        }

        1
    }
}

/// Async version of find_one with callback
#[lua_function]
pub unsafe fn find_one_async(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    let filter = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert filter: {}", e);
            lua_pushboolean(l, 0);
            return 1;
        }
    };

    let callback = if lua_isfunction(l, 3) {
        maybe_register_hook(l);
        lua_pushvalue(l, 3);
        Some(luaL_ref(l, LUA_REGISTRYINDEX))
    } else {
        None
    };

    let job = Job {
        operation: Operation::FindOne {
            collection: collection.clone(),
            filter,
        },
        callback,
        result: None,
    };

    match submit_job(job) {
        Ok(_) => lua_pushboolean(l, 1),
        Err(e) => {
            error!("Failed to submit job: {}", e);
            lua_pushboolean(l, 0);
        }
    }

    1
}

/// Async version of update_one with callback
#[lua_function]
pub unsafe fn update_one_async(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    let filter = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert filter: {}", e);
            lua_pushboolean(l, 0);
            return 1;
        }
    };

    let update = match lua_table_to_bson(l, 3) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert update: {}", e);
            lua_pushboolean(l, 0);
            return 1;
        }
    };

    let upsert = if lua_isboolean(l, 4) {
        lua_toboolean(l, 4) != 0
    } else {
        false
    };

    let callback = if lua_isfunction(l, 5) {
        maybe_register_hook(l);
        lua_pushvalue(l, 5);
        Some(luaL_ref(l, LUA_REGISTRYINDEX))
    } else {
        None
    };

    let job = Job {
        operation: Operation::UpdateOne {
            collection: collection.clone(),
            filter,
            update,
            upsert,
        },
        callback,
        result: None,
    };

    match submit_job(job) {
        Ok(_) => lua_pushboolean(l, 1),
        Err(e) => {
            error!("Failed to submit job: {}", e);
            lua_pushboolean(l, 0);
        }
    }

    1
}

/// Async version of update_many with callback
#[lua_function]
pub unsafe fn update_many_async(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    let filter = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert filter: {}", e);
            lua_pushboolean(l, 0);
            return 1;
        }
    };

    let update = match lua_table_to_bson(l, 3) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert update: {}", e);
            lua_pushboolean(l, 0);
            return 1;
        }
    };

    let upsert = if lua_isboolean(l, 4) {
        lua_toboolean(l, 4) != 0
    } else {
        false
    };

    let callback = if lua_isfunction(l, 5) {
        maybe_register_hook(l);
        lua_pushvalue(l, 5);
        Some(luaL_ref(l, LUA_REGISTRYINDEX))
    } else {
        None
    };

    let job = Job {
        operation: Operation::UpdateMany {
            collection: collection.clone(),
            filter,
            update,
            upsert,
        },
        callback,
        result: None,
    };

    match submit_job(job) {
        Ok(_) => lua_pushboolean(l, 1),
        Err(e) => {
            error!("Failed to submit job: {}", e);
            lua_pushboolean(l, 0);
        }
    }

    1
}

/// Async version of delete_one with callback
#[lua_function]
pub unsafe fn delete_one_async(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    let filter = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert filter: {}", e);
            lua_pushboolean(l, 0);
            return 1;
        }
    };

    let callback = if lua_isfunction(l, 3) {
        maybe_register_hook(l);
        lua_pushvalue(l, 3);
        Some(luaL_ref(l, LUA_REGISTRYINDEX))
    } else {
        None
    };

    let job = Job {
        operation: Operation::DeleteOne {
            collection: collection.clone(),
            filter,
        },
        callback,
        result: None,
    };

    match submit_job(job) {
        Ok(_) => lua_pushboolean(l, 1),
        Err(e) => {
            error!("Failed to submit job: {}", e);
            lua_pushboolean(l, 0);
        }
    }

    1
}

/// Async version of delete_many with callback
#[lua_function]
pub unsafe fn delete_many_async(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    let filter = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert filter: {}", e);
            lua_pushboolean(l, 0);
            return 1;
        }
    };

    let callback = if lua_isfunction(l, 3) {
        maybe_register_hook(l);
        lua_pushvalue(l, 3);
        Some(luaL_ref(l, LUA_REGISTRYINDEX))
    } else {
        None
    };

    let job = Job {
        operation: Operation::DeleteMany {
            collection: collection.clone(),
            filter,
        },
        callback,
        result: None,
    };

    match submit_job(job) {
        Ok(_) => lua_pushboolean(l, 1),
        Err(e) => {
            error!("Failed to submit job: {}", e);
            lua_pushboolean(l, 0);
        }
    }

    1
}

/// Async version of count_documents with callback
#[lua_function]
pub unsafe fn count_documents_async(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    let filter = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert filter: {}", e);
            lua_pushboolean(l, 0);
            return 1;
        }
    };

    let callback = if lua_isfunction(l, 3) {
        maybe_register_hook(l);
        lua_pushvalue(l, 3);
        Some(luaL_ref(l, LUA_REGISTRYINDEX))
    } else {
        None
    };

    let job = Job {
        operation: Operation::CountDocuments {
            collection: collection.clone(),
            filter,
        },
        callback,
        result: None,
    };

    match submit_job(job) {
        Ok(_) => lua_pushboolean(l, 1),
        Err(e) => {
            error!("Failed to submit job: {}", e);
            lua_pushboolean(l, 0);
        }
    }

    1
}

/// Async version of aggregate with callback
#[lua_function]
pub unsafe fn aggregate_async(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    // Convert pipeline array
    if !lua_istable(l, 2) {
        error!("Expected table for pipeline");
        lua_pushboolean(l, 0);
        return 1;
    }

    let mut pipeline = Vec::new();
    let mut index = 1;

    loop {
        lua_rawgeti(l, 2, index);
        if lua_isnil(l, -1) {
            lua_pop(l, 1);
            break;
        }

        match lua_table_to_bson(l, -1) {
            Ok(stage) => pipeline.push(stage),
            Err(e) => {
                error!("Failed to convert pipeline stage {}: {}", index, e);
                lua_pop(l, 1);
                lua_pushboolean(l, 0);
                return 1;
            }
        }

        lua_pop(l, 1);
        index += 1;
    }

    // automatically append an _id field with a null value so developers can use "nil" as a grouping key
    for stage in pipeline.iter_mut() {
        if let Ok(group) = stage.get_document_mut("$group") {
            if !group.contains_key("_id") {
                group.insert("_id", mongodb::bson::Bson::Null);
            }
        }
    }

    let callback = if lua_isfunction(l, 3) {
        maybe_register_hook(l);
        lua_pushvalue(l, 3);
        Some(luaL_ref(l, LUA_REGISTRYINDEX))
    } else {
        None
    };

    let job = Job {
        operation: Operation::Aggregate {
            collection: collection.clone(),
            pipeline,
        },
        callback,
        result: None,
    };

    match submit_job(job) {
        Ok(_) => lua_pushboolean(l, 1),
        Err(e) => {
            error!("Failed to submit job: {}", e);
            lua_pushboolean(l, 0);
        }
    }

    1
}
