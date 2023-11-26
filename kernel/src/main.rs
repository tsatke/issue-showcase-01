#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;

use bootloader_api::{entry_point, BootInfo};

entry_point!(main);

fn main(_: &'static mut BootInfo) -> ! {
    loop {}
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
