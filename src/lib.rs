use jni::errors::Error;
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue};
use jni::sys::jstring;
use crate::typ::ObjectType;

mod typ;

// public class JSON {
//
//     public static native String toJson(Object input);
//
//     static {
//         System.loadLibrary("json_lib");
//     }
// }

#[no_mangle]
pub extern "system" fn Java_JSON_toJson(env: JNIEnv, class: JClass, input: JObject) -> jstring {
    if input.is_null() {
        return env.new_string("").unwrap().into_inner()
    }
    let value = JValue::Object(input);
    let obj_type = ObjectType::from_jvalue(env, value);
    let x = obj_type.serialize(env);

    match x {
        Ok(x) => {
            return env.new_string(&x).unwrap().into_inner()
        }
        Err(_) => {
            env.new_string("").unwrap().into_inner()
        }
    }
}
