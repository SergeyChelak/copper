#![no_std]
#![no_main]

use copper::{QemuExitCode, exit_qemu, halt_loop, serial_print, serial_println};
use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    halt_loop()
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test didn't panic]");
    exit_qemu(QemuExitCode::Failed);

    halt_loop()
}

#[allow(clippy::assertions_on_constants)]
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert!(false);
}
