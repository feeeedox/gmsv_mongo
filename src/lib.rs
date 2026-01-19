#[macro_use]
extern crate rglua;

use log::{info, LevelFilter};
use rglua::lua::LuaState;
use rglua::prelude::*;

mod core;
mod config;
mod error;
mod types;
mod operations;
mod api;
mod utils;
mod updatecheck;

use std::sync::atomic::{AtomicBool, Ordering};

static SUPPRESS_MESSAGES: AtomicBool = AtomicBool::new(false);

fn init_logging() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .format_timestamp(Some(env_logger::fmt::TimestampPrecision::Millis))
        .try_init()
        .ok();
}

#[gmod_open]
unsafe fn open(l: LuaState) -> i32 {
    init_logging();

    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");

    info!("  Loading {} v{}...", name, version);

    // Initialize async worker
    use once_cell::sync::Lazy;
    Lazy::force(&core::worker::JOB_QUEUE);
    Lazy::force(&core::worker::CALLBACK_QUEUE);

    if let Err(e) = updatecheck::check_latest_version() {
        log::warn!("Failed to check for updates: {}", e);
    }

    luaL_newmetatable(l, cstr!("MongoDBClient"));
    lua_pushvalue(l, -1);
    lua_setfield(l, -2, cstr!("__index"));
    lua_pushcfunction(l, api::get_database as LuaCFunction);
    lua_setfield(l, -2, cstr!("Database"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::list_databases) });
    lua_setfield(l, -2, cstr!("ListDatabases"));
    lua_pop(l, 1);

    // Register MongoDBDatabase metatable
    luaL_newmetatable(l, cstr!("MongoDBDatabase"));
    lua_pushvalue(l, -1);
    lua_setfield(l, -2, cstr!("__index"));
    lua_pushcfunction(l, api::get_collection as LuaCFunction);
    lua_setfield(l, -2, cstr!("Collection"));
    lua_pushcfunction(l, api::list_collections as LuaCFunction);
    lua_setfield(l, -2, cstr!("ListCollections"));
    lua_pushcfunction(l, api::create_collection as LuaCFunction);
    lua_setfield(l, -2, cstr!("CreateCollection"));
    lua_pushcfunction(l, api::drop_collection as LuaCFunction);
    lua_setfield(l, -2, cstr!("DropCollection"));
    lua_pushcfunction(l, api::collection_stats as LuaCFunction);
    lua_setfield(l, -2, cstr!("Stats"));
    lua_pushcfunction(l, api::drop_database as LuaCFunction);
    lua_setfield(l, -2, cstr!("Drop"));
    lua_pop(l, 1);

    // Register MongoDBCollection metatable
    luaL_newmetatable(l, cstr!("MongoDBCollection"));
    lua_pushvalue(l, -1);
    lua_setfield(l, -2, cstr!("__index"));

    // CRUD operations
    lua_pushcfunction(l, api::insert_one as LuaCFunction);
    lua_setfield(l, -2, cstr!("InsertOne"));
    lua_pushcfunction(l, api::insert_many as LuaCFunction);
    lua_setfield(l, -2, cstr!("InsertMany"));
    lua_pushcfunction(l, api::find as LuaCFunction);
    lua_setfield(l, -2, cstr!("Find"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::find_one) });
    lua_setfield(l, -2, cstr!("FindOne"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::update_one) });
    lua_setfield(l, -2, cstr!("UpdateOne"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::update_many) });
    lua_setfield(l, -2, cstr!("UpdateMany"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::delete_one) });
    lua_setfield(l, -2, cstr!("DeleteOne"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::delete_many) });
    lua_setfield(l, -2, cstr!("DeleteMany"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::count_documents) });
    lua_setfield(l, -2, cstr!("Count"));

    lua_pushcfunction(l, api::insert_one_async as LuaCFunction);
    lua_setfield(l, -2, cstr!("InsertOneAsync"));
    lua_pushcfunction(l, api::insert_many_async as LuaCFunction);
    lua_setfield(l, -2, cstr!("InsertManyAsync"));
    lua_pushcfunction(l, api::find_async as LuaCFunction);
    lua_setfield(l, -2, cstr!("FindAsync"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::find_one_async) });
    lua_setfield(l, -2, cstr!("FindOneAsync"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::update_one_async) });
    lua_setfield(l, -2, cstr!("UpdateOneAsync"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::update_many_async) });
    lua_setfield(l, -2, cstr!("UpdateManyAsync"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::delete_one_async) });
    lua_setfield(l, -2, cstr!("DeleteOneAsync"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::delete_many_async) });
    lua_setfield(l, -2, cstr!("DeleteManyAsync"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::count_documents_async) });
    lua_setfield(l, -2, cstr!("CountAsync"));

    // Aggregation
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::aggregate) });
    lua_setfield(l, -2, cstr!("Aggregate"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::aggregate_async) });
    lua_setfield(l, -2, cstr!("AggregateAsync"));

    // Index management
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::create_index) });
    lua_setfield(l, -2, cstr!("CreateIndex"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::list_indexes) });
    lua_setfield(l, -2, cstr!("ListIndexes"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::drop_index) });
    lua_setfield(l, -2, cstr!("DropIndex"));

    lua_pop(l, 1);

    // Create global MongoDB table
    lua_newtable(l);

    // Client creation
    lua_pushcfunction(l, api::new_client as LuaCFunction);
    lua_setfield(l, -2, cstr!("Client"));
    lua_pushcfunction(l, unsafe { std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(api::new_client_with_options) });
    lua_setfield(l, -2, cstr!("ClientWithOptions"));

    // Utility functions
    lua_pushcfunction(l, suppress_messages);
    lua_setfield(l, -2, cstr!("SuppressMessages"));
    lua_pushcfunction(l, get_version);
    lua_setfield(l, -2, cstr!("Version"));

    lua_setglobal(l, cstr!("MongoDB"));
    
    0
}

extern "C" fn suppress_messages(l: LuaState) -> i32 {
    let suppress = lua_toboolean(l, 1) != 0;
    SUPPRESS_MESSAGES.store(suppress, Ordering::Relaxed);
    info!("Message suppression: {}", if suppress { "enabled" } else { "disabled" });
    0
}

extern "C" fn get_version(l: LuaState) -> i32 {
    unsafe {
        let version = env!("CARGO_PKG_VERSION");
        let cstr = std::ffi::CString::new(version).unwrap();
        lua_pushstring(l, cstr.as_ptr());
    }
    1
}

#[gmod_close]
fn close(_l: LuaState) -> i32 {
    info!("MongoDB module unloading...");
    info!("Goodbye!");
    0
}
