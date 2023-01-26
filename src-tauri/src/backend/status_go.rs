use std::ffi::{CStr, CString};
extern crate libloading as lib;

fn get_status_lib_file() -> String {
    #[cfg(target_os = "linux")] {
        String::from("shared/libstatus.so")
    }
    #[cfg(target_os = "macos")] {
        String::from("shared/libstatus.dylib")
    }
    #[cfg(target_os = "windows")] {
        String::from("shared/libstatus.dll")
    }
}

pub struct StatusGo {
    lib: lib::Library
}

impl StatusGo {
    pub fn init() -> Result<StatusGo, String> {
        unsafe {
            let lib = lib::Library::new(get_status_lib_file());
            match lib {
                Ok(lib) => Ok(StatusGo { lib }),
                Err(err) => Err(err.to_string())
            }
        }
    }

    pub fn call_rpc(&self, rpc: &[u8], value: &str) -> String {
        let c_str = CString::new(value).unwrap();
        let c_raw: *mut ::std::os::raw::c_char = c_str.as_ptr() as *mut ::std::os::raw::c_char;

        unsafe {
            // TODO: handle function error instred of unwrap
            let func: lib::Symbol<unsafe extern "C" fn(str_: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char> = self.lib.get(rpc).unwrap();
            let res = func(c_raw);
            return CStr::from_ptr(res).to_str().unwrap().to_string();
        }
    }
}
