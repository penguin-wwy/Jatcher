use serde::{Deserialize, Serialize};
use std::os::raw::c_char;
use std::ffi::CStr;
use std::fs::File;
use std::io::Read;
use std::mem::transmute;
use logger::error;
use std::sync::Once;
use simple_logging::log_to_file;
use std::path::Path;
use log::LevelFilter;

//static EMPTY_BOX: Box<Configuration> = unsafe { Box::from_raw(null_mut()) };

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    pub verbose: bool,
    pub log_file: Option<String>,
    pub output_file: Option<String>,
    pub heap_print: bool,
    pub class_print: bool,
    pub break_point_json: Option<String>,
    pub watch_var: Option<String>,
}

impl Configuration {
//    const fn create_empty() -> Configuration {
//        return Configuration {
//            verbose: false,
//            log_file: None,
//            output_file: None,
//            heap_print: false,
//            class_print: false,
//            break_point_json: None,
//            watch_var: None
//        };
//    }

    fn create_from(data: &str) -> serde_json::Result<Self> {
        serde_json::from_str(data)
    }
}

struct ConfigParser {
    init: Once,
    config: Option<Box<Configuration>>,
    path: Option<String>
}

static mut CONFIG_BOX: ConfigParser = ConfigParser {
    init: Once::new(),
    config: None,
    path: None
};

#[no_mangle]
#[allow(unused_must_use)]
pub unsafe extern "C" fn parse_config(path: *const c_char) -> i32 {
    assert!(!path.is_null());
    let mut content = String::new();
    CStr::from_ptr(path).to_str().map(|path_str| {
        File::open(path_str).as_mut().map(|file| {
            file.read_to_string(&mut content).map(|_| {
                CONFIG_BOX.init.call_once(|| {
                    match Configuration::create_from(content.as_str()) {
                        Ok(c) => {
                            CONFIG_BOX.config = Some(Box::new(c));
                            CONFIG_BOX.path = Some(String::from(path_str))
                        }
                        Err(e) => {
                            error(format!("{:?}", e).as_str());
                            CONFIG_BOX.config = None;
                        }
                    };
                });
            });
        });
    });
    return match CONFIG_BOX.config.is_some() {
        true => 0,
        false => 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn config_init() {
    if CONFIG_BOX.config.is_none() {
        return;
    }
    let config = CONFIG_BOX.config.as_ref().unwrap().as_ref();
    let mut log_level = LevelFilter::Info;
    if config.verbose {
        log_level = LevelFilter::Debug;
    }
    if let Some(ref log_path) = config.log_file {
        log_to_file(Path::new(log_path).canonicalize().unwrap().to_str().unwrap(), log_level);
    }
    if let Some(ref bk_path) = config.break_point_json {
        let breakpoint_path = Path::new(bk_path);
        if breakpoint_path.is_absolute() {

        } else {

        }
    }
}