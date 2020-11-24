// #![feature(global_asm)]

#![no_std]
#![no_main]
use core::fmt::Write;
use core::panic::PanicInfo;

// global_asm!(include_str!("head.S"));

#[no_mangle]
pub extern "C" fn Start_Kernel() -> ! {
    use rustos::display::{PutChar,Screen,ScreenColor};

    let mut screen = Screen.write();
    // let mut putchar = PutChar::new(&mut screen,0,0,ScreenColor::new(0,0,0,0));
    // writeln!(putchar,"Hello world!").unwrap();
    // writeln!(putchar,"today is {} year {} m {} day",2020,11,24).unwrap();
    loop {}
}
/// 这个函数将在panic时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
