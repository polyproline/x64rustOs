// #![feature(global_asm)]

#![no_std]
#![no_main]
use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr::write_volatile;


use rustos::display::{PutChar,Screen,ScreenColor,SCREEN_HEIGHT,SCREEN_WIDTH};

// global_asm!(include_str!("head.S"));

#[no_mangle]
pub extern "C" fn Start_Kernel() -> ! {
    let mut screen = Screen.write();
    let mut putchar = PutChar::new(&mut screen,0,0,ScreenColor::new(0,0,0,0));
    {
        let mut addr = 0xffff800000a00000 as *mut u8;
        unsafe{
            for _ in 0..SCREEN_WIDTH*20{
                write_volatile(addr.offset(0),0x00);
                write_volatile(addr.offset(1),0x00);
                write_volatile(addr.offset(2),0xff);
                write_volatile(addr.offset(3),0x00);
                addr = addr.offset(4);
            }
            for _ in 0..SCREEN_WIDTH*20{
                write_volatile(addr.offset(0),0x00);
                write_volatile(addr.offset(1),0xff);
                write_volatile(addr.offset(2),0x00);
                write_volatile(addr.offset(3),0x00);
                addr = addr.offset(4);
            }
            for _ in 0..SCREEN_WIDTH*20{
                write_volatile(addr.offset(0),0xff);
                write_volatile(addr.offset(1),0x00);
                write_volatile(addr.offset(2),0x00);
                write_volatile(addr.offset(3),0x00);
                addr = addr.offset(4);
            }
            for _ in 0..SCREEN_WIDTH*20{
                write_volatile(addr.offset(0),0xff);
                write_volatile(addr.offset(1),0xff);
                write_volatile(addr.offset(2),0xff);
                write_volatile(addr.offset(3),0x00);
                addr = addr.offset(4);
            }
        }
    }
    writeln!(putchar,"Hello world!").unwrap();
    writeln!(putchar,"today is {} year {} m {} day",2020,11,24).unwrap();
    loop {}
}
/// 这个函数将在panic时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
