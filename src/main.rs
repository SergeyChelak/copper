#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(copper::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use copper::{
    halt_loop,
    memory::{active_level_4_table, translate_addr},
    println,
};
use core::panic::PanicInfo;
use x86_64::{VirtAddr, structures::paging::PageTable};

entry_point!(kernel_start);

fn kernel_start(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    copper::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];
    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }

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
