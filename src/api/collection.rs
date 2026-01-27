use crate::log_info;
use crate::operations;
use crate::types::{bson_to_lua_table, lua_table_to_bson};
use crate::utils::{check_string, opt_boolean, push_error, read_userdata, write_userdata};
use log::error;
use mongodb::bson::Document;
use mongodb::{Collection, Database};
use rglua::lua::LuaState;
use rglua::prelude::*;

#[lua_function]
pub extern "C" fn get_collection(l: LuaState) -> i32 {
    unsafe {
        let database: Database = match read_userdata(l, 1) {
            Ok(db) => db,
            Err(e) => return push_error(l, e),
        };

        let collection_name = match check_string(l, 2) {
            Ok(s) => s,
            Err(e) => return push_error(l, e),
        };

        let collection: Collection<Document> = database.collection(&collection_name);

        write_userdata(l, collection);
        luaL_getmetatable(l, cstr!("MongoDBCollection"));
        lua_setmetatable(l, -2);

        1
    }
}

/// Insert a single document
#[lua_function]
pub extern "C" fn insert_one(l: LuaState) -> i32 {
    unsafe {
        let collection: Collection<Document> = match read_userdata(l, 1) {
            Ok(c) => c,
            Err(e) => return push_error(l, e),
        };

        let document = match lua_table_to_bson(l, 2) {
            Ok(doc) => doc,
            Err(e) => {
                error!("Failed to convert document: {}", e);
                lua_pushnil(l);
                return 1;
            }
        };

        match operations::insert_one(collection.clone(), document) {
            Ok(id) => {
                use std::ffi::CString;
                let cstr = CString::new(id).unwrap();
                lua_pushstring(l, cstr.as_ptr());
            }
            Err(e) => {
                error!("Failed to insert document: {}", e);
                lua_pushnil(l);
            }
        }

        1
    }
}

#[lua_function]
pub extern "C" fn insert_many(l: LuaState) -> i32 {
    unsafe {
        let collection: Collection<Document> = match read_userdata(l, 1) {
            Ok(c) => c,
            Err(e) => return push_error(l, e),
        };

        if !lua_istable(l, 2) {
            error!("Expected table of documents");
            lua_pushnil(l);
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
                    lua_pushnil(l);
                    return 1;
                }
            }

            lua_pop(l, 1);
            index += 1;
        }

        match operations::insert_many(collection.clone(), documents) {
            Ok(ids) => {
                lua_newtable(l);
                for (i, id) in ids.iter().enumerate() {
                    use std::ffi::CString;
                    let cstr = CString::new(id.as_str()).unwrap();
                    lua_pushstring(l, cstr.as_ptr());
                    lua_rawseti(l, -2, (i + 1) as i32);
                }
            }
            Err(e) => {
                error!("Failed to insert documents: {}", e);
                lua_pushnil(l);
            }
        }

        1
    }
}

#[lua_function]
pub extern "C" fn find(l: LuaState) -> i32 {
    unsafe {
        let collection: Collection<Document> = match read_userdata(l, 1) {
            Ok(c) => c,
            Err(e) => return push_error(l, e),
        };

        let filter = match lua_table_to_bson(l, 2) {
            Ok(doc) => doc,
            Err(e) => {
                error!("Failed to convert filter: {}", e);
                lua_pushnil(l);
                return 1;
            }
        };

        let limit = if lua_isnumber(l, 3) != 0 {
            Some(lua_tonumber(l, 3) as i64)
        } else {
            None
        };

        match operations::find(collection.clone(), filter, limit) {
            Ok(documents) => {
                lua_newtable(l);
                for (i, doc) in documents.iter().enumerate() {
                    bson_to_lua_table(l, doc);
                    lua_rawseti(l, -2, (i + 1) as i32);
                }
            }
            Err(e) => {
                error!("Failed to find documents: {}", e);
                lua_pushnil(l);
            }
        }

        1
    }
}

#[lua_function]
pub unsafe fn find_one(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    let filter = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert filter: {}", e);
            lua_pushnil(l);
            return 1;
        }
    };

    match operations::find_one(collection.clone(), filter) {
        Ok(Some(doc)) => {
            bson_to_lua_table(l, &doc);
        }
        Ok(None) => {
            lua_pushnil(l);
        }
        Err(e) => {
            error!("Failed to find document: {}", e);
            lua_pushnil(l);
        }
    }

    1
}

#[lua_function]
pub unsafe fn update_one(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    let filter = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert filter: {}", e);
            lua_pushnumber(l, 0.0);
            return 1;
        }
    };

    let update = match lua_table_to_bson(l, 3) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert update: {}", e);
            lua_pushnumber(l, 0.0);
            return 1;
        }
    };

    let upsert = opt_boolean(l, 4, false);

    match operations::update_one(collection.clone(), filter, update, upsert) {
        Ok(count) => {
            lua_pushnumber(l, count as f64);
        }
        Err(e) => {
            error!("Failed to update document: {}", e);
            lua_pushnumber(l, 0.0);
        }
    }

    1
}

#[lua_function]
pub unsafe fn update_many(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    let filter = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert filter: {}", e);
            lua_pushnumber(l, 0.0);
            return 1;
        }
    };

    let update = match lua_table_to_bson(l, 3) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert update: {}", e);
            lua_pushnumber(l, 0.0);
            return 1;
        }
    };

    let upsert = opt_boolean(l, 4, false);

    match operations::update_many(collection.clone(), filter, update, upsert) {
        Ok(count) => {
            lua_pushnumber(l, count as f64);
        }
        Err(e) => {
            error!("Failed to update documents: {}", e);
            lua_pushnumber(l, 0.0);
        }
    }

    1
}

#[lua_function]
pub unsafe fn delete_one(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    let filter = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert filter: {}", e);
            lua_pushnumber(l, 0.0);
            return 1;
        }
    };

    match operations::delete_one(collection.clone(), filter) {
        Ok(count) => {
            lua_pushnumber(l, count as f64);
        }
        Err(e) => {
            error!("Failed to delete document: {}", e);
            lua_pushnumber(l, 0.0);
        }
    }

    1
}

#[lua_function]
pub unsafe fn delete_many(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    let filter = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert filter: {}", e);
            lua_pushnumber(l, 0.0);
            return 1;
        }
    };

    match operations::delete_many(collection.clone(), filter) {
        Ok(count) => {
            lua_pushnumber(l, count as f64);
        }
        Err(e) => {
            error!("Failed to delete documents: {}", e);
            lua_pushnumber(l, 0.0);
        }
    }

    1
}

#[lua_function]
pub unsafe fn count_documents(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    let filter = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert filter: {}", e);
            lua_pushnumber(l, 0.0);
            return 1;
        }
    };

    match operations::count_documents(collection.clone(), filter) {
        Ok(count) => {
            lua_pushnumber(l, count as f64);
        }
        Err(e) => {
            error!("Failed to count documents: {}", e);
            lua_pushnumber(l, 0.0);
        }
    }

    1
}

#[lua_function]
pub unsafe fn aggregate(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    // Convert pipeline array
    if !lua_istable(l, 2) {
        error!("Expected table for pipeline");
        lua_pushnil(l);
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
                lua_pushnil(l);
                return 1;
            }
        }

        lua_pop(l, 1);
        index += 1;
    }

    // automatically append an _id field with a null value so developers can use "nil" as a grouping key (basically the same like null, but nil is getting removed in Lua)
    for stage in pipeline.iter_mut() {
        if let Ok(group) = stage.get_document_mut("$group") {
            if !group.contains_key("_id") {
                group.insert("_id", mongodb::bson::Bson::Null);
            }
        }
    }

    match operations::aggregate(&collection, pipeline) {
        Ok(documents) => {
            lua_newtable(l);
            for (i, doc) in documents.iter().enumerate() {
                bson_to_lua_table(l, doc);
                lua_rawseti(l, -2, (i + 1) as i32);
            }
        }
        Err(e) => {
            error!("Failed to aggregate: {}", e);
            lua_pushnil(l);
        }
    }

    1
}

#[lua_function]
pub unsafe fn create_index(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    let keys = match lua_table_to_bson(l, 2) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to convert index keys: {}", e);
            lua_pushnil(l);
            return 1;
        }
    };

    let unique = opt_boolean(l, 3, false);
    let name = if lua_isstring(l, 4) != 0 {
        check_string(l, 4).ok()
    } else {
        None
    };

    match operations::create_index(&collection, keys, unique, name) {
        Ok(index_name) => {
            use std::ffi::CString;
            let cstr = CString::new(index_name).unwrap();
            lua_pushstring(l, cstr.as_ptr());
        }
        Err(e) => {
            error!("Failed to create index: {}", e);
            lua_pushnil(l);
        }
    }

    1
}

#[lua_function]
pub unsafe fn list_indexes(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    match operations::list_indexes(&collection) {
        Ok(indexes) => {
            lua_newtable(l);
            for (i, index) in indexes.iter().enumerate() {
                bson_to_lua_table(l, index);
                lua_rawseti(l, -2, (i + 1) as i32);
            }
        }
        Err(e) => {
            error!("Failed to list indexes: {}", e);
            lua_pushnil(l);
        }
    }

    1
}

#[lua_function]
pub unsafe fn drop_index(l: LuaState) -> i32 {
    let collection: Collection<Document> = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    let index_name = match check_string(l, 2) {
        Ok(s) => s,
        Err(e) => return push_error(l, e),
    };

    match operations::drop_index(&collection, &index_name) {
        Ok(_) => {
            log_info!("Dropped index: {}", index_name);
            lua_pushboolean(l, 1);
        }
        Err(e) => {
            error!("Failed to drop index: {}", e);
            lua_pushboolean(l, 0);
        }
    }

    1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_exists() {
        assert!(true);
    }
}
