//Here lies assembly code. NEATLY separated.//

//Ill separate by category.

pub mod fs {
    use core::arch::asm;

    const SYNC: isize = 0xa2;
    const GETDENTS64: isize = 0xd9;
    const OPENAT: isize = 0x101;
    const RENAMEAT: isize = 0x108;
    const MKDIR: isize = 0x53;
    const READ: isize = 0x00;
    const CLOSE: isize = 0x03;
    const MOUNT: isize = 0xa5;
    const UMOUNT2: isize = 0xa6;

    #[inline(always)]
    pub unsafe fn read(fd: isize, buf: &mut [u8]) -> isize {
        let res;
        unsafe {
            asm!(
                "syscall",
                inout("rax") READ => res,
                in("rdi") fd,
                in("rsi") buf.as_mut_ptr(),
                in("rdx") buf.len(),
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }
        res
    }

    #[inline(always)]
    pub unsafe fn getdents64(fd: isize, dirp: *mut u8, count: usize) -> isize {
        let mut res: isize = GETDENTS64;

        unsafe {
            asm!(
                "syscall",
                inout("rax") res,
                in("rdi") fd,
                in("rsi") dirp,
                in("rdx") count,
                options(nostack),
                clobber_abi("system")
            )
        };

        res
    }

    #[inline(always)]
    pub unsafe fn close(fd: isize) -> isize {
        let res: isize;
        unsafe {
            asm!(
                "syscall",
                inout("rax") CLOSE => res,
                in("rdi") fd,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        };
        res
    }

    #[inline(always)]
    pub unsafe fn sync() -> isize {
        let mut res: isize = SYNC;
        unsafe {
            asm!(
                "syscall",
                inout("rax") res,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        };
        res
    }

    #[inline(always)]
    pub unsafe fn openat(dfd: i32, name: *const u8, flags: usize, mode: usize) -> isize {
        let res;

        unsafe {
            asm!(
                "syscall",
                inout("rax") OPENAT => res,
                in("rdi") dfd,
                in("rsi") name,
                in("rdx") flags,
                in("r10") mode,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        res
    }

    #[inline(always)]
    pub unsafe fn renameat(
        oldfd: i32,
        old_name: *const u8,
        newfd: i32,
        new_name: *const u8,
    ) -> isize {
        let res;

        unsafe {
            asm!(
                "syscall",
                inout("rax") RENAMEAT => res,
                in("rdi") oldfd,
                in("rsi") old_name,
                in("rdx") newfd,
                in("r10") new_name,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        res
    }

    #[inline(always)]
    pub unsafe fn mkdir(path: *const u8, mode: u32) -> isize {
        let mut res: isize = MKDIR;
        unsafe {
            asm!(
                "syscall",
                inout("rax") res,
                in("rdi") path,
                in("rsi") mode,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }
        if res == -17 {
            res = 0;
        }

        res
    }

    #[inline(always)]
    pub unsafe fn mount(
        source: *const u8,
        target: *const u8,
        filesystem: *const u8,
        flags: usize,
        voidptr: *const u8,
    ) -> isize {
        let mut res: isize = MOUNT;
        unsafe {
            asm!(
                "syscall",
                inout("rax") res,
                in("rdi") source,
                in("rsi") target,
                in("rdx") filesystem,
                in("r10") flags,
                in("r8") voidptr,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        res
    }

    #[inline(always)]
    pub unsafe fn umount(target: *const u8, flags: isize) -> isize {
        let mut res: isize = UMOUNT2;

        unsafe {
            asm!(
                "syscall",
                inout("rax") res,
                in("rdi") target,
                in("rsi") flags,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        res
    }
}

pub mod symlinks {
    use core::arch::asm;

    const UNLINKAT: isize = 0x107;
    const SYMLINKAT: isize = 0x10a;

    #[inline(always)]
    pub unsafe fn unlinkat(dfd: i32, name: *const u8, flags: usize) -> isize {
        let res;

        unsafe {
            asm!(
                "syscall",
                inout("rax") UNLINKAT => res,
                in("rdi") dfd,
                in("rsi") name,
                in("rdx") flags,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        res
    }

    #[inline(always)]
    pub unsafe fn symlinkat(path_to_file: *const u8, fd: isize, path_for_link: *const u8) -> isize {
        let mut res: isize = SYMLINKAT;
        unsafe {
            asm!(
                "syscall",
                inout("rax") res,
                in("rdi") path_to_file,
                in("rsi") fd,
                in("rdx") path_for_link,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }
        res
    }
}

pub mod procs {
    use core::arch::asm;

    const KILL: isize = 0x3E;
    const FORK: i32 = 0x39;
    const EXECVE: isize = 0x3b;
    const EXIT: isize = 0x3c;
    const SETSID: isize = 0x70;
    const DUP2: isize = 0x21;
    const IOCTL: isize = 0x10;
    const WAIT4: i32 = 0x3d;

    #[inline(always)]
    pub unsafe fn ioctl(fd: isize, cmd: isize, long_arg: isize) -> isize {
        let mut res: isize = IOCTL;

        unsafe {
            asm!(
                "syscall",
                inout("rax") res,
                in("rdi") fd,
                in("rsi") cmd,
                in("rdx") long_arg,
                options(nostack),
                clobber_abi("system")
            )
        };

        res
    }

    #[inline(always)]
    pub unsafe fn dup2(oldfd: isize, newfd: isize) {
        let mut _res: isize = DUP2;

        unsafe {
            asm!(
                "syscall",
                inout("rax") _res,
                in("rdi") oldfd,
                in("rsi") newfd,
                options(nostack),
                clobber_abi("system")
            )
        };
    }

    #[inline(always)]
    pub unsafe fn setsid() {
        let mut _res: isize = SETSID;
        unsafe {
            asm!(
                "syscall",
                inout("rax") _res,
                options(nostack),
                clobber_abi("system")
            )
        };
    }

    #[inline(always)]
    pub unsafe fn exit(code: isize) -> ! {
        unsafe {
            asm!(
                "syscall",
                in("rax") EXIT,
                in("rdi") code,
                options(noreturn)
            );
        }
    }

    #[inline(always)]
    pub unsafe fn fork() -> i32 {
        let mut pid: i32 = FORK;
        unsafe {
            asm!(
                "syscall",
                inout("rax") pid,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        };
        pid
    }

    #[inline(always)]
    pub unsafe fn execve(
        argv_pointer: *const *const u8,
        envp_pointer: *const *const u8,
        file_name_ptr: *const u8,
    ) -> isize {
        let mut res: isize = EXECVE;

        unsafe {
            asm!(
                "syscall",
                inout("rax") res,
                in("rdi") file_name_ptr,
                in ("rsi") argv_pointer,
                in ("rdx") envp_pointer,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        res
    }

    #[inline(always)]
    pub unsafe fn wait4(pid: i32, flag: usize) -> i32 {
        let mut res: i32 = 0;
        let mut out_val: i32 = WAIT4;

        unsafe {
            asm!(
                "syscall",
                inout("rax") out_val,
                in("rdi") pid,
                in("rsi") &mut res as *mut i32,
                in("rdx") flag,
                in("r10") 0isize,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        out_val
    }

    #[inline(always)]
    pub unsafe fn kill(sig: isize, to: isize) {
        let mut _res: isize = KILL;
        unsafe {
            asm!(
                "syscall",
                inout("rax") _res,
                in("rdi") to,
                in("rsi") sig,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        };
    }
}

pub mod signals {
    use core::arch::asm;

    use crate::asm::prelude::VOID_PTR;

    const SIGPROCMASK: isize = 0x0e;
    const RT_SIGTIMEDWAIT: isize = 0x80;

    #[inline(always)]
    pub unsafe fn rt_sigtimedwait(mask: u64) -> isize {
        let mut res: isize = RT_SIGTIMEDWAIT;
        unsafe {
            asm!(
                "syscall",
                inout("rax") res,
                in("rdi") &mask as *const u64,
                in("rsi") VOID_PTR,
                in("rdx") 0_u64,
                in("r10") 8_u64,
                options(nostack),
                clobber_abi("system")
            )
        };

        res
    }

    #[inline(always)]
    pub unsafe fn sigprocmask(mask: u64) {
        let mut _res: isize = SIGPROCMASK;
        unsafe {
            asm!(
                "syscall",
                inout("rax") _res,
                in("rdi") 0u64,
                in("rsi") &mask as *const u64,
                in("rdx") 0_u64,
                in("r10") 8_u64,
                options(nostack),
                clobber_abi("system")
            )
        };
    }

    const TORVALDS_BIRTHDAY: u64 = 672274793;
    const FEELDEAD: u64 = 0xfee1dead;

    #[inline(always)]
    pub unsafe fn poweroff(cmd: u64) -> ! {
        unsafe {
            asm!(
                "syscall",
                in("rax") 169u64,
                in("rdi") FEELDEAD,
                in("rsi") TORVALDS_BIRTHDAY,
                in("rdx") cmd,
                in("r10") 0u64,
                clobber_abi("system")
            );

            loop {
                asm!(
                    "syscall",
                    in("rax") 34,
                    lateout("rax") _,
                    clobber_abi("system")
                );
            }
        }
    }
}

pub mod time {
    use core::arch::asm;
    #[repr(C)]
    pub struct Timespec {
        pub tv_sec: i64,
        pub tv_nsec: i64,
    }
    const NANOSLEEP: isize = 0x23;

    #[inline(always)]
    pub unsafe fn nanosleep(time: &Timespec) -> isize {
        let mut res: isize = NANOSLEEP;
        unsafe {
            asm!(
                "syscall",
                inout("rax") res,
                in("rdi") time as *const Timespec,
                in("rsi") 0usize,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        };
        res
    }
}

pub mod debug {
    use core::arch::asm;
    const WRITE: isize = 0x01;

    #[inline(always)]
    pub unsafe fn write(str: &[u8], to: usize) -> isize {
        let mut res = WRITE;

        unsafe {
            asm!(
                "syscall",
                inout("rax") res,
                in("rdi") to,
                in("rsi") str.as_ptr(),
                in("rdx") str.len(),
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        res
    }

    #[inline(always)]
    pub fn print(str: &str) {
        unsafe { write(str.as_bytes(), 1) };
        unsafe { write(&[b'\n'], 1) };
    }
}

pub mod prelude {
    pub const VOID_PTR: *const u8 = core::ptr::null();
}
