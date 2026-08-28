#[cfg(target_arch = "x86_64")]
#[path = "archs/x86_64.rs"]
mod x86_64;
#[cfg(target_arch = "x86_64")]
use x86_64::{SyscallID, raw_syscalls};

//Now functions
pub mod prelude {
    pub const VOID_PTR: *const u8 = core::ptr::null();
}

pub mod fs {
    use super::SyscallID;
    use super::raw_syscalls::*;

    #[inline(always)]
    pub unsafe fn read(fd: isize, buf: &mut [u8]) -> isize {
        syscall3(SyscallID::Read, fd, buf.as_mut_ptr(), buf.len())
    }

    #[inline(always)]
    pub unsafe fn getdents64(fd: isize, dirp: *mut u8, count: usize) -> isize {
        syscall3(SyscallID::Getdents64, fd, dirp, count)
    }

    #[inline(always)]
    pub unsafe fn close(fd: isize) -> isize {
        syscall1(SyscallID::Close, fd)
    }

    #[inline(always)]
    pub unsafe fn sync() -> isize {
        syscall0(SyscallID::Sync)
    }

    #[inline(always)]
    pub unsafe fn openat(dfd: i32, name: *const u8, flags: usize, mode: usize) -> isize {
        syscall4(SyscallID::Openat, dfd, name, flags, mode)
    }

    #[inline(always)]
    pub unsafe fn renameat(
        oldfd: i32,
        old_name: *const u8,
        newfd: i32,
        new_name: *const u8,
    ) -> isize {
        syscall4(SyscallID::Renameat, oldfd, old_name, newfd, new_name)
    }

    #[inline(always)]
    pub unsafe fn mkdir(path: *const u8, mode: u32) -> isize {
        let res = syscall2(SyscallID::Mkdir, path, mode);
        if res == -17 { 0 } else { res }
    }

    #[inline(always)]
    pub unsafe fn mount(
        source: *const u8,
        target: *const u8,
        filesystem: *const u8,
        flags: usize,
        voidptr: *const u8,
    ) -> isize {
        syscall5(SyscallID::Mount, source, target, filesystem, flags, voidptr)
    }

    #[inline(always)]
    pub unsafe fn umount(target: *const u8, flags: isize) -> isize {
        syscall2(SyscallID::Umount2, target, flags)
    }
}

pub mod symlinks {
    use super::SyscallID;
    use super::raw_syscalls::*;

    #[inline(always)]
    pub unsafe fn unlinkat(dfd: i32, name: *const u8, flags: usize) -> isize {
        syscall3(SyscallID::Unlinkat, dfd, name, flags)
    }

    #[inline(always)]
    pub unsafe fn symlinkat(path_to_file: *const u8, fd: isize, path_for_link: *const u8) -> isize {
        syscall3(SyscallID::Symlinkat, path_to_file, fd, path_for_link)
    }
}

pub mod procs {
    use super::SyscallID;
    use super::raw_syscalls::*;

    #[inline(always)]
    pub unsafe fn setsid() {
        syscall0(SyscallID::Setsid);
    }

    #[inline(always)]
    pub unsafe fn exit(code: isize) {
        syscall1(SyscallID::Exit, code);
    }

    #[inline(always)]
    pub unsafe fn fork() -> i32 {
        syscall0(SyscallID::Fork) as i32
    }

    #[inline(always)]
    pub unsafe fn execve(
        argv_pointer: *const *const u8,
        envp_pointer: *const *const u8,
        file_name_ptr: *const u8,
    ) -> isize {
        syscall3(SyscallID::Execve, file_name_ptr, argv_pointer, envp_pointer)
    }

    #[inline(always)]
    pub unsafe fn wait4(pid: i32, flag: usize) -> i32 {
        let mut res: i32 = 0;
        syscall4(SyscallID::Wait4, pid, &mut res as *mut i32, flag, 0isize) as i32
    }

    #[inline(always)]
    pub unsafe fn kill(sig: isize, to: isize) {
        syscall2(SyscallID::Kill, to, sig);
    }
}

pub mod signals {
    use super::SyscallID;
    use super::prelude::VOID_PTR;
    use super::raw_syscalls::*;

    const TORVALDS_BIRTHDAY: u64 = 672274793;
    const FEELDEAD: u64 = 0xfee1dead;

    #[inline(always)]
    pub unsafe fn rt_sigtimedwait(mask: u64) -> isize {
        syscall4(
            SyscallID::RtSigtimedwait,
            &mask as *const u64,
            VOID_PTR,
            0_u64,
            8_u64,
        )
    }

    #[inline(always)]
    pub unsafe fn sigprocmask(mask: u64) {
        syscall4(
            SyscallID::Sigprocmask,
            0u64,
            &mask as *const u64,
            0_u64,
            8_u64,
        );
    }

    #[inline(always)]
    pub unsafe fn poweroff(cmd: u64) -> ! {
        syscall4(SyscallID::Reboot, FEELDEAD, TORVALDS_BIRTHDAY, cmd, 0u64);
        loop {
            syscall0(SyscallID::Pause);
        }
    }
}

pub mod time {
    use super::SyscallID;
    use super::raw_syscalls::*;

    #[repr(C)]
    pub struct Timespec {
        pub tv_sec: i64,
        pub tv_nsec: i64,
    }

    #[inline(always)]
    pub unsafe fn nanosleep(time: &Timespec) -> isize {
        syscall2(SyscallID::Nanosleep, time as *const Timespec, 0usize)
    }
}

pub mod debug {
    use super::SyscallID;
    use super::raw_syscalls::*;

    #[inline(always)]
    pub unsafe fn write(str: &[u8], to: usize) -> isize {
        syscall3(SyscallID::Write, to, str.as_ptr(), str.len())
    }

    #[inline(always)]
    pub fn print(str: &str) {
        unsafe { write(str.as_bytes(), 1) };
        unsafe { write(b"\n", 1) };
    }
}
