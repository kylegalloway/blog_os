#![feature(panic_implementation)] // required for defining the panic handler
#![feature(abi_x86_interrupt)]
#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points if not testing
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))] // Silence the warnings when testing

#[macro_use]
extern crate blog_os;

use core::panic::PanicInfo;

extern crate x86_64;
use x86_64::structures::idt::{ExceptionStackFrame, Idt};

#[macro_use]
extern crate lazy_static;

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
#[cfg(not(test))]
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Leeroy {}Jenkins!", "nnnnnn");

    blog_os::gdt::init();
    init_idt();

    // invoke a breakpoint exception
    // x86_64::instructions::int3();

    // trigger a page fault
    // unsafe { *(0xdeadbeaf as *mut u64) = 42 };

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();

    println!("It didn't crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(blog_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    _error_code: u64,
) {
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}
