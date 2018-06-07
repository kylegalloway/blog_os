#![feature(panic_implementation)] // required for defining the panic handler
#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points if not testing
#![cfg_attr(test, allow(dead_code, unused_macros))] // Silence the warnings when testing

#[cfg(test)] // Only import if testing
extern crate std;

#[cfg(test)]
extern crate array_init;

extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
extern crate lazy_static;

#[cfg(not(test))]
use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(not(test))] // Only compile when the test flag is not set
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Leeroy {}Jenkins!", "nnnnnn");

    loop {}
}
