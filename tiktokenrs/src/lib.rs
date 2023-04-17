// use the lib c to make call on c
use std::os::raw::c_char;
use std::ffi::{CStr, CString};

// it is used as interface to make call on c with them
// description: convert c_char to string in rust
// input:       *const name     c_char
// output:      *mut            c_char
pub extern "C" fn hello_to_my_name(name: *const c_char) -> *mut c_char {
    let name: &str = unsafe { CStr::from_ptr(name).to_str().unwrap() };
    let result: String =  format!("Hello, {}!", name);
    let result: CString = CString::new(result).unwrap();
    result.into_raw()
}

fn main() {
    println!("Hello, world!");
}
