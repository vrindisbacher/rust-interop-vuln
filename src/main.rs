use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn call_vulnerable(input: *const c_char);
}

fn main() {
    // Create a string that exceeds the buffer size in C (more than 10 bytes)
    let input = "This string is way too long!";
    let c_input = CString::new(input).expect("CString::new failed");
    println!("MAKING AN UNSAFE CALL");
    unsafe {
        call_vulnerable(c_input.as_ptr());
    }
    // would not reach here
    println!("HELLOOOOO");
}
