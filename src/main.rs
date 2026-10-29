#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CreateOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader as _;
use CreateOS::println;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", " I love Coding");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    CreateOS::test_panic_handler(info);
}