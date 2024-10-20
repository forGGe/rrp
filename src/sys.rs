use core::{panic::PanicInfo, ptr};

use crate::main;

pub unsafe fn early_main() {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    // Zero-out BSS section
    let count = &raw const _ebss as *const u8 as usize
        - &raw const _sbss as *const u8 as usize;
    ptr::write_bytes(&raw mut _sbss as *mut u8, 0, count);

    // Copy data section from FLASH to RAM
    let count = &raw const _edata as *const u8 as usize
        - &raw const _sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(
        &_sidata as *const u8,
        &raw mut _sdata as *mut u8,
        count,
    );

    main();
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
