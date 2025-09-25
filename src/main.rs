use std::ffi::CString;
use std::os::raw::c_char;

unsafe extern "C" {
    fn hello_from_c(name: *const c_char);
}

fn main() {
    let name = CString::new("Jarek").unwrap();

    unsafe {
        hello_from_c(name.as_ptr());
    }
}
