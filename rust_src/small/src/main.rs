// #![no_std]
// #![no_main]
//
// extern crate libc;
// fn main() {
//     std::process::exit(0);
// }

#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
