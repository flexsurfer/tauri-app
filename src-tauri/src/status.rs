use std::ffi::{CStr, CString};
extern crate libloading as lib;

fn lib_file() -> String {
    #[cfg(target_os = "linux")]
    {
        String::from("shared/libstatus.so")
    }
    #[cfg(target_os = "macos")]
    {
        String::from("../shared/libstatus.dylib")
    }
    #[cfg(target_os = "windows")]
    {
        String::from("../shared/libstatus.dll")
    }
}

pub fn sha3(value: &str) -> Result<String, Box<dyn std::error::Error>> {
    let c_str = CString::new(value).unwrap();
    let c_raw: *mut ::std::os::raw::c_char = c_str.as_ptr() as *mut ::std::os::raw::c_char;

    unsafe {
        let lib = lib::Library::new(lib_file())?;
        let func: lib::Symbol<unsafe extern "C" fn(str_: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char> = lib.get(b"Sha3")?;
        let res = func(c_raw);
        Ok(CStr::from_ptr(res).to_str().unwrap() .to_string())
    }
}
