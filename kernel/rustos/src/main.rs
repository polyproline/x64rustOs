// #![feature(global_asm)]

#![no_std]
#![no_main]

use core::panic::PanicInfo;

// global_asm!(include_str!("head.S"));

#[no_mangle]
pub extern "C" fn Start_Kernel() -> ! {
    loop {}
}
/// 这个函数将在panic时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}