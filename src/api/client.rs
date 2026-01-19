use rglua::lua::LuaState;
use rglua::prelude::*;
use mongodb::Client;
use crate::config::ConnectionConfig;
use crate::core::connection::MongoConnection;
use crate::utils::{write_userdata, check_string, push_error};
use log::{info, error};

#[lua_function]
pub extern "C" fn new_client(l: LuaState) -> i32 {
    unsafe {
        let connection_string = match check_string(l, 1) {
            Ok(s) => s,
            Err(e) => return push_error(l, e),
        };

        let config = match ConnectionConfig::new(&connection_string) {
            Ok(cfg) => cfg,
            Err(e) => {
                error!("Invalid connection string: {}", e);
                lua_pushnil(l);
                return 1;
            }
        };

        let connection = match MongoConnection::new(config) {
            Ok(conn) => conn,
            Err(e) => {
                error!("Failed to create connection: {}", e);
                lua_pushnil(l);
                return 1;
            }
        };

        if let Err(e) = connection.test_connection() {
            error!("Connection test failed: {}", e);
            lua_pushnil(l);
            return 1;
        }

        info!("Successfully connected to MongoDB");

        write_userdata(l, connection.client().clone());
        luaL_getmetatable(l, cstr!("MongoDBClient"));
        lua_setmetatable(l, -2);

        1
    }
}

#[lua_function]
pub unsafe fn new_client_with_options(l: LuaState) -> i32 {
    let connection_string = match check_string(l, 1) {
        Ok(s) => s,
        Err(e) => return push_error(l, e),
    };

    let mut config = match ConnectionConfig::new(&connection_string) {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Invalid connection string: {}", e);
            lua_pushnil(l);
            return 1;
        }
    };

    if lua_istable(l, 2) {
        lua_pushstring(l, cstr!("app_name"));
        lua_gettable(l, 2);
        if lua_isstring(l, -1) != 0 {
            if let Ok(app_name) = check_string(l, -1) {
                config = config.with_app_name(app_name);
            }
        }
        lua_pop(l, 1);

        lua_pushstring(l, cstr!("max_pool_size"));
        lua_gettable(l, 2);
        if lua_isnumber(l, -1) != 0 {
            let size = lua_tonumber(l, -1) as u32;
            config = config.with_max_pool_size(size);
        }
        lua_pop(l, 1);

        lua_pushstring(l, cstr!("retry_writes"));
        lua_gettable(l, 2);
        if lua_isboolean(l, -1) != false {
            let enabled = lua_toboolean(l, -1) != 0;
            config = config.with_retry_writes(enabled);
        }
        lua_pop(l, 1);
    }

    let connection = match MongoConnection::new(config) {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to create connection: {}", e);
            lua_pushnil(l);
            return 1;
        }
    };

    if let Err(e) = connection.test_connection() {
        error!("Connection test failed: {}", e);
        lua_pushnil(l);
        return 1;
    }

    info!("Successfully connected to MongoDB with custom options");

    write_userdata(l, connection.client().clone());
    luaL_getmetatable(l, cstr!("MongoDBClient"));
    lua_setmetatable(l, -2);

    1
}

#[lua_function]
pub unsafe fn list_databases(l: LuaState) -> i32 {
    use crate::utils::read_userdata;

    let client: Client = match read_userdata(l, 1) {
        Ok(c) => c,
        Err(e) => return push_error(l, e),
    };

    let databases = match crate::core::runtime::block_on(async move {
        client.list_database_names().await
    }) {
        Ok(dbs) => dbs,
        Err(e) => {
            error!("Failed to list databases: {}", e);
            lua_pushnil(l);
            return 1;
        }
    };

    lua_newtable(l);
    for (i, db_name) in databases.iter().enumerate() {
        use std::ffi::CString;
        let cstr = CString::new(db_name.as_str()).unwrap();
        lua_pushstring(l, cstr.as_ptr());
        lua_rawseti(l, -2, (i + 1) as i32);
    }

    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exists() {
        assert!(true);
    }
}
