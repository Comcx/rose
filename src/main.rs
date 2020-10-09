#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rose::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rose::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rose::init();

    // invoke a breakpoint exception
    //x86_64::instructions::interrupts::int3();

    // trigger a page fault
    /*
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };
    */

    // Recursive double fault
    /*
    fn recur() {
        recur();
    }
    recur();
    */

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    rose::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rose::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rose::test_panic_handler(info)
}
