#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod log;
mod sbi;
pub mod trap;
pub mod syscall;
mod batch;
pub mod sync;
pub mod trap;
use core::arch::global_asm;

use crate::sbi::shutdown;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    log::print_log();
    clear_bss();
    println!("hello world");
    // panic!("Shutdown machine!");
    shutdown(false)
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
