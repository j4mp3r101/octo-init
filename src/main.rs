#![no_std]
#![no_main]

mod asm;
mod better_proc_hand;
mod parser;
mod stage1;
mod stage2;
mod stage3;
mod stage4;

use core::panic::PanicInfo;

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

#[unsafe(no_mangle)]
pub extern "C" fn octo_main() -> ! {
    let ent = stage1();

    let q = stage2(ent);
    let int = stage3(q);
    stage4(int);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//Due to compiler issues had to add this:
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

//this one also doesnt work
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        core::arch::asm!(
            "rep movsb",
            inout("rdi") dest => _,
            inout("rsi") src => _,
            inout("rcx") n => _,
            options(nostack, preserves_flags)
        );
    }

    dest
}

//This things seems to not work
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    unsafe { memcmp(s1, s2, n) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlen(s: *const i8) -> usize {
    let mut count = 0;
    while unsafe { *s.add(count) != 0 } {
        count += 1;
    }
    count
}
