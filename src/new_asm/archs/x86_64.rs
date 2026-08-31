#[repr(isize)]
#[derive(Copy, Clone)]
pub enum SyscallID {
    //fs
    Read = 0x00,
    Close = 0x03,
    Mkdir = 0x53,
    Sync = 0xA2,
    Mount = 0xA5,
    Umount2 = 0xA6,
    Getdents64 = 0xD9,
    Openat = 0x101,
    Renameat = 0x108,

    //symlinks
    Symlinkat = 0x10A,
    Unlinkat = 0x107,
    Readlinkat = 0x10B,

    //debug
    Write = 0x01,

    //signals
    Sigprocmask = 0x0E,
    Pause = 0x22,
    Nanosleep = 0x23,
    Reboot = 0xA9,
    RtSigtimedwait = 0x80,

    //procs
    Fork = 0x39,
    Execve = 0x3B,
    Exit = 0x3C,
    Wait4 = 0x3D,
    Kill = 0x3E,
    Setsid = 0x70,
}

#[allow(clippy::wrong_self_convention)]
pub trait SyscallArg {
    fn as_arg(self) -> usize;
}

impl SyscallArg for usize {
    #[inline(always)]
    fn as_arg(self) -> usize {
        self
    }
}
impl SyscallArg for isize {
    #[inline(always)]
    fn as_arg(self) -> usize {
        self as usize
    }
}
impl SyscallArg for i32 {
    #[inline(always)]
    fn as_arg(self) -> usize {
        self as usize
    }
}
impl SyscallArg for u32 {
    #[inline(always)]
    fn as_arg(self) -> usize {
        self as usize
    }
}
impl SyscallArg for u64 {
    #[inline(always)]
    fn as_arg(self) -> usize {
        self as usize
    }
}
impl SyscallArg for i64 {
    #[inline(always)]
    fn as_arg(self) -> usize {
        self as usize
    }
}

impl<T> SyscallArg for *const T {
    #[inline(always)]
    fn as_arg(self) -> usize {
        self as usize
    }
}

impl<T> SyscallArg for *mut T {
    #[inline(always)]
    fn as_arg(self) -> usize {
        self as usize
    }
}

pub mod raw_syscalls {
    use super::{SyscallArg, SyscallID};
    #[inline(always)]
    pub fn syscall0(nr: SyscallID) -> isize {
        let output: isize;

        unsafe {
            core::arch::asm!(
                "syscall",
                inout("rax") nr as usize => output,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        output
    }
    #[inline(always)]
    pub fn syscall1<A>(nr: SyscallID, arg0: A) -> isize
    where
        A: SyscallArg,
    {
        let output: isize;

        unsafe {
            core::arch::asm!(
                "syscall",
                inout("rax") nr as usize => output,
                in("rdi") arg0.as_arg(),
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        output
    }
    #[inline(always)]
    pub fn syscall2<A, B>(nr: SyscallID, arg0: A, arg1: B) -> isize
    where
        A: SyscallArg,
        B: SyscallArg,
    {
        let output: isize;

        unsafe {
            core::arch::asm!(
                "syscall",
                inout("rax") nr as usize => output,
                in("rdi") arg0.as_arg(),
                in("rsi") arg1.as_arg(),
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        output
    }
    #[inline(always)]
    pub fn syscall3<A, B, C>(nr: SyscallID, arg0: A, arg1: B, arg2: C) -> isize
    where
        A: SyscallArg,
        B: SyscallArg,
        C: SyscallArg,
    {
        let output: isize;

        unsafe {
            core::arch::asm!(
                "syscall",
                inout("rax") nr as usize => output,
                in("rdi") arg0.as_arg(),
                in("rsi") arg1.as_arg(),
                in("rdx") arg2.as_arg(),
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        output
    }
    #[inline(always)]
    pub fn syscall4<A, B, C, D>(nr: SyscallID, arg0: A, arg1: B, arg2: C, arg3: D) -> isize
    where
        A: SyscallArg,
        B: SyscallArg,
        C: SyscallArg,
        D: SyscallArg,
    {
        let output: isize;

        unsafe {
            core::arch::asm!(
                "syscall",
                inout("rax") nr as usize => output,
                in("rdi") arg0.as_arg(),
                in("rsi") arg1.as_arg(),
                in("rdx") arg2.as_arg(),
                in("r10") arg3.as_arg(),
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        output
    }
    #[inline(always)]
    pub fn syscall5<A, B, C, D, E>(
        nr: SyscallID,
        arg0: A,
        arg1: B,
        arg2: C,
        arg3: D,
        arg4: E,
    ) -> isize
    where
        A: SyscallArg,
        B: SyscallArg,
        C: SyscallArg,
        D: SyscallArg,
        E: SyscallArg,
    {
        let output: isize;

        unsafe {
            core::arch::asm!(
                "syscall",
                inout("rax") nr as usize => output,
                in("rdi") arg0.as_arg(),
                in("rsi") arg1.as_arg(),
                in("rdx") arg2.as_arg(),
                in("r10") arg3.as_arg(),
                in("r8") arg4.as_arg(),
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        output
    }
}

//After that comes the SPECIFIC codes.

pub const O_WRONLY_ID: usize = 1;
pub const O_TRUNC_ID: usize = 512;
pub const O_CREATE_ID: usize = 64;
