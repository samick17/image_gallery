use std::os::raw::c_char;
use std::ffi::{CStr, CString};

#[no_mangle]
pub fn get_value_i32() -> i32 {
    10
}
#[no_mangle]
pub fn get_value_f32() -> f32 {
    10.5
}
#[no_mangle]
pub extern "C" fn pass_string(arg: *const c_char) -> *mut c_char {
    let arg_str = unsafe { CStr::from_ptr(arg).to_str().unwrap() };
    let c_string = CString::new(format!("{}{}", arg_str, ", value from rust")).unwrap();
    c_string.into_raw()
}
type Callback = extern "C" fn(i32);
#[no_mangle]
pub extern "C" fn fn_with_callback(callback: Callback) {
    callback(1);
    callback(2);
    callback(3);
    callback(4);
    callback(5);
}

