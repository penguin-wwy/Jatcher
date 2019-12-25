use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn info_log(message: *const c_char) {
    assert!(!message.is_null());
    let info_str = unsafe {
        CStr::from_ptr(message).to_str().unwrap()
    };
    info(info_str);
}

#[no_mangle]
pub unsafe extern "C" fn error_log(message: *const c_char) {
    assert!(!message.is_null());
    let err_str = unsafe {
        CStr::from_ptr(message).to_str().unwrap()
    };
    error(err_str)
}

#[no_mangle]
pub unsafe extern "C" fn warn_log(message: *const c_char) {
    assert!(!message.is_null());
    let err_str = unsafe {
        CStr::from_ptr(message).to_str().unwrap()
    };
    warn(err_str)
}

pub fn info(message: &str) {
    log::info!("{}", message);
}

pub fn warn(message: &str) {
    log::warn!("{}", message);
}

pub fn error(message: &str) {
    log::error!("{}", message);
}