#![no_std]
#![no_main]
#![feature(linkage)]

mod sys;
mod vectors;

pub fn main() {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn custom_fault_handler() {
    loop {}
}
