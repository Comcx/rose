#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rose::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rose::println;
use bootloader::{BootInfo, entry_point};


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    //use rose::memory::active_level_4_table;
    use rose::memory;
    use x86_64::VirtAddr;
    use x86_64::structures::paging::mapper::MapperAllSizes;

    println!("Hello World{}", "!");
    rose::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

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

    // Raise a page fault
    //let ptr = 0xdeadbeaf as *mut u32;
    //unsafe { *ptr = 42; }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    rose::hlt_loop();
}

/*
#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
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

    // Raise a page fault
    //let ptr = 0xdeadbeaf as *mut u32;
    //unsafe { *ptr = 42; }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    rose::hlt_loop();
}
*/


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
