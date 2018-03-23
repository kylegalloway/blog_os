#![feature(lang_items)] // required for defining the panic handler
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

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

// Windows:
// compile with `cargo build`
// #[no_mangle]
// pub extern "C" fn WinMainCRTStartup() -> ! {
//     WinMain();
// }

// #[no_mangle]
// pub extern "C" fn WinMain() -> ! {
//     loop {}
// }

// Linux:
// compile with `cargo rustc -- -Z pre-link-arg=-nostartfiles`
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

// macOS:
// compile with `cargo rustc -- -Z pre-link-arg=-lSystem
// #[no_mangle]
// pub extern "C" fn main() -> ! {
//     loop {}
// }
