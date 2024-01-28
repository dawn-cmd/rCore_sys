#![feature(linkage)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]
#[macro_use]
pub mod console;
mod lang_items;
mod log;
mod sbi;
mod syscall;
use syscall::*;
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit;!");
}
#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}
fn clear_bss() {
    extern "C" {
        static mut sbss: usize;
        static mut ebss: usize;
    }
    unsafe {
        (sbss..ebss).for_each(|a| (a as *mut u8).write_volatile(0));
    }
}
