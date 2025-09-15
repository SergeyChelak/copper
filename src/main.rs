#![no_std]
#![no_main]
use core::panic::PanicInfo;

// mod spinlock;
mod unsafe_ptr;
mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Copper started");
    halt_loop()
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    halt_loop()
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
