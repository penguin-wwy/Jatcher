use std::cell::RefCell;
use std::collections::HashMap;
use std::ptr;
use std::sync::Mutex;

static mut RTINFO: *const Mutex<*mut RTInfo> = 0 as *const Mutex<*mut RTInfo>;

pub struct _jmethodID {}
pub type jmethodID = *const _jmethodID;

pub struct _jobject {}
pub type jobject = *mut _jobject;
pub type jclass = jobject;
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

// mark runtime info, example method id map
pub struct RTInfo {
    klasses: Klasses,
    methods: Methods,
}

impl RTInfo {
    fn new() -> Self {
        RTInfo {
            klasses: Klasses::new(),
            methods: Methods::new(),
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
}