use crate::info;
pub fn print_log() {
    extern "C" {
        static mut sdata: usize;
        static mut edata: usize;
        static mut srodata: usize;
        static mut erodata: usize;
        static mut stext: usize;
        static mut etext: usize;
        static mut sbss: usize;
        static mut ebss: usize;
    }
    unsafe {
        info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        // info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    }
}
