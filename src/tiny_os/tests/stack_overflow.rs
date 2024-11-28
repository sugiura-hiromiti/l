#![no_std]
#![feature(abi_x86_interrupt)]
#![no_main]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use tiny_os::exit_qemu;
use tiny_os::serial_print;
use tiny_os::serial_println;
use tiny_os::QemuExitCode;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;

lazy_static! {
   static ref TEST_IDT: InterruptDescriptorTable = {
      let mut idt = InterruptDescriptorTable::new();
      unsafe {
         idt.double_fault
            .set_handler_fn(test_double_fault_handler,)
            .set_stack_index(tiny_os::gdt::DOUBLE_FAULT_IST_INDEX,);
      }
      idt
   };
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
   serial_print!("stack_overflow::stackoverflow()..\t");
   tiny_os::gdt::init();
   init_test_idt();

   stack_overflow(); //trigger a stack overflow
   panic!("Execution continued after stack overflow");
}

extern "x86-interrupt" fn test_double_fault_handler(
   _stack_frame: InterruptStackFrame,
   _error_code: u64,
) -> ! {
   serial_println!("[ok]");
   exit_qemu(QemuExitCode::Success,);
   loop {}
}

pub fn init_test_idt() { TEST_IDT.load(); }

#[panic_handler]
fn panic(info: &PanicInfo,) -> ! { tiny_os::test_panic_handler(info,) }

#[allow(unconditional_recursion)]
fn stack_overflow() {
   stack_overflow();
   //prevent tail recursion optimizations
   volatile::Volatile::new(0,).read();
}
