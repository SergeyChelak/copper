#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(copper::test_runner)]
#![reexport_test_harness_main = "test_main"]

use copper::{halt_loop, println};
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    copper::init();

    x86_64::instructions::interrupts::int3();

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
