#![no_std]
#![no_main]

mod lang_items;
use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    loop {}
}

fn clear_bss() {
    extern "C" {
        static mut sbss: usize;
        static mut ebss: usize;
    }
    unsafe {
        (sbss..ebss).for_each(|a| { (a as *mut u8).write_volatile(0) });
    }
}
