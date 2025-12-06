#![no_std]
#![no_main]

use core::cmp::min;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let x = 1;
    let y = 2;
    let m = min(x, y);
    unsafe { core::arch::asm!("mov rdi, {}", const m, "syscall", const 60u8); }
    loop {}
}
