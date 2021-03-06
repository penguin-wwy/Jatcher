use std::cell::RefCell;
use std::collections::HashMap;
use std::{ptr, mem};
use std::sync::Mutex;
use std::os::raw::{c_void, c_char, c_int};
use std::ffi::CString;
use std::str::Utf8Error;
use logger::{error_log, error};
use core::borrow::Borrow;

const NULL_STRING: &'static str = "<null>";

static mut RTINFO: *mut RTInfo = 0 as *mut RTInfo;

#[repr(C)]
pub struct CStruct {}
pub type jmethodID = *mut CStruct;
pub type jobject = *mut CStruct;
pub type jclass = *mut CStruct;
pub type JNIEnv = *mut CStruct;
pub type jvmtiEnv = *mut CStruct;
//pub type jthrowable = jobject;
//pub type jstring = jobject;
//pub type jarray = jobject;
//pub type jbooleanArray = jarray;
//pub type jbyteArray = jarray;
//pub type jcharArray = jarray;
//pub type jshortArray = jarray;
//pub type jintArray = jarray;
//pub type jlongArray = jarray;
//pub type jfloatArray = jarray;
//pub type jdoubleArray = jarray;
//pub type jobjectArray = jarray;
//pub type jweak = jobject;

struct Klasses {
    id_map: RefCell<HashMap<String, jclass>>,
    name_map: RefCell<HashMap<jclass, String>>,
}

impl Klasses {
    fn new() -> Self {
        Klasses {
            id_map: RefCell::new(HashMap::new()),
            name_map: RefCell::new(HashMap::new()),
        }
    }

    fn get_class_id(&self, name: &String) -> Option<jclass> {
        match self.id_map.borrow().get(name) {
            Some(c) => Some(*c),
            None => None,
        }
    }

    fn get_class_name(&self, id: &jclass) -> Option<String> {
        match self.name_map.borrow().get(id) {
            Some(s) => Some(String::from(s.as_str())),
            None => None
        }
    }

    fn insert_class_id(&mut self, id: jclass, name: &str) {
        self.id_map.get_mut().insert(String::from(name), id);
        self.name_map.get_mut().insert(id, String::from(name));
    }
}

struct Methods {
    id_map: RefCell<HashMap<String, jmethodID>>,
    name_map: RefCell<HashMap<jmethodID, String>>,
}

impl Methods {
    fn new() -> Self {
        Methods {
            id_map: RefCell::new(HashMap::new()),
            name_map: RefCell::new(HashMap::new()),
        }
    }

    fn get_method_id(&self, name: &String) -> Option<jmethodID> {
        match self.id_map.borrow().get(name) {
            Some(i) => Some(*i),
            None => None,
        }
    }

    fn get_method_name(&self, id: &jmethodID) -> Option<String> {
        match self.name_map.borrow().get(id) {
            Some(s) => Some(String::from(s.as_str())),
            None => None
        }
    }

    fn insert_method_id(&mut self, id: jmethodID, name: &str) {
        self.id_map.get_mut().insert(String::from(name), id);
        self.name_map.get_mut().insert(id, String::from(name));
    }
}

#[derive(Deserialize, Serialize)]
pub struct BreakPoint {
    class_name: String,
    method_name: String,
    method_signature: String,
    line: u32,
    var: String,
}

impl BreakPoint {
    pub fn vec_from_str(data: &str) -> serde_json::Result<Vec<Self>> {
        serde_json::from_str(data)
    }

    pub fn get_class_name(&self) -> &String {
        &self.class_name
    }

    pub fn get_method_name(&self) -> &String {
        &self.method_name
    }

    pub fn get_method_signature(&self) -> &String {
        &self.method_signature
    }

    pub fn get_method_full_name(&self) -> String {
        format!("{}{}", self.method_name, self.method_signature)
    }

    pub fn get_line_number(&self) -> u32 {
        self.line.clone()
    }

    pub fn get_variable(&self) -> &String {
        &self.var
    }
}

// mark runtime info, example method id map
pub struct RTInfo {
    klasses: Klasses,
    methods: Methods,
    break_points: HashMap<String, Vec<BreakPoint>>,
}

impl RTInfo {

    pub unsafe fn rt_instance() -> &'static mut Self {
        if RTINFO == ptr::null_mut() {
            RTINFO = &mut *Box::new(RTInfo {
                klasses: Klasses::new(),
                methods: Methods::new(),
                break_points: HashMap::new()
            }) as *mut RTInfo;
        }
        &mut *RTINFO
    }

    pub fn get_class_id(&self, name: &String) -> Option<jclass> {
        self.klasses.get_class_id(name)
    }

    pub fn get_class_name(&self, id: &jclass) -> Option<String> {
        self.klasses.get_class_name(id)
    }

    pub fn get_method_id(&self, name: &String) -> Option<jmethodID> {
        self.methods.get_method_id(name)
    }

    pub fn get_method_name(&self, id: &jmethodID) -> Option<String> {
        self.methods.get_method_name(id)
    }

    pub fn insert_class_id(&mut self, id: jclass, name: &str) {
        self.klasses.insert_class_id(id, name);
    }

    pub fn insert_method_id(&mut self, id: jmethodID, name: &str) {
        self.methods.insert_method_id(id, name);
    }

    pub fn insert_bk(&mut self, class_name: String, bk_point: BreakPoint) {
        if self.break_points.get(&class_name).is_some() {
            self.break_points.get_mut(&class_name).unwrap().push(bk_point);
        } else {
            self.break_points.insert(class_name, vec![bk_point]);
        }
    }

    pub fn get_bk_vec(&self, class_name: &String) -> Option<&Vec<BreakPoint>> {
        self.break_points.get(class_name)
    }
}

#[repr(C)]
pub struct KlassMethod {
    jvmti: jvmtiEnv,
    jni: JNIEnv,
    klass: jclass,
    class_name: *const u8,
    method_id: jmethodID,
    method_name: *const u8,
    method_signature: *const u8,
}

extern {
    fn set_break_point(km: *const KlassMethod, lines: *const u32, len: usize);
}

#[no_mangle]
pub unsafe extern "C" fn preprocess_method(km: *const KlassMethod) {
    let class_name_cstr = CString::from_raw((*km).class_name as *mut c_char);
    let class_name = class_name_cstr.to_str();
    match class_name {
        Ok(s) => {
            RTInfo::rt_instance().insert_class_id((*km).klass, s)
        },
        Err(e) => {
            error(format!("{:?}", e).as_str());
            return;
        }
    }
    let name_cstr = CString::from_raw((*km).method_name as *mut c_char);
    let name = name_cstr.to_str();
    let signature_cstr = CString::from_raw((*km).method_signature as *mut c_char);
    let signature = signature_cstr.to_str();
    if name.is_err() || signature.is_err() {
        return;
    }
    let fun_sig = format!("{}{}:{}", class_name.unwrap(), name.unwrap(), signature.unwrap());
    RTInfo::rt_instance().insert_method_id((*km).method_id, fun_sig.as_str());
    let break_point_vec = RTInfo::rt_instance().get_bk_vec(String::from(class_name.unwrap()).borrow());
    match break_point_vec {
        Some(v) => {
            let mut line_vec: Vec<u32> = vec![];
            for bk in v {
                if name.unwrap() == bk.method_name.as_str()
                    && signature.unwrap() == bk.method_signature.as_str() {
                    line_vec.push(bk.line);
                }
            }
            set_break_point(km, line_vec.as_ptr(), line_vec.len());
        }
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_var_name(method: jmethodID, line: u32, len: *mut usize) -> *const c_void {
    match RTInfo::rt_instance().get_method_name(&method) {
        Some(s) => {
            let mut var_list = Vec::<*const u8>::new();
            let class_name = &s[..s.find(";").unwrap_or(0)];
            match RTInfo::rt_instance().get_bk_vec(&class_name.to_string()) {
                Some(v) => {
                    for bk in v {
                        if bk.line == line {
                            var_list.push(bk.var.as_ptr());
                        }
                    }
                    *len = var_list.len();
                    let ptr = var_list.as_ptr();
                    mem::forget(var_list);
                    ptr as *const c_void
                },
                None => 0 as *const c_void,
            }
        },
        None => 0 as *const c_void
    }
}

#[no_mangle]
pub unsafe extern "C" fn deallocate_str_vec_buffer(ptr: *mut *const u8, len: usize) {
    drop(Vec::from_raw_parts(ptr, len, len));
}