use libloading::{Library, Symbol};
use std::ffi::{CStr, CString};

fn main() {
    unsafe {
        let lib = Library::new(concat!(env!("OUT_DIR"), "/main.so")).unwrap();
        let run_command: Symbol<unsafe extern "C" fn(*const i8) -> *const i8> =
            lib.get(b"RunCommand").unwrap();

        let cmd = CString::new("ls").unwrap();
        let result_ptr = run_command(cmd.as_ptr());

        let result = CStr::from_ptr(result_ptr).to_str().unwrap();
        println!("Go returned: {}", result);
    }
}
