use crate::log_info;
use crate::operations;
use crate::utils::{check_string, push_error, read_userdata, write_userdata};
use log::error;
use mongodb::{Client, Database};
use rglua::lua::LuaState;
use rglua::prelude::*;

#[lua_function]
pub extern "C" fn get_database(l: LuaState) -> i32 {
    unsafe {
        let client: Client = match read_userdata(l, 1) {
            Ok(c) => c,
            Err(e) => return push_error(l, e),
        };

        let database_name = match check_string(l, 2) {
            Ok(s) => s,
            Err(e) => return push_error(l, e),
        };

        let db = client.database(&database_name);

        write_userdata(l, db);
        luaL_getmetatable(l, cstr!("MongoDBDatabase"));
        lua_setmetatable(l, -2);

        1
    }
}

#[lua_function]
pub extern "C" fn list_collections(l: LuaState) -> i32 {
    unsafe {
        let database: Database = match read_userdata(l, 1) {
            Ok(db) => db,
            Err(e) => return push_error(l, e),
        };

        let collections = match operations::list_collections(&database) {
            Ok(cols) => cols,
            Err(e) => {
                error!("Failed to list collections: {}", e);
                lua_pushnil(l);
                return 1;
            }
        };

        lua_newtable(l);
        for (i, col_name) in collections.iter().enumerate() {
            use std::ffi::CString;
            let cstr = CString::new(col_name.as_str()).unwrap();
            lua_pushstring(l, cstr.as_ptr());
            lua_rawseti(l, -2, (i + 1) as i32);
        }

        1
    }
}

#[lua_function]
pub extern "C" fn create_collection(l: LuaState) -> i32 {
    unsafe {
        let database: Database = match read_userdata(l, 1) {
            Ok(db) => db,
            Err(e) => return push_error(l, e),
        };

        let collection_name = match check_string(l, 2) {
            Ok(s) => s,
            Err(e) => return push_error(l, e),
        };

        match operations::create_collection(&database, &collection_name) {
            Ok(_) => {
                log_info!("Created collection: {}", collection_name);
                lua_pushboolean(l, 1);
            }
            Err(e) => {
                error!("Failed to create collection: {}", e);
                lua_pushboolean(l, 0);
            }
        }

        1
    }
}

#[lua_function]
pub extern "C" fn drop_collection(l: LuaState) -> i32 {
    unsafe {
        let database: Database = match read_userdata(l, 1) {
            Ok(db) => db,
            Err(e) => return push_error(l, e),
        };

        let collection_name = match check_string(l, 2) {
            Ok(s) => s,
            Err(e) => return push_error(l, e),
        };

        match operations::drop_collection(&database, &collection_name) {
            Ok(_) => {
                log_info!("Dropped collection: {}", collection_name);
                lua_pushboolean(l, 1);
            }
            Err(e) => {
                error!("Failed to drop collection: {}", e);
                lua_pushboolean(l, 0);
            }
        }

        1
    }
}

#[lua_function]
pub extern "C" fn collection_stats(l: LuaState) -> i32 {
    unsafe {
        let database: Database = match read_userdata(l, 1) {
            Ok(db) => db,
            Err(e) => return push_error(l, e),
        };

        let collection_name = match check_string(l, 2) {
            Ok(s) => s,
            Err(e) => return push_error(l, e),
        };

        match operations::collection_stats(&database, &collection_name) {
            Ok(stats) => {
                crate::types::bson_to_lua_table(l, &stats);
            }
            Err(e) => {
                error!("Failed to get collection stats: {}", e);
                lua_pushnil(l);
            }
        }

        1
    }
}

#[lua_function]
pub extern "C" fn drop_database(l: LuaState) -> i32 {
    unsafe {
        let database: Database = match read_userdata(l, 1) {
            Ok(db) => db,
            Err(e) => return push_error(l, e),
        };

        match operations::drop_database(&database) {
            Ok(_) => {
                log_info!("Dropped database: {}", database.name());
                lua_pushboolean(l, 1);
            }
            Err(e) => {
                error!("Failed to drop database: {}", e);
                lua_pushboolean(l, 0);
            }
        }

        1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_exists() {
        assert!(true);
    }
}
