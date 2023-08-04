#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;

// static HELLO: &[u8] = b"Hello World";
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!!!");

    blog_os::init();

    //x86_64::instructions::interrupts::int3(); //Breakpoint

    #[cfg(test)]
    test_main();

    println!("It didn't break the code");

    loop { }
}

// Panic handlers
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop { }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}