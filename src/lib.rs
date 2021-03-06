#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[link(name = "ittnotify", kind = "static")]
extern "C"
{
    #[link_name = "__itt_pause_init_3_0"]
    pub fn __itt_pause();
}

#[cfg(test)]
mod LinkingTests {
    use super::*;

    #[test]
    pub fn linking_is_working() {
        unsafe {
            __itt_pause();
        }
    }
}
