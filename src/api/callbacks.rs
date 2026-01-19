use crate::core::worker::{decrease_callbacks_pending, get_callbacks_pending, mark_hook_unregistered, JobResult, CALLBACK_QUEUE, LUA_REGISTRYINDEX};
use crate::types::bson_to_lua_table;
use log::{error, info};
use rglua::lua::LuaState;
use rglua::prelude::*;

pub unsafe extern "C" fn poll_callbacks(l: LuaState) -> i32 {
    let guard = match CALLBACK_QUEUE.lock() {
        Ok(g) => g,
        Err(_) => {
            error!("Failed to lock callback queue");
            return 0;
        }
    };

    let mut processed = 0;

    loop {
        match guard.1.try_recv() {
            Ok(job) => {
                processed += 1;

                if let Some(callback_ref) = job.callback {
                    // Get the callback function from registry
                    lua_rawgeti(l, LUA_REGISTRYINDEX, callback_ref);

                    // Check if it's a function
                    if !lua_isfunction(l, -1) {
                        lua_pop(l, 1);
                        luaL_unref(l, LUA_REGISTRYINDEX, callback_ref);
                        continue;
                    }

                    // Push callback arguments based on result type
                    if let Some(result) = job.result {
                        push_job_result(l, result);
                    } else {
                        lua_pushnil(l); // error
                        lua_pushstring(l, cstr!("No result"));
                    }

                    // Call the callback
                    if lua_pcall(l, 2, 0, 0) != 0 {
                        error!("Error calling callback: {}",
                            std::ffi::CStr::from_ptr(lua_tostring(l, -1))
                                .to_string_lossy());
                        lua_pop(l, 1);
                    }

                    // Unreference the callback
                    luaL_unref(l, LUA_REGISTRYINDEX, callback_ref);
                }
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => break,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                error!("Callback queue disconnected");
                deafen(l);
                return 0;
            }
        }
    }

    if processed > 0 {
        decrease_callbacks_pending(processed);

        if get_callbacks_pending() == 0 {
            deafen(l);
        }
    }

    0
}

unsafe fn push_job_result(l: LuaState, result: JobResult) {
    match result {
        JobResult::InsertOne(res) => {
            match res {
                Ok(id) => {
                    lua_pushnil(l); // no error
                    lua_pushstring(l, std::ffi::CString::new(id).unwrap().as_ptr());
                }
                Err(e) => {
                    lua_pushstring(l, std::ffi::CString::new(e).unwrap().as_ptr());
                    lua_pushnil(l);
                }
            }
        }
        JobResult::InsertMany(res) => {
            match res {
                Ok(ids) => {
                    lua_pushnil(l);
                    lua_newtable(l);
                    for (i, id) in ids.iter().enumerate() {
                        lua_pushstring(l, std::ffi::CString::new(id.as_str()).unwrap().as_ptr());
                        lua_rawseti(l, -2, (i + 1) as i32);
                    }
                }
                Err(e) => {
                    lua_pushstring(l, std::ffi::CString::new(e).unwrap().as_ptr());
                    lua_pushnil(l);
                }
            }
        }
        JobResult::Find(res) => {
            match res {
                Ok(documents) => {
                    lua_pushnil(l);
                    lua_newtable(l);
                    for (i, doc) in documents.iter().enumerate() {
                        bson_to_lua_table(l, doc);
                        lua_rawseti(l, -2, (i + 1) as i32);
                    }
                }
                Err(e) => {
                    lua_pushstring(l, std::ffi::CString::new(e).unwrap().as_ptr());
                    lua_pushnil(l);
                }
            }
        }
        JobResult::FindOne(res) => {
            match res {
                Ok(Some(doc)) => {
                    lua_pushnil(l);
                    bson_to_lua_table(l, &doc);
                }
                Ok(None) => {
                    lua_pushnil(l);
                    lua_pushnil(l);
                }
                Err(e) => {
                    lua_pushstring(l, std::ffi::CString::new(e).unwrap().as_ptr());
                    lua_pushnil(l);
                }
            }
        }
        JobResult::UpdateOne(res) | JobResult::UpdateMany(res) |
        JobResult::DeleteOne(res) | JobResult::DeleteMany(res) |
        JobResult::CountDocuments(res) => {
            match res {
                Ok(count) => {
                    lua_pushnil(l);
                    lua_pushnumber(l, count as f64);
                }
                Err(e) => {
                    let cstr = std::ffi::CString::new(e).unwrap();
                    lua_pushstring(l, cstr.as_ptr());
                    lua_pushnil(l);
                }
            }
        }
        JobResult::Aggregate(res) => {
            match res {
                Ok(documents) => {
                    lua_pushnil(l);
                    lua_newtable(l);
                    for (i, doc) in documents.iter().enumerate() {
                        bson_to_lua_table(l, doc);
                        lua_rawseti(l, -2, (i + 1) as i32);
                    }
                }
                Err(e) => {
                    let cstr = std::ffi::CString::new(e).unwrap();
                    lua_pushstring(l, cstr.as_ptr());
                    lua_pushnil(l);
                }
            }
        }
    }
}

pub unsafe fn listen(l: LuaState) {
    lua_getglobal(l, cstr!("hook"));
    lua_getfield(l, -1, cstr!("Add"));
    lua_pushstring(l, cstr!("Think"));
    lua_pushstring(l, cstr!("gmsv_mongo_async"));
    lua_pushcfunction(l, std::mem::transmute::<unsafe extern "C" fn(LuaState) -> i32, LuaCFunction>(poll_callbacks));
    lua_call(l, 3, 0);
    lua_pop(l, 1);
}

fn deafen(l: LuaState) {
    unsafe {
        lua_getglobal(l, cstr!("hook"));
        lua_getfield(l, -1, cstr!("Remove"));
        lua_pushstring(l, cstr!("Think"));
        lua_pushstring(l, cstr!("gmsv_mongo_async"));
        lua_call(l, 2, 0);
        lua_pop(l, 1);
        mark_hook_unregistered();
    }
}


