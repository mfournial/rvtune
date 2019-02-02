#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn lol() {
        unsafe {assert_eq!(1, get_one())}
        // unsafe {__itt_pause();}
    }

}
