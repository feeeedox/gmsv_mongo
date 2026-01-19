use crate::error::{LuaError, LuaResult};
use mongodb::bson::{oid::ObjectId, Bson, DateTime, Document};
use rglua::lua::LuaState;
use rglua::prelude::*;
use std::ffi::{CStr, CString};


// Lua type constants
const LUA_TNIL: i32 = 0;
const LUA_TBOOLEAN: i32 = 1;
const LUA_TLIGHTUSERDATA: i32 = 2;
const LUA_TNUMBER: i32 = 3;
const LUA_TSTRING: i32 = 4;
const LUA_TTABLE: i32 = 5;
const LUA_TFUNCTION: i32 = 6;
const LUA_TUSERDATA: i32 = 7;
const LUA_TTHREAD: i32 = 8;

pub fn lua_table_to_bson(l: LuaState, index: i32) -> LuaResult<Document> {
    unsafe {
        if lua_type(l, index) != LUA_TTABLE {
            return Err(LuaError::TypeConversion {
                expected: "table".to_string(),
                actual: lua_typename(l, lua_type(l, index))
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
            });
        }

        let mut doc = Document::new();

        let abs_index = if index < 0 {
            lua_gettop(l) + index + 1
        } else {
            index
        };

        lua_pushnil(l);
        while lua_next(l, abs_index) != 0 {
            let key = get_table_key(l, -2)?;

            let value = lua_value_to_bson(l, -1)?;

            doc.insert(key, value);

            lua_pop(l, 1);
        }

        Ok(doc)
    }
}

unsafe fn lua_value_to_bson(l: LuaState, index: i32) -> LuaResult<Bson> {
    let value_type = lua_type(l, index);

    match value_type {
        LUA_TNIL => Ok(Bson::Null),

        LUA_TBOOLEAN => {
            let value = lua_toboolean(l, index) != 0;
            Ok(Bson::Boolean(value))
        }

        LUA_TNUMBER => {
            let value = lua_tonumber(l, index);
            if value.fract() == 0.0 && value.abs() <= (i64::MAX as f64) {
                Ok(Bson::Int64(value as i64))
            } else {
                Ok(Bson::Double(value))
            }
        }

        LUA_TSTRING => {
            let ptr = lua_tostring(l, index);
            if ptr.is_null() {
                return Err(LuaError::TypeConversion {
                    expected: "valid string".to_string(),
                    actual: "null pointer".to_string(),
                });
            }
            let value = CStr::from_ptr(ptr)
                .to_str()
                .map_err(|e| LuaError::TypeConversion {
                    expected: "UTF-8 string".to_string(),
                    actual: format!("invalid UTF-8: {}", e),
                })?
                .to_string();

            if value.starts_with("ObjectId(") && value.ends_with(")") {
                let oid_str = &value[9..value.len()-1];
                if let Ok(oid) = ObjectId::parse_str(oid_str) {
                    return Ok(Bson::ObjectId(oid));
                }
            }

            Ok(Bson::String(value))
        }

        LUA_TTABLE => {
            lua_pushstring(l, cstr!("__bson_type"));
            lua_gettable(l, index);

            if lua_type(l, -1) == LUA_TSTRING {
                let type_ptr = lua_tostring(l, -1);
                if !type_ptr.is_null() {
                    let bson_type = CStr::from_ptr(type_ptr).to_str().ok();
                    lua_pop(l, 1);

                    if bson_type == Some("date") {
                        return parse_date_table(l, index);
                    }
                }
            }
            lua_pop(l, 1);

            let abs_index = if index < 0 {
                lua_gettop(l) + index + 1
            } else {
                index
            };

            lua_table_to_bson_value(l, index)
        }

        _ => Err(LuaError::TypeConversion {
            expected: "nil, boolean, number, string, or table".to_string(),
            actual: lua_typename(l, value_type)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
        }),
    }
}

unsafe fn lua_table_to_bson_value(l: LuaState, index: i32) -> LuaResult<Bson> {
    let abs = if index < 0 {
        lua_gettop(l) + index + 1
    } else {
        index
    };

    if lua_table_is_array(l, abs) {
        let len = lua_objlen(l, abs);
        let mut arr = Vec::with_capacity(len as usize);

        for i in 1..=len {
            lua_rawgeti(l, abs, i as i32);
            let val = lua_value_to_bson(l, -1)?;
            arr.push(val);
            lua_pop(l, 1);
        }

        Ok(Bson::Array(arr))
    } else {
        let doc = lua_table_to_bson(l, abs)?;
        Ok(Bson::Document(doc))
    }
}

unsafe fn lua_table_is_array(l: LuaState, index: i32) -> bool {
    let abs = if index < 0 {
        lua_gettop(l) + index + 1
    } else {
        index
    };

    let len = lua_objlen(l, abs);
    if len == 0 {
        return false;
    }

    lua_pushnil(l);
    while lua_next(l, abs) != 0 {
        let key_type = lua_type(l, -2);
        if key_type != LUA_TNUMBER {
            lua_pop(l, 2);
            return false;
        }

        let key = lua_tonumber(l, -2);
        if key.fract() != 0.0 || key < 1.0 || key > len as f64 {
            lua_pop(l, 2);
            return false;
        }

        lua_pop(l, 1);
    }

    true
}

unsafe fn get_table_key(l: LuaState, index: i32) -> LuaResult<String> {
    let key_type = lua_type(l, index);

    match key_type {
        LUA_TSTRING => {
            let ptr = lua_tostring(l, index);
            if ptr.is_null() {
                return Err(LuaError::InvalidArgument {
                    position: index as usize,
                    message: "Null string pointer for table key".to_string(),
                });
            }
            CStr::from_ptr(ptr)
                .to_str()
                .map(|s| s.to_string())
                .map_err(|e| LuaError::TableConversion(format!("Invalid UTF-8 in key: {}", e)))
        }

        LUA_TNUMBER => {
            let key = lua_tonumber(l, index);
            Ok(key.to_string())
        }

        _ => Err(LuaError::TableConversion(
            "Table keys must be strings or numbers".to_string()
        )),
    }
}

unsafe fn parse_date_table(l: LuaState, index: i32) -> LuaResult<Bson> {
    lua_pushstring(l, cstr!("timestamp"));
    lua_gettable(l, index);

    if lua_type(l, -1) == LUA_TNUMBER {
        let timestamp = lua_tonumber(l, -1) as i64;
        lua_pop(l, 1);

        let dt = DateTime::from_millis(timestamp);
        return Ok(Bson::DateTime(dt));
    }

    lua_pop(l, 1);
    Err(LuaError::TableConversion("Invalid date table format".to_string()))
}

pub unsafe fn bson_to_lua_table(l: LuaState, doc: &Document) {
    lua_newtable(l);

    for (key, value) in doc.iter() {
        let key_cstr = CString::new(key.as_str()).unwrap();
        lua_pushstring(l, key_cstr.as_ptr());

        bson_value_to_lua(l, value);

        lua_settable(l, -3);
    }
}

/// Convert a BSON value to a Lua value
unsafe fn bson_value_to_lua(l: LuaState, value: &Bson) {
    match value {
        Bson::Null | Bson::Undefined => lua_pushnil(l),

        Bson::Boolean(b) => lua_pushboolean(l, *b as i32),

        Bson::Int32(i) => lua_pushnumber(l, *i as f64),
        Bson::Int64(i) => lua_pushnumber(l, *i as f64),
        Bson::Double(d) => lua_pushnumber(l, *d),

        Bson::String(s) => {
            let cstr = CString::new(s.as_str()).unwrap_or_else(|_| CString::new("").unwrap());
            lua_pushstring(l, cstr.as_ptr());
        }

        Bson::Document(doc) => {
            bson_to_lua_table(l, doc);
        }

        Bson::Array(arr) => {
            lua_newtable(l);
            for (i, item) in arr.iter().enumerate() {
                bson_value_to_lua(l, item);
                lua_rawseti(l, -2, (i + 1) as i32);
            }
        }

        Bson::ObjectId(oid) => {
            let oid_str = format!("ObjectId({})", oid.to_hex());
            let cstr = CString::new(oid_str).unwrap();
            lua_pushstring(l, cstr.as_ptr());
        }

        Bson::DateTime(dt) => {
            lua_newtable(l);
            
            lua_pushstring(l, cstr!("__bson_type"));
            lua_pushstring(l, cstr!("date"));
            lua_settable(l, -3);
            
            lua_pushstring(l, cstr!("timestamp"));
            lua_pushnumber(l, dt.timestamp_millis() as f64);
            lua_settable(l, -3);
        }

        Bson::Binary(bin) => {
            let hex = hex::encode(&bin.bytes);
            let cstr = CString::new(hex).unwrap_or_else(|_| CString::new("").unwrap());
            lua_pushstring(l, cstr.as_ptr());
        }

        _ => {
            lua_pushnil(l);
        }
    }
}

unsafe fn lua_typename(_l: LuaState, tp: i32) -> Option<&'static str> {
    match tp {
        LUA_TNIL => Some("nil"),
        LUA_TBOOLEAN => Some("boolean"),
        LUA_TNUMBER => Some("number"),
        LUA_TSTRING => Some("string"),
        LUA_TTABLE => Some("table"),
        LUA_TFUNCTION => Some("function"),
        LUA_TUSERDATA => Some("userdata"),
        LUA_TTHREAD => Some("thread"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bson_to_document() {
        let mut doc = Document::new();
        doc.insert("string", "test");
        doc.insert("number", 42);
        doc.insert("bool", true);

        assert_eq!(doc.len(), 3);
    }
}
