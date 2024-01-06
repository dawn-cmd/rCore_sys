#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod console;
mod lang_items;
mod sbi;
use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello World!!!");
    panic!("Shutdown machine!");
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
