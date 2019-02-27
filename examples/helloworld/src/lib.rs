extern "C" {
    fn send(start: i32);
}

fn cstr(s:&str) -> i32{
    std::ffi::CString::new(s).unwrap().into_raw() as i32
}

#[no_mangle]
pub fn execute() -> () {
    unsafe {
        send(cstr(r#"{"status":200,"body":"Hello World!"}"#));
    }
}
