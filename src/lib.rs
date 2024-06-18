#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(c_size_t)]

mod vga_buffer;
mod io;
mod keyboard;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn kmain() -> ! {
    // this function is the entry point, since the linker looks for a function

    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    // vga_buffer::print_something();

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    println!("Hello World{}", "!");
    println!("42");

    loop {
        if let Some(scancode) = keyboard::read_key() {
            if let Some(key) = keyboard::translate_scancode(scancode) {
                print!("{}", key);
            }
        }
    }
}
