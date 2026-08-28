#![no_std]
#![no_main]

mod better_proc_hand;
mod new_asm;
mod panic;
mod parser;
mod stage1;
mod stage2;
mod stage3;
mod stage4;

use stage1::stage1;
use stage2::stage2;
use stage3::stage3;
use stage4::stage4;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm!("and rsp, -16", "call octo_main", options(noreturn));
    };
}

//The massive issue happened quite easily -> BAD ALLOCATION!!!!!

#[unsafe(no_mangle)]
pub extern "C" fn octo_main() -> ! {
    let ent = stage1();

    stage2(ent);

    let int = stage3();

    stage4(int);
}

//panic handler which reboots on panic.
#[cfg(not(test))]
use core::panic::PanicInfo;
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    panic::panic()
}

//Due to compiler issues had to add this:
#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let a = unsafe { *s1.add(i) };
        let b = unsafe { *s2.add(i) };
        if a != b {
            return (a as i32) - (b as i32);
        }
    }
    0
}
#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    unsafe {
        core::arch::asm!(
            "rep stosb",
            inout("rdi") s => _,
            inout("rcx") n => _,
            in("al") c as u8,
            options(nostack, preserves_flags)
        )
    };

    s
}

#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let d = dest;
    let s = src;
    let count = n;

    unsafe {
        core::arch::asm!(
            "rep movsb",
            inout("rdi") d => _,
            inout("rsi") s => _,
            inout("rcx") count => _,
            options(nostack, preserves_flags)
        );
    }

    dest
}

//This things seems to not work
#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    unsafe { memcmp(s1, s2, n) }
}
#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlen(s: *const i8) -> usize {
    let mut count = 0;
    while unsafe { *s.add(count) != 0 } {
        count += 1;
    }
    count
}
