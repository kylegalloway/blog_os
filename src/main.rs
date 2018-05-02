#![feature(lang_items)] // required for defining the panic handler
#![feature(const_fn)] // allows for functions to be marked as const and called in const scopes
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

#[macro_use]
mod vga_buffer;

#[cfg(not(test))] // Only compile when the test flag is not set
#[lang = "panic_fmt"] // define the function that should be called on panic
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _column: u32,
) -> ! {
    loop {}
}

#[cfg(not(test))] // Only compile when the test flag is not set
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Leeroy {}Jenkins!", "nnnnnn");

    loop {}
}
