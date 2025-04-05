#![no_std]
#![feature(alloc_error_handler)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod task;
pub mod vga_buf;

use core::panic::PanicInfo;

#[cfg(test)] use bootloader::BootInfo;
#[cfg(test)] use bootloader::entry_point;

pub trait Testable {
	fn run(&self,);
}

impl<T: Fn(),> Testable for T {
	fn run(&self,) {
		serial_print!("{}..\t", core::any::type_name::<T,>());
		self();
		serial_println!("[ok]");
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
#[repr(u32)]
pub enum QemuExitCode {
	Success = 0x10,
	Failed  = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode,) {
	use x86_64::instructions::port::Port;

	unsafe {
		let mut port = Port::new(0xf4,);
		port.write(exit_code as u32,);
	}
}

#[cfg(test)]
entry_point!(test_kernel_main);

///Entry point for `cargo test`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo,) -> ! {
	init();
	test_main();
	hlt_loop();
}

pub fn test_runner(tests: &[&dyn Testable],) {
	serial_println!("Running {} tests", tests.len());
	for test in tests {
		test.run();
	}
	exit_qemu(QemuExitCode::Success,);
}

pub fn test_panic_handler(info: &PanicInfo,) -> ! {
	serial_println!("[failed]");
	serial_println!("Error: {}\n", info);
	exit_qemu(QemuExitCode::Failed,);
	hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo,) -> ! {
	test_panic_handler(info,)
}

pub fn init() {
	gdt::init();
	interrupts::init_idt();
	unsafe { interrupts::PICS.lock().initialize() };
	x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
	loop {
		x86_64::instructions::hlt();
	}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout,) -> ! {
	panic!("allocation error: {:?}", layout)
}
