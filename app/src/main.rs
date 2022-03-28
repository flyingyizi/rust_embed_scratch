
#![no_std]
#![no_main]



static RODATA: &[u8] = b"Hello, world!";
static mut BSS: u8 = 0;
static mut DATA: u16 = 1;

//extern crate rt;
rt::entry!(main);
fn main() -> ! {
    let _x = RODATA;
    let _y = unsafe { &BSS };
    let _z = unsafe { &DATA };

    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}