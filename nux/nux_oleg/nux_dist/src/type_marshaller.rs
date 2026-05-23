// Type Marshaller - Handles automatic type conversion between Nux and foreign languages
// Provides bidirectional type mapping and memory-safe conversions

use std::collections::HashMap;
use std::fmt;

/// Represents a Nux value that can be marshalled to/from foreign types
#[derive(Debug, Clone)]
pub enum NuxValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<NuxValue>),
    Map(HashMap<String, NuxValue>),
    Function(String), // Function name/reference
    ForeignObject(ForeignObjectRef),
}

/// Reference to a foreign language object
#[derive(Debug, Clone)]
pub struct ForeignObjectRef {
    pub language: String,
    pub object_id: usize,
    pub type_name: String,
}

/// Type marshaller for converting between Nux and foreign types
pub struct TypeMarshaller {
    // Cache for foreign object references
    foreign_objects: HashMap<usize, ForeignObjectRef>,
    next_object_id: usize,
}

impl TypeMarshaller {
    pub fn new() -> Self {
        TypeMarshaller {
            foreign_objects: HashMap::new(),
            next_object_id: 0,
        }
    }

    /// Convert Nux value to Python-compatible representation
    pub fn to_python(&self, value: &NuxValue) -> Result<PythonValue, String> {
        match value {
            NuxValue::Null => Ok(PythonValue::None),
            NuxValue::Bool(b) => Ok(PythonValue::Bool(*b)),
            NuxValue::Int(i) => Ok(PythonValue::Int(*i)),
            NuxValue::Float(f) => Ok(PythonValue::Float(*f)),
            NuxValue::String(s) => Ok(PythonValue::String(s.clone())),
            NuxValue::Array(arr) => {
                let py_arr: Result<Vec<PythonValue>, String> = arr
                    .iter()
                    .map(|v| self.to_python(v))
                    .collect();
                Ok(PythonValue::List(py_arr?))
            }
            NuxValue::Map(map) => {
                let mut py_dict = HashMap::new();
                for (k, v) in map.iter() {
                    py_dict.insert(k.clone(), self.to_python(v)?);
                }
                Ok(PythonValue::Dict(py_dict))
            }
            NuxValue::Function(name) => Ok(PythonValue::Callable(name.clone())),
            NuxValue::ForeignObject(obj_ref) => {
                if obj_ref.language == "python" {
                    Ok(PythonValue::Object(obj_ref.object_id))
                } else {
                    Err(format!("Cannot convert {} object to Python", obj_ref.language))
                }
            }
        }
    }

    /// Convert Python value to Nux representation
    pub fn from_python(&mut self, value: &PythonValue) -> Result<NuxValue, String> {
        match value {
            PythonValue::None => Ok(NuxValue::Null),
            PythonValue::Bool(b) => Ok(NuxValue::Bool(*b)),
            PythonValue::Int(i) => Ok(NuxValue::Int(*i)),
            PythonValue::Float(f) => Ok(NuxValue::Float(*f)),
            PythonValue::String(s) => Ok(NuxValue::String(s.clone())),
            PythonValue::List(list) => {
                let nux_arr: Result<Vec<NuxValue>, String> = list
                    .iter()
                    .map(|v| self.from_python(v))
                    .collect();
                Ok(NuxValue::Array(nux_arr?))
            }
            PythonValue::Dict(dict) => {
                let mut nux_map = HashMap::new();
                for (k, v) in dict.iter() {
                    nux_map.insert(k.clone(), self.from_python(v)?);
                }
                Ok(NuxValue::Map(nux_map))
            }
            PythonValue::Callable(name) => Ok(NuxValue::Function(name.clone())),
            PythonValue::Object(obj_id) => {
                let obj_ref = ForeignObjectRef {
                    language: "python".to_string(),
                    object_id: *obj_id,
                    type_name: "object".to_string(),
                };
                Ok(NuxValue::ForeignObject(obj_ref))
            }
        }
    }

    /// Convert Nux value to JavaScript-compatible representation
    pub fn to_javascript(&self, value: &NuxValue) -> Result<JavaScriptValue, String> {
        match value {
            NuxValue::Null => Ok(JavaScriptValue::Null),
            NuxValue::Bool(b) => Ok(JavaScriptValue::Boolean(*b)),
            NuxValue::Int(i) => Ok(JavaScriptValue::Number(*i as f64)),
            NuxValue::Float(f) => Ok(JavaScriptValue::Number(*f)),
            NuxValue::String(s) => Ok(JavaScriptValue::String(s.clone())),
            NuxValue::Array(arr) => {
                let js_arr: Result<Vec<JavaScriptValue>, String> = arr
                    .iter()
                    .map(|v| self.to_javascript(v))
                    .collect();
                Ok(JavaScriptValue::Array(js_arr?))
            }
            NuxValue::Map(map) => {
                let mut js_obj = HashMap::new();
                for (k, v) in map.iter() {
                    js_obj.insert(k.clone(), self.to_javascript(v)?);
                }
                Ok(JavaScriptValue::Object(js_obj))
            }
            NuxValue::Function(name) => Ok(JavaScriptValue::Function(name.clone())),
            NuxValue::ForeignObject(obj_ref) => {
                if obj_ref.language == "javascript" {
                    Ok(JavaScriptValue::ObjectRef(obj_ref.object_id))
                } else {
                    Err(format!("Cannot convert {} object to JavaScript", obj_ref.language))
                }
            }
        }
    }

    /// Convert JavaScript value to Nux representation
    pub fn from_javascript(&mut self, value: &JavaScriptValue) -> Result<NuxValue, String> {
        match value {
            JavaScriptValue::Null | JavaScriptValue::Undefined => Ok(NuxValue::Null),
            JavaScriptValue::Boolean(b) => Ok(NuxValue::Bool(*b)),
            JavaScriptValue::Number(n) => {
                if n.fract() == 0.0 && *n >= i64::MIN as f64 && *n <= i64::MAX as f64 {
                    Ok(NuxValue::Int(*n as i64))
                } else {
                    Ok(NuxValue::Float(*n))
                }
            }
            JavaScriptValue::String(s) => Ok(NuxValue::String(s.clone())),
            JavaScriptValue::Array(arr) => {
                let nux_arr: Result<Vec<NuxValue>, String> = arr
                    .iter()
                    .map(|v| self.from_javascript(v))
                    .collect();
                Ok(NuxValue::Array(nux_arr?))
            }
            JavaScriptValue::Object(obj) => {
                let mut nux_map = HashMap::new();
                for (k, v) in obj.iter() {
                    nux_map.insert(k.clone(), self.from_javascript(v)?);
                }
                Ok(NuxValue::Map(nux_map))
            }
            JavaScriptValue::Function(name) => Ok(NuxValue::Function(name.clone())),
            JavaScriptValue::ObjectRef(obj_id) => {
                let obj_ref = ForeignObjectRef {
                    language: "javascript".to_string(),
                    object_id: *obj_id,
                    type_name: "object".to_string(),
                };
                Ok(NuxValue::ForeignObject(obj_ref))
            }
        }
    }

    /// Register a foreign object and return its ID
    pub fn register_foreign_object(&mut self, language: &str, type_name: &str) -> usize {
        let obj_id = self.next_object_id;
        self.next_object_id += 1;

        let obj_ref = ForeignObjectRef {
            language: language.to_string(),
            object_id: obj_id,
            type_name: type_name.to_string(),
        };

        self.foreign_objects.insert(obj_id, obj_ref);
        obj_id
    }

    /// Get foreign object reference by ID
    pub fn get_foreign_object(&self, obj_id: usize) -> Option<&ForeignObjectRef> {
        self.foreign_objects.get(&obj_id)
    }

    /// Release a foreign object reference
    pub fn release_foreign_object(&mut self, obj_id: usize) {
        self.foreign_objects.remove(&obj_id);
    }
}

/// Python value representation
#[derive(Debug, Clone)]
pub enum PythonValue {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List(Vec<PythonValue>),
    Dict(HashMap<String, PythonValue>),
    Callable(String),
    Object(usize), // Object ID
}

/// JavaScript value representation
#[derive(Debug, Clone)]
pub enum JavaScriptValue {
    Null,
    Undefined,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JavaScriptValue>),
    Object(HashMap<String, JavaScriptValue>),
    Function(String),
    ObjectRef(usize), // Object ID
}

/// Rust value representation (for FFI)
#[derive(Debug, Clone)]
pub enum RustValue {
    Unit,
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    String(String),
    Vec(Vec<RustValue>),
    Struct(HashMap<String, RustValue>),
    Pointer(usize),
}

/// C value representation (for FFI)
#[derive(Debug, Clone)]
pub enum CValue {
    Void,
    Bool(bool),
    Char(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    UChar(u8),
    UShort(u16),
    UInt(u32),
    ULong(u64),
    Float(f32),
    Double(f64),
    Pointer(usize),
    String(String),
    Array(Vec<CValue>),
}

impl fmt::Display for NuxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NuxValue::Null => write!(f, "null"),
            NuxValue::Bool(b) => write!(f, "{}", b),
            NuxValue::Int(i) => write!(f, "{}", i),
            NuxValue::Float(fl) => write!(f, "{}", fl),
            NuxValue::String(s) => write!(f, "\"{}\"", s),
            NuxValue::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            NuxValue::Map(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", k, v)?;
                }
                write!(f, "}}")
            }
            NuxValue::Function(name) => write!(f, "<function {}>", name),
            NuxValue::ForeignObject(obj_ref) => {
                write!(f, "<{} object #{}>", obj_ref.language, obj_ref.object_id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nux_to_python() {
        let marshaller = TypeMarshaller::new();
        let nux_val = NuxValue::Int(42);
        let py_val = marshaller.to_python(&nux_val).unwrap();
        match py_val {
            PythonValue::Int(i) => assert_eq!(i, 42),
            _ => panic!("Expected PythonValue::Int"),
        }
    }

    #[test]
    fn test_python_to_nux() {
        let mut marshaller = TypeMarshaller::new();
        let py_val = PythonValue::String("hello".to_string());
        let nux_val = marshaller.from_python(&py_val).unwrap();
        match nux_val {
            NuxValue::String(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected NuxValue::String"),
        }
    }

    #[test]
    fn test_nux_array_to_python_list() {
        let marshaller = TypeMarshaller::new();
        let nux_val = NuxValue::Array(vec![
            NuxValue::Int(1),
            NuxValue::Int(2),
            NuxValue::Int(3),
        ]);
        let py_val = marshaller.to_python(&nux_val).unwrap();
        match py_val {
            PythonValue::List(list) => assert_eq!(list.len(), 3),
            _ => panic!("Expected PythonValue::List"),
        }
    }

    #[test]
    fn test_register_foreign_object() {
        let mut marshaller = TypeMarshaller::new();
        let obj_id = marshaller.register_foreign_object("python", "MyClass");
        assert_eq!(obj_id, 0);
        let obj_ref = marshaller.get_foreign_object(obj_id).unwrap();
        assert_eq!(obj_ref.language, "python");
        assert_eq!(obj_ref.type_name, "MyClass");
    }
}
