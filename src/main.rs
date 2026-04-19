#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(copper::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use copper::{
    halt_loop,
    memory::{self, translate_addr},
    println,
};
use core::panic::PanicInfo;
use x86_64::{
    VirtAddr,
    structures::paging::{Page, Translate},
};

entry_point!(kernel_start);

fn kernel_start(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    copper::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    // map unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write something to display on screen thru memory mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    println!("It didn't crash!");
    halt_loop()
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    halt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    copper::test_panic_handler(info)
}
