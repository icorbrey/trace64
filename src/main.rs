#![no_main]
#![no_std]

mod hardware;

use core::panic::PanicInfo;
use hardware::video_interface;

#[no_mangle]
fn main() {
    video_interface::init();
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
