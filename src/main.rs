#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use blog_os::println;
use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;

entry_point!(kernel_main);

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::interrupts::PICS;
    use blog_os::memory::{self, create_example_mapping};

    println!("Hello World{}", "!");

    blog_os::gdt::init();
    blog_os::interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    let mut recursive_page_table = unsafe { memory::init(boot_info.p4_table_addr as usize) };
    let mut frame_allocator = memory::init_frame_allocator(&boot_info.memory_map);

    create_example_mapping(&mut recursive_page_table, &mut frame_allocator);
    unsafe { (0xdeadbeaf900 as *mut u64).write_volatile(0xf021f077f065f04e) };

    println!("It did not crash!");
    blog_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop()
}
