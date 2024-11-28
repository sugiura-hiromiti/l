//! writing an os by rust
#![no_std] //don't link rust standard library
#![no_main] //disable all rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(tiny_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::entry_point;
use bootloader::BootInfo;
use core::panic::PanicInfo;
use tiny_os::hlt_loop;
use tiny_os::println;
use tiny_os::task::executor::Executor;
use tiny_os::task::keyboard;
use tiny_os::task::Task;

static HELLO: &str = "hell on world, see you dream";

entry_point!(kernel_main);

/// - This `fn` is entry point of our kernel.
///By using bootloader::entry_point!, we can name arbitrarily.
///
///# Define
///
///```rust
/// entry_point!(kernel_main);
/// ```
fn kernel_main(boot_info: &'static BootInfo,) -> ! {
   println!("{}", HELLO);
   tiny_os::init();

   use tiny_os::allocator;
   use tiny_os::memory;
   use tiny_os::memory::BootInfoFrameAllocator;
   use x86_64::VirtAddr;

   let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset,);
   let mut mapper = unsafe { memory::init(phys_mem_offset,) };
   let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map,) };

   //allocate value to heap
   allocator::init_heap(&mut mapper, &mut frame_allocator,).expect("heap inititalization failed",);

   #[cfg(test)]
   test_main();

   let mut executor = Executor::new();
   executor.spawn(Task::new(example_task(),),);
   executor.spawn(Task::new(keyboard::print_keypresses(),),);
   executor.run();
}

///This `fn` is called on panic
#[cfg(not(test))] //new attribute
#[panic_handler]
fn panic(info: &PanicInfo,) -> ! {
   println!("{}", info);
   hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo,) -> ! { tiny_os::test_panic_handler(info,) }

#[test_case]
fn trivial_assertion() {
   assert_eq!(1, 1);
}

async fn async_number() -> u32 { 55 }

async fn example_task() {
   let n = async_number().await;
   println!("async number: {}", n);
}
