use std::cell::RefCell;
use std::collections::HashMap;
use std::ptr;
use std::sync::Mutex;
use std::os::raw::{c_void, c_char};

const NULL_STRING: &'static str = "<null>";

static mut RTINFO: *const Mutex<*mut RTInfo> = 0 as *const Mutex<*mut RTInfo>;

#[repr(C)]
pub struct CStruct;
pub type jmethodID = *const CStruct;
pub type jobject = *const CStruct;
pub type jclass = *const CStruct;
pub type JNIEnv = *const CStruct;
pub type jvmtiEnv = *const CStruct;
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
    fn new() -> Self {
        RTInfo {
            klasses: Klasses::new(),
            methods: Methods::new(),
            break_points: HashMap::new()
        }
    }

    pub unsafe fn rt_instance() -> &'static mut Self {
        if RTINFO == ptr::null_mut() {
            let mut r = Box::new(RTInfo::new());
            RTINFO = &Mutex::new(&mut *r as *mut RTInfo) as *const Mutex<*mut RTInfo>;
        }
        &mut (**(*RTINFO).lock().unwrap())
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

#[no_mangle]
pub unsafe extern "C" fn preprocess_method(km: *const KlassMethod) {

}