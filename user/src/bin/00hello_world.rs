#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
pub fn main() -> i32 {
    println!("Hello world from user mode program!");
    // unsafe {
    //     llvm_asm!("sret");
    // }
    0
}
