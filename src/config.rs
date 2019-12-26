use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::os::raw::c_char;
use std::ffi::CStr;
use std::fs::File;
use std::io::Read;
use std::mem::transmute;
use logger::error;
use std::ptr::null_mut;
use std::sync::Once;
use std::cell::Cell;

static EMPTY_BOX: Box<Configuration> = unsafe { Box::from_raw(null_mut()) };

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    pub verbose: bool,
    pub log_file: Option<String>,
    pub output_file: Option<String>,
    pub bytecode_dump: Vec<String>,
    pub heap_print: bool,
    pub class_print: bool,
    pub break_point_json: Option<String>,
    pub watch_var: Option<String>,
}

impl Configuration {
    fn create_from(data: &str) -> Box<serde_json::Result<Self>> {
        Box::new(serde_json::from_str(data))
    }
}

struct ConfigParser {
    init: Once,
    config: Cell<Box<Configuration>>,
    success: Cell<bool>,
}

static CONFIG_BOX: ConfigParser = ConfigParser {
    init: Once::new(),
    config: Cell::new(EMPTY_BOX),
    success: Cell::new(false),
};

#[no_mangle]
pub unsafe extern "C" fn parse_config(path: *const c_char) -> i32 {
    assert!(!path.is_null());
    let mut content = String::new();
    CStr::from_ptr(path).to_str().and_then(|path_str| {
        File::open(path_str).as_mut().and_then(|file| {
            file.read_to_string(&mut content).and_then(|_| {
                CONFIG_BOX.init.call_once(|| {
                    match Configuration::create_from(content.as_str()) {
                        Ok(C) => {
                            CONFIG_BOX.config.set(Box::new(C));
                            CONFIG_BOX.success.set(true);
                        },
                        Err(E) => {
                            error(format!("{:?}", e).as_str());
                            CONFIG_BOX.success.set(false);
                        }
                    };
                });
            });
        });
    });
    return match CONFIG_BOX.success.get() {
        true => 0,
        false => 1
    }
}