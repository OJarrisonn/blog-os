#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::{println, memory::BootInfoFrameAllocator};
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

// static HELLO: &[u8] = b"Hello World";
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!!!");
    blog_os::init();

    // ==================================================

    use blog_os::memory;
    use x86_64::{structures::paging::Page, VirtAddr};

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0xf021f077f065f04e) };

    // ==================================================

    #[cfg(test)]
    test_main();

    println!("It didn't crashed");
    blog_os::hlt_loop();
}

// Panic handlers
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}