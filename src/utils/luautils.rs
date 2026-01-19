use std::ptr;
use rglua::lua::LuaState;
use rglua::prelude::*;
use crate::error::{LuaError, LuaResult};

pub unsafe fn write_userdata<T: Clone>(l: LuaState, data: T) {
    let size = std::mem::size_of::<T>();
    let ptr = lua_newuserdata(l, size) as *mut T;
    ptr::write(ptr, data);
}

pub unsafe fn read_userdata<T: Clone>(l: LuaState, index: i32) -> LuaResult<T> {
    let ptr = lua_touserdata(l, index);
    if ptr.is_null() {
        return Err(LuaError::InvalidUserdata(
            "Expected userdata, got null pointer".to_string()
        ));
    }

    let data_ptr = ptr as *const T;
    Ok((*data_ptr).clone())
}

pub unsafe fn is_userdata(l: LuaState, index: i32) -> bool {
    lua_type(l, index) == 7 // LUA_TUSERDATA
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
    c_str.to_str()
        .map(|s| s.to_string())
        .map_err(|e| LuaError::InvalidArgument {
            position: index as usize,
            message: format!("Invalid UTF-8 string: {}", e),
        })
}

pub unsafe fn opt_string(l: LuaState, index: i32) -> LuaResult<Option<String>> {
    if lua_isnoneornil(l, index) {
        return Ok(None);
    }
    check_string(l, index).map(Some)
}

pub unsafe fn check_number(l: LuaState, index: i32) -> LuaResult<f64> {
    if lua_isnumber(l, index) == 0 {
        return Err(LuaError::InvalidArgument {
            position: index as usize,
            message: "Expected number".to_string(),
        });
    }
    Ok(lua_tonumber(l, index))
}

pub unsafe fn check_integer(l: LuaState, index: i32) -> LuaResult<i64> {
    let num = check_number(l, index)?;
    Ok(num as i64)
}

pub unsafe fn check_boolean(l: LuaState, index: i32) -> LuaResult<bool> {
    if lua_isboolean(l, index) == false {
        return Err(LuaError::InvalidArgument {
            position: index as usize,
            message: "Expected boolean".to_string(),
        });
    }
    Ok(lua_toboolean(l, index) != 0)
}

pub unsafe fn opt_boolean(l: LuaState, index: i32, default: bool) -> bool {
    if lua_isnoneornil(l, index) {
        return default;
    }
    lua_toboolean(l, index) != 0
}

pub unsafe fn push_error(l: LuaState, error: impl std::fmt::Display) -> i32 {
    let error_msg = format!("{}", error);
    let c_str = std::ffi::CString::new(error_msg).unwrap_or_else(|_| {
        std::ffi::CString::new("Unknown error").unwrap()
    });
    luaL_error(l, c_str.as_ptr());
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_userdata_size() {
        let size = std::mem::size_of::<String>();
        assert!(size > 0);
    }
}
