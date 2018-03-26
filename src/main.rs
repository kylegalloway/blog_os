#![feature(lang_items)] // required for defining the panic handler
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

extern crate rlibc;

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

static OUTPUT: &[u8] = b"Leeroy nnnJenkins!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *const u8 as *mut u8; // cast 0xb8000 into a raw pointer

    for (i, &byte) in OUTPUT.iter().enumerate() {
        // iterate over OUTPUT
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // write the string byte
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // write the colorbyte (0xb is light cyan)
        }
    }

    loop {}
}
