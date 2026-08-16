use core::arch;

//This asm will work on arm.
trait SyscallArg {
    fn get_as_usize(self) -> usize;
}
impl SyscallArg for u8 {
    #[inline(always)]
    fn get_as_usize(self) -> usize {
        self as usize
    }
}
impl SyscallArg for i8 {
    #[inline(always)]
    fn get_as_usize(self) -> usize {
        self as usize
    }
}
impl SyscallArg for u16 {
    #[inline(always)]
    fn get_as_usize(self) -> usize {
        self as usize
    }
}
impl SyscallArg for i16 {
    #[inline(always)]
    fn get_as_usize(self) -> usize {
        self as usize
    }
}
impl SyscallArg for u32 {
    #[inline(always)]
    fn get_as_usize(self) -> usize {
        self as usize
    }
}
impl SyscallArg for i32 {
    #[inline(always)]
    fn get_as_usize(self) -> usize {
        self as usize
    }
}
impl SyscallArg for u64 {
    #[inline(always)]
    fn get_as_usize(self) -> usize {
        self as usize
    }
}
impl SyscallArg for i64 {
    #[inline(always)]
    fn get_as_usize(self) -> usize {
        self as usize
    }
}
impl SyscallArg for usize {
    #[inline(always)]
    fn get_as_usize(self) -> usize {
        self
    }
}
impl SyscallArg for isize {
    #[inline(always)]
    fn get_as_usize(self) -> usize {
        self as usize
    }
}

impl<T> SyscallArg for *const T {
    #[inline(always)]
    fn get_as_usize(self) -> usize {
        self as usize
    }
}
impl<T> SyscallArg for *mut T {
    #[inline(always)]
    fn get_as_usize(self) -> usize {
        self as usize
    }
}

#[inline(always)]
pub fn syscall6<A2, A3, A4, A5, A6, A7>(a: i32, b: A2, c: A3, d: A4, e: A5, f: A6, g: A7) -> i32
where
    A2: SyscallArg,
    A3: SyscallArg,
    A4: SyscallArg,
    A5: SyscallArg,
    A6: SyscallArg,
    A7: SyscallArg,
{
    let result: i32;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        arch::asm!("syscall",
            in("rax") a,
            in("rdi") b.get_as_usize(),
            in("rsi") c.get_as_usize(),
            in("rdx") d.get_as_usize(),
            in("r10") e.get_as_usize(),
            in("r8") f.get_as_usize(),
            in("r9") g.get_as_usize(),
            lateout("rax") result,
            options(nostack),
            out("rcx") _,
            out("r11") _
        )
    };

    #[cfg(target_arch = "aarch64")]
    unsafe {
        arch::asm!("svc #0",
            in("x8") a,
            in("x0") b.get_as_usize(),
            in("x1") c.get_as_usize(),
            in("x2") d.get_as_usize(),
            in("x3") e.get_as_usize(),
            in("x4") f.get_as_usize(),
            in("x5") g.get_as_usize(),
            lateout("x0") result,
            options(nostack)
        )
    };

    result
}

#[inline(always)]
pub fn syscall5<A2, A3, A4, A5, A6>(a: i32, b: A2, c: A3, d: A4, e: A5, f: A6) -> i32
where
    A2: SyscallArg,
    A3: SyscallArg,
    A4: SyscallArg,
    A5: SyscallArg,
    A6: SyscallArg,
{
    let result: i32;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        arch::asm!("syscall",
            in("rax") a,
            in("rdi") b.get_as_usize(),
            in("rsi") c.get_as_usize(),
            in("rdx") d.get_as_usize(),
            in("r10") e.get_as_usize(),
            in("r8") f.get_as_usize(),
            lateout("rax") result,
            options(nostack),
            out("rcx") _,
            out("r11") _
        )
    };

    #[cfg(target_arch = "aarch64")]
    unsafe {
        arch::asm!("svc #0",
            in("x8") a,
            in("x0") b.get_as_usize(),
            in("x1") c.get_as_usize(),
            in("x2") d.get_as_usize(),
            in("x3") e.get_as_usize(),
            in("x4") f.get_as_usize(),
            lateout("x0") result,
            options(nostack)
        )
    };

    result
}

#[inline(always)]
pub fn syscall4<A2, A3, A4, A5>(a: i32, b: A2, c: A3, d: A4, e: A5) -> i32
where
    A2: SyscallArg,
    A3: SyscallArg,
    A4: SyscallArg,
    A5: SyscallArg,
{
    let result: i32;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        arch::asm!("syscall",
            in("rax") a,
            in("rdi") b.get_as_usize(),
            in("rsi") c.get_as_usize(),
            in("rdx") d.get_as_usize(),
            in("r10") e.get_as_usize(),
            lateout("rax") result,
            options(nostack),
            out("rcx") _,
            out("r11") _
        )
    };

    #[cfg(target_arch = "aarch64")]
    unsafe {
        arch::asm!("svc #0",
            in("x8") a,
            in("x0") b.get_as_usize(),
            in("x1") c.get_as_usize(),
            in("x2") d.get_as_usize(),
            in("x3") e.get_as_usize(),
            lateout("x0") result,
            options(nostack)
        )
    };

    result
}

#[inline(always)]
pub fn syscall3<A2, A3, A4>(a: i32, b: A2, c: A3, d: A4) -> i32
where
    A2: SyscallArg,
    A3: SyscallArg,
    A4: SyscallArg,
{
    let result: i32;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        arch::asm!("syscall",
            in("rax") a,
            in("rdi") b.get_as_usize(),
            in("rsi") c.get_as_usize(),
            in("rdx") d.get_as_usize(),
            lateout("rax") result,
            options(nostack),
            out("rcx") _,
            out("r11") _
        )
    };

    #[cfg(target_arch = "aarch64")]
    unsafe {
        arch::asm!("svc #0",
            in("x8") a,
            in("x0") b.get_as_usize(),
            in("x1") c.get_as_usize(),
            in("x2") d.get_as_usize(),
            lateout("x0") result,
            options(nostack)
        )
    };

    result
}

#[inline(always)]
pub fn syscall2<A2, A3>(a: i32, b: A2, c: A3) -> i32
where
    A2: SyscallArg,
    A3: SyscallArg,
{
    let result: i32;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        arch::asm!("syscall",
            in("rax") a,
            in("rdi") b.get_as_usize(),
            in("rsi") c.get_as_usize(),
            lateout("rax") result,
            options(nostack),
            out("rcx") _,
            out("r11") _
        )
    };

    #[cfg(target_arch = "aarch64")]
    unsafe {
        arch::asm!("svc #0",
            in("x8") a,
            in("x0") b.get_as_usize(),
            in("x1") c.get_as_usize(),
            lateout("x0") result,
            options(nostack)
        )
    };

    result
}

#[inline(always)]
pub fn syscall1<A2>(a: i32, b: A2) -> i32
where
    A2: SyscallArg,
{
    let result: i32;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        arch::asm!("syscall",
            in("rax") a,
            in("rdi") b.get_as_usize(),
            lateout("rax") result,
            options(nostack),
            out("rcx") _,
            out("r11") _
        )
    };

    #[cfg(target_arch = "aarch64")]
    unsafe {
        arch::asm!("svc #0",
            in("x8") a,
            in("x0") b.get_as_usize(),
            lateout("x0") result,
            options(nostack)
        )
    };

    result
}

#[inline(always)]
pub fn syscall0(a: i32) -> i32 {
    let result: i32;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        arch::asm!("syscall",
            in("rax") a,
            lateout("rax") result,
            options(nostack),
            out("rcx") _,
            out("r11") _
        )
    };

    #[cfg(target_arch = "aarch64")]
    unsafe {
        arch::asm!("svc #0",
            in("x8") a,
            lateout("x0") result,
            options(nostack)
        )
    };

    result
}
//alot of repeating code but it works so :/
