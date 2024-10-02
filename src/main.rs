use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn buffer_overflow_me(input: *const c_char);
    // Note the incorrect type here
    fn add_10(input: u8) -> u8;
}

fn buffer_overflow() {
    // Create a string that exceeds the buffer size in C (more than 10 bytes)
    let input = "This string is way too long!";
    let c_input = CString::new(input).expect("CString::new failed");
    println!("MAKING AN UNSAFE CALL");
    unsafe {
        buffer_overflow_me(c_input.as_ptr());
    }
    // would not reach here
    println!("HELLOOOOO");
}

fn integer_overflow() {
    // logic bug that will cause the result to wrap
    unsafe {
        println!("{}", add_10(std::u8::MAX));
    }
}

fn main() {
    // buffer_overflow();
    integer_overflow();
}
