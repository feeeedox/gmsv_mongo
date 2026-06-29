use crate::error::{LuaError, LuaResult};
use rglua::lua::LuaState;
use rglua::prelude::*;
use std::ptr;

pub unsafe fn write_userdata<T: Clone>(l: LuaState, data: T) {
    let size = std::mem::size_of::<T>();
    let ptr = lua_newuserdata(l, size) as *mut T;
    ptr::write(ptr, data);
}

pub unsafe fn read_userdata<T: Clone>(l: LuaState, index: i32) -> LuaResult<T> {
    let ptr = lua_touserdata(l, index);
    if ptr.is_null() {
        return Err(LuaError::InvalidUserdata(
            "Expected userdata, got null pointer".to_string(),
        ));
    }

    let data_ptr = ptr as *const T;
    Ok((*data_ptr).clone())
}
pub unsafe fn check_string(l: LuaState, index: i32) -> LuaResult<String> {
    let ptr = luaL_checkstring(l, index);
    if ptr.is_null() {
        return Err(LuaError::InvalidArgument {
            position: index as usize,
            message: "Expected string, got nil".to_string(),
        });
    }

    let c_str = std::ffi::CStr::from_ptr(ptr);
    c_str
        .to_str()
        .map(|s| s.to_string())
        .map_err(|e| LuaError::InvalidArgument {
            position: index as usize,
            message: format!("Invalid UTF-8 string: {}", e),
        })
}

pub unsafe fn opt_boolean(l: LuaState, index: i32, default: bool) -> bool {
    if lua_isnoneornil(l, index) {
        return default;
    }
    lua_toboolean(l, index) != 0
}

pub unsafe fn push_error(l: LuaState, error: impl std::fmt::Display) -> i32 {
    let error_msg = format!("{}", error);
    let c_str = std::ffi::CString::new(error_msg)
        .unwrap_or_else(|_| std::ffi::CString::new("Unknown error").unwrap());
    luaL_error(l, c_str.as_ptr());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_userdata_size() {
        let size = std::mem::size_of::<String>();
        assert!(size > 0);
    }
}
