#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::println;

// Entry point for the program due to its exported name
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    os::init();

    #[cfg(test)]
    test_main();

    os::hlt_loop();
}

// Function that will get called in case of a panic. It never returns (as marked by returning `!`)
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os::hlt_loop();
}

// Special panic handler for testing to use serial output
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
