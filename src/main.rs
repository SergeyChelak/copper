#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;
mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Copper started");

    #[cfg(test)]
    test_main();

    halt_loop()
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    halt_loop()
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Exit: {}", info);
    exit_qemu(QemuExitCode::Failed);
    halt_loop()
}

#[allow(unused)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    tests.iter().for_each(|test| test.run());

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert!(true);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

fn halt_loop() -> ! {
    loop {
        unsafe {
            core::arch::asm! {
                "cli",
                "hlt"
            }
        }
    }
}
