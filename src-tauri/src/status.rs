extern crate libloading as lib;

fn lib_file() -> String {
    #[cfg(target_os = "linux")]
    {
        String::from("shared/libstatus.so")
    }
    #[cfg(target_os = "macos")]
    {
        String::from("shared/libstatus.dylib")
    }
    #[cfg(target_os = "windows")]
    {
        String::from("shared/libstatus.dll")
    }
}

pub fn sha3(value: &str) -> Result<(), OCIError> {
    let go_str_ref = GoString {
        p: c_ref.as_ptr(),
        n: c_ref.as_bytes().len() as isize,
    };

    let lib = lib::Library::new(lib_file())?;
    unsafe {
        let func: lib::Symbol<unsafe extern "C" fn(r: GoString) -> i64> =
            lib.get(b"Sha3")?;
        match func(go_str_ref) {
            0 => return Ok(()),
            _ => return Err(()),
        }
    }
}