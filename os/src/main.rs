#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod console;
mod lang_items;
mod sbi;
use core::arch::global_asm;

use crate::sbi::shutdown;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    extern "C" {
        static mut sdata: usize;
        static mut edata: usize;
        static mut srodata: usize;
        static mut erodata: usize;
        static mut stext: usize;
        static mut etext: usize;
    }
    unsafe {
        info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        error!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    }

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
