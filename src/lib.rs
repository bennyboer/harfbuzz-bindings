#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    use super::*;

    #[test]
    fn should_print_harfbuzz_version() {
        unsafe {
            let harfbuzz_version = CStr::from_ptr(hb_version_string()).to_str().unwrap();
            println!("{}", harfbuzz_version);
        }
    }
}
