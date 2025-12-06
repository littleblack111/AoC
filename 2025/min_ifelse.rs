#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let x = 1;
    let y = 2;
    let m = if x < y { x } else { y };
    unsafe { core::arch::asm!("mov rdi, {}", const m, "syscall", const 60u8); }
    loop {}
}
