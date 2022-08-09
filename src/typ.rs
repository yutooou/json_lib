use std::collections::HashMap;
use jni::errors::Error;
use jni::JNIEnv;
use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jobject, jshort};
use jni::objects::{JObject, JString, JValue};

pub trait Convert {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error>;
    // TODO: 反序列化
    // fn deserialize() -> Object;
}

pub struct ObjectType;
impl ObjectType {
    pub fn from_jvalue(env: JNIEnv, input: JValue) -> Box<dyn Convert> {
        match input {
            JValue::Int(i) => {
                Box::new(Int{value: i})
            }
            JValue::Byte(b) => {
                Box::new(Byte{value: b})
            }
            JValue::Char(c) => {
                Box::new(Char{value: c})
            }
            JValue::Short(s) => {
                Box::new(Short{value: s})
            }
            JValue::Long(l) => {
                Box::new(Long{value: l})
            }
            JValue::Bool(b) => {
                Box::new(Bool{value: b})
            }
            JValue::Float(f) => {
                Box::new(Float{value: f})
            }
            JValue::Double(d) => {
                Box::new(Double{value: d})
            }
            JValue::Void => {
                Box::new(_Null{})
            }
            JValue::Object(input) => {
                let input = input.into_inner();
                if input.is_null() {
                    Box::new(_Null{})
                } else if env.is_instance_of(input, env.find_class("java/lang/Byte").unwrap()).unwrap(){
                    Box::new(_Byte{value: input})
                } else if env.is_instance_of(input, env.find_class("java/lang/Short").unwrap()).unwrap(){
                    Box::new(_Short{value: input})
                } else if env.is_instance_of(input, env.find_class("java/lang/Integer").unwrap()).unwrap(){
                    Box::new(_Integer{value: input})
                } else if env.is_instance_of(input, env.find_class("java/lang/Long").unwrap()).unwrap(){
                    Box::new(_Long{value: input})
                } else if env.is_instance_of(input, env.find_class("java/lang/Float").unwrap()).unwrap(){
                    Box::new(_Float{value: input})
                } else if env.is_instance_of(input, env.find_class("java/lang/Double").unwrap()).unwrap(){
                    Box::new(_Double{value: input})
                } else if env.is_instance_of(input, env.find_class("java/lang/Boolean").unwrap()).unwrap(){
                    Box::new(_Boolean{value: input})
                } else if env.is_instance_of(input, env.find_class("java/lang/Character").unwrap()).unwrap(){
                    Box::new(_Character{value: input})
                } else if env.is_instance_of(input, env.find_class("java/lang/String").unwrap()).unwrap(){
                    Box::new(_String{value: input})
                } else if env.is_instance_of(input, env.find_class("java/util/Map").unwrap()).unwrap() {
                    Box::new(_Map{value: input})
                } else if env.is_instance_of(input, env.find_class("java/util/List").unwrap()).unwrap() {
                    Box::new(_List{value: input})
                } else {
                    Box::new(_Object{value: input})
                }
            }
        }
    }
}
// int
pub struct Int {
    value: jint,
}

impl Convert for Int {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let x: i32 = self.value.into();
        Ok(x.to_string())
    }
}

// byte
pub struct Byte {
    value: jbyte,
}

impl Convert for Byte {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let x: i8 = self.value.into();
        Ok(x.to_string())
    }
}

// char
pub struct Char {
    value: jchar,
}

impl Convert for Char {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let data: u16 = self.value.into();
        let data = String::from_utf16(&vec![data]).unwrap();
        let mut res = String::from("\"");
        res.push_str(&data);
        res.push('"');
        Ok(res)
    }
}

// short
pub struct Short {
    value: jshort,
}

impl Convert for Short {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let x: i16 = self.value.into();
        Ok(x.to_string())
    }
}

// long
pub struct Long {
    value: jlong,
}

impl Convert for Long {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let x: i64 = self.value.into();
        Ok(x.to_string())
    }
}

// float
pub struct Float {
    value: jfloat,
}

impl Convert for Float {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let x: f32 = self.value.into();
        if x == 0.0 {
            Ok("0.0".to_string())
        } else {
            Ok(x.to_string())
        }
    }
}

// boolean
pub struct Bool {
    value: jboolean,
}

impl Convert for Bool {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let x: u8 = self.value.into();
        if x == 0 {
            Ok(String::from("false"))
        } else {
            Ok(String::from("true"))
        }
    }
}

// double
pub struct Double {
    value: jdouble,
}

impl Convert for Double {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let x: f64 = self.value.into();
        if x == 0.0 {
            Ok("0.0".to_string())
        } else {
            Ok(x.to_string())
        }

    }
}


// 注意这里_Null存在的意义十分重要
// 因为在实际使用过程中，对象很可能是null（包括包装类型）
// 在env.is_instance_of的过程中 当然null就是任何类的实例
// 故会匹配到第一个类型，那么他的默认值是错误的
pub struct _Null{
}

impl Convert for _Null {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        Ok(String::from("null"))
    }
}

// 包装类型Byte
pub struct _Byte {
    value: jobject,
}

impl Convert for _Byte {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let result = env.call_method(self.value, "byteValue", "()B", &vec![])?.b()?;
        Ok(result.to_string())
    }
}

// 包装类型Short
pub struct _Short {
    value: jobject,
}

impl Convert for _Short {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let result = env.call_method(self.value, "shortValue", "()S", &vec![])?.s()?;
        Ok(result.to_string())
    }
}

// 包装类型Integer
pub struct _Integer {
    value: jobject,
}

impl Convert for _Integer {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let result = env.call_method(self.value, "intValue", "()I", &vec![])?.i()?;
        Ok(result.to_string())
    }
}

// 包装类型Long
pub struct _Long {
    value: jobject,
}

impl Convert for _Long {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let result = env.call_method(self.value, "longValue", "()J", &vec![])?.j()?;
        Ok(result.to_string())
    }
}

// 包装类型Float
pub struct _Float {
    value: jobject,
}

impl Convert for _Float {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let result = env.call_method(self.value, "floatValue", "()F", &vec![])?;
        let x: f32 = result.f()?.into();
        if x == 0.0 {
            Ok("0.0".to_string())
        } else {
            Ok(x.to_string())
        }
    }
}

// 包装类型Double
pub struct _Double {
    value: jobject,
}

impl Convert for _Double {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let result = env.call_method(self.value, "doubleValue", "()D", &vec![])?;
        let x: f64 = result.d()?.into();
        if x == 0.0 {
            Ok("0.0".to_string())
        } else {
            Ok(x.to_string())
        }
    }
}

// 包装类型Boolean
pub struct _Boolean {
    value: jobject,
}

impl Convert for _Boolean {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let result = env.call_method(self.value, "booleanValue", "()Z", &vec![])?.z()?;
        Ok(result.to_string())
    }
}

// 包装类型Character
pub struct _Character {
    value: jobject,
}

impl Convert for _Character {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let result = env.call_method(self.value, "charValue", "()C", &vec![])?.c()?;
        let data: u16 = result.into();
        let data = String::from_utf16(&vec![data]).unwrap();
        let mut res = String::from("\"");
        res.push_str(&data);
        res.push('"');
        Ok(res)
    }
}

// 包装类型String
pub struct _String {
    value: jobject,
}

impl Convert for _String {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let data: JString = self.value.into();
        let data = env.get_string(data)?;
        let data = data.to_str().unwrap();
        let mut res = String::from("\"");
        res.push_str(data);
        res.push('"');
        Ok(res)
    }
}

// java/util/Map
pub struct _Map {
    value: jobject,
}

impl Convert for _Map {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let mut res: String  = String::from("{");
        let end = "}";
        let map = env.get_map(self.value.into())?;
        let iter = map.iter()?;
        let mut flag = false;
        for (kobj, vobj) in iter {
            flag = true;
            let value = JValue::Object(kobj);
            let obj_type = ObjectType::from_jvalue(env, value);
            let k = obj_type.serialize(env)?;

            let value = JValue::Object(vobj);
            let obj_type = ObjectType::from_jvalue(env, value);
            let v = obj_type.serialize(env)?;
            res.push_str(&k);
            res.push_str(": ");
            res.push_str(&v);
            res.push_str(", ");
        }
        if flag {
            res.remove(res.len() - 1);
            res.remove(res.len() - 1);
        }
        Ok(res + end)
    }
}

// java/util/List
pub struct _List {
    value: jobject,
}

impl Convert for _List {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let mut res: String  = String::from("[");
        let end = "]";
        let list = env.get_list(self.value.into())?;
        let size = list.size()?;
        for i in 0..size {

            let item = match list.get(i)? {
                Some(l) => {
                    l
                }
                None => {
                    JObject::null()
                }
            };

            // JObject
            let value = JValue::Object(item);
            let obj_type = ObjectType::from_jvalue(env, value);
            let sub = obj_type.serialize(env)?;
            res.push_str(&sub);
            if i != size - 1 {
                res.push_str(", ")
            }
        }
        Ok(res + end)
    }
}

// 带有public HashMap fieldTag()的其他对象
pub struct _Object {
    value: jobject,
}

impl Convert for _Object {
    fn serialize(&self, env: JNIEnv) -> Result<String, Error> {
        let mut res = String::from("{");
        let tag_map = self.load_tag(env)?;
        let size = tag_map.len();
        let mut idx = 0;
        for (name, tag) in tag_map.iter() {
            idx += 1;
            let value = env.get_field(self.value, name, tag)?;
            let obj_type = ObjectType::from_jvalue(env, value);
            let sub = obj_type.serialize(env)?;
            if idx != size {
                // 先不考虑性能
                res.push('"');
                res.push_str(name);
                res.push_str("\": ");
                res.push_str(&sub);
                res.push_str(", ");
            } else {
                res.push('"');
                res.push_str(name);
                res.push_str("\": ");
                res.push_str(&sub);
            }
        }
        let end = "}";
        Ok(res + end)
    }
}

impl _Object {
    // 读取fieldTag()方法，获取对象的字段信息包括（k：字段名 v：字段类型）
    fn load_tag(&self, env: JNIEnv) -> Result<HashMap<String, String>, Error> {
        let field_tag = env.call_method(self.value, "fieldTag", "()Ljava/util/HashMap;", &vec![])?.l()?;
        let field_tag = env.get_map(field_tag)?;
        let iter = field_tag.iter()?;
        let mut map: HashMap<String, String> = HashMap::new();
        for (kobj, vobj) in iter {
            let k: String = env.get_string(JString::from(kobj))?.into();
            let v: String = env.get_string(JString::from(vobj))?.into();
            let mut tag = String::from("");
            if v == "int" {
                tag = "I".to_string()
            } else if v == "byte" {
                tag = "B".to_string()
            } else if v == "char" {
                tag = "C".to_string()
            } else if v == "short" {
                tag = "S".to_string()
            } else if v == "long" {
                tag = "J".to_string()
            } else if v == "boolean" {
                tag = "Z".to_string()
            } else if v == "float" {
                tag = "F".to_string()
            } else if v == "double" {
                tag = "D".to_string()
            } else {
                tag.push_str("L");
                tag.push_str(&v.replace(".", "/"));
                tag.push_str(";");
            }
            map.insert(k, tag);
        }
        Ok(map)
    }
}
