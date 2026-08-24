//Here lies assembly code. NEATLY separated.//

//Also trying to incorporate modularity 0_0

pub mod easyc {
    use core::arch::asm;

    const OPENAT: isize = 0x101;
    const CLOSE: isize = 0x03;
    const READ: isize = 0x00;

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
}

pub mod proc_hand {
    use core::arch::asm;

    const OPENAT: isize = 0x101;
    const RENAMEAT: isize = 0x108;
    const UNLINKAT: isize = 0x107;

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
}

pub mod signal_processing {
    use core::arch::asm;

    //syscalls
    const EPOLL_WAIT: isize = 0xe8;
    const EPOLL_CREATE1: isize = 0x126;
    const EPOLL_CTL: isize = 0xe9;

    const SIGNAL_FD4: isize = 0x121;

    #[inline(always)]
    pub unsafe fn signalfd4(fd: isize, mask: *const u8, sizemask: usize, flags: i32) -> isize {
        let mut res: isize = SIGNAL_FD4;
        unsafe {
            asm!(
                "syscall",
                inout("rax") res,
                in("rdi") fd,
                in("rsi") mask,
                in("rdx") sizemask,
                in("r10") flags,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        res
    }

    //implementations
    #[inline(always)]
    pub unsafe fn epoll_create1(flags: i32) -> isize {
        let mut res: isize = EPOLL_CREATE1;
        unsafe {
            asm!(
                "syscall",
                inout("rax") res,
                in("rdi") flags,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        res
    }

    #[inline(always)]
    pub unsafe fn epoll_ctl(epfd: isize, op: i32, fd: isize, event: *const u8) -> isize {
        let mut res: isize = EPOLL_CTL;
        unsafe {
            asm!(
                "syscall",
                inout("rax") res,
                in("rdi") epfd,
                in("rsi") op,
                in("rdx") fd,
                in("r10") event,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        res
    }

    #[inline(always)]
    pub unsafe fn epoll_wait(epfd: isize, events: *mut u8, maxevents: i32, timeout: i32) -> isize {
        let mut res: isize = EPOLL_WAIT;
        unsafe {
            asm!(
                "syscall",
                inout("rax") res,
                in("rdi") epfd,
                in("rsi") events,
                in("rdx") maxevents,
                in("r10") timeout,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        res
    }
}

pub mod stage1 {
    use core::arch;

    const MOUNT: isize = 0xa5;
    const MKDIR: isize = 0x53;
    const SYMLINKAT: isize = 0x10a;
    const ACCEPT: isize = 43;
    const SOCKET: isize = 41;
    const BIND: isize = 49;
    const LISTEN: isize = 50;

    #[inline(always)]
    pub unsafe fn accept(sockfd: isize, addr: *mut u8, addrlen: *mut u32) -> isize {
        let mut res: isize = ACCEPT;
        unsafe {
            arch::asm!(
                "syscall",
                inout("rax") res,
                in("rdi") sockfd,
                in("rsi") addr,
                in("rdx") addrlen,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        res
    }

    #[inline(always)]
    pub unsafe fn socket(domain: i32, socket_type: i32, protocol: i32) -> isize {
        let mut res: isize = SOCKET;
        unsafe {
            arch::asm!(
                "syscall",
                inout("rax") res,
                in("rdi") domain as isize,
                in("rsi") socket_type as isize,
                in("rdx") protocol as isize,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        res
    }

    #[inline(always)]
    pub unsafe fn bind(sockfd: isize, addr: *const u8, addrlen: u32) -> isize {
        let mut res: isize = BIND;
        unsafe {
            arch::asm!(
                "syscall",
                inout("rax") res,
                in("rdi") sockfd,
                in("rsi") addr,
                in("rdx") addrlen as isize,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        res
    }

    #[inline(always)]
    pub unsafe fn listen(sockfd: isize, backlog: i32) -> isize {
        let mut res: isize = LISTEN;
        unsafe {
            arch::asm!(
                "syscall",
                inout("rax") res,
                in("rdi") sockfd,
                in("rsi") backlog as isize,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        res
    }

    #[inline(always)]
    pub unsafe fn symlinkat(path_to_file: *const u8, fd: isize, path_for_link: *const u8) -> isize {
        let mut res: isize = SYMLINKAT;
        unsafe {
            arch::asm!(
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

    #[inline(always)]
    pub unsafe fn mkdir(path: *const u8, mode: u32) -> isize {
        let mut res: isize = MKDIR;
        unsafe {
            arch::asm!(
                "syscall",
                inout("rax") res,
                in("rdi") path,
                in("rsi") mode,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        // a little magic :?
        //just checks if file alredy exists (so it doesnt fail!)
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
            arch::asm!(
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
}

pub mod stage2 {
    use core::arch;

    const FORK: i32 = 0x39;
    const EXECVE: isize = 0x3b;
    const OPEN: isize = 0x02;
    const CLOSE: isize = 0x03;
    const EXIT: isize = 0x3c;
    const READ: isize = 0x00;
    const SETSID: isize = 0x70;
    const DUP2: isize = 0x21;
    const IOCTL: isize = 0x10;
    const GETDENTS64: isize = 0xd9;
    const OPENAT: isize = 0x101;

    #[inline(always)]
    pub unsafe fn openat(dfd: isize, name: *const u8, flags: usize, mode: usize) -> isize {
        let mut res = OPENAT;

        unsafe {
            arch::asm!(
                "syscall",
                inout("rax") res,
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
    pub unsafe fn getdents64(fd: isize, dirp: *mut u8, count: usize) -> isize {
        let mut res: isize = GETDENTS64;

        unsafe {
            arch::asm!(
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
    pub unsafe fn ioctl(fd: isize, cmd: isize, long_arg: isize) -> isize {
        let mut res: isize = IOCTL;

        unsafe {
            arch::asm!(
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
            arch::asm!(
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
            arch::asm!(
                "syscall",
                inout("rax") _res,
                options(nostack),
                clobber_abi("system")
            )
        };
    }

    #[inline(always)]
    pub unsafe fn read(fd: isize, buffer: &mut [u8]) -> isize {
        let mut res: isize = READ;
        unsafe {
            arch::asm!(
                "syscall",
                inout("rax") res,
                in("rdi") fd,
                in("rsi") buffer.as_mut_ptr(),
                in("rdx") buffer.len(),
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        };
        res
    }

    #[inline(always)]
    pub unsafe fn exit(code: isize) -> ! {
        unsafe {
            arch::asm!(
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
            arch::asm!(
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
            arch::asm!(
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
    pub unsafe fn open(name: *const u8, flags: usize, mode: usize) -> isize {
        let mut res = OPEN;

        unsafe {
            arch::asm!(
                "syscall",
                inout("rax") res,
                in("rdi") name,
                in("rsi") flags,
                in("rdx") mode,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        res
    }

    #[inline(always)]
    pub unsafe fn close(name: isize) -> isize {
        let mut res = CLOSE;

        unsafe {
            arch::asm!(
                "syscall",
                inout("rax") res,
                in("rdi") name,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        }

        res
    }
}

pub mod stage3 {
    use core::arch;

    use crate::asm::prelude::VOID_PTR;

    #[repr(C)]
    pub struct Timespec {
        pub tv_sec: i64,
        pub tv_nsec: i64,
    }

    //Default. ill move it to lib maybe later.
    pub const WAIT_TIME: Timespec = Timespec {
        tv_sec: 0,
        tv_nsec: 500000000,
    };

    const WAIT4: i32 = 0x3d;
    const NANOSLEEP: isize = 0x23;
    const RTSIGPENDING: isize = 0x7f;
    const SIGPROCMASK: isize = 0x0e;
    const RT_SIGTIMEDWAIT: isize = 0x80;

    #[inline(always)]
    pub unsafe fn rt_sigtimedwait(mask: u64) -> isize {
        let mut res: isize = RT_SIGTIMEDWAIT;
        unsafe {
            arch::asm!(
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
            arch::asm!(
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

    #[inline(always)]
    pub unsafe fn rtsigpending() -> u64 {
        let mut _res: isize = RTSIGPENDING;
        let mut pending: u64 = 0;
        unsafe {
            arch::asm!(
                "syscall",
                inout("rax") _res,
                in("rdi") &mut pending as *mut u64,
                in("rsi") 8usize,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        };

        pending
    }

    #[inline(always)]
    pub unsafe fn nanosleep(time: &Timespec) -> isize {
        let mut res: isize = NANOSLEEP;
        unsafe {
            arch::asm!(
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

    #[inline(always)]
    pub unsafe fn wait4(pid: i32, flag: usize) -> i32 {
        let mut res: i32 = 0;
        let mut out_val: i32 = WAIT4;

        unsafe {
            arch::asm!(
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
}

pub mod stage4 {
    use core::arch;

    use crate::asm::stage3::Timespec;

    pub const KILL_PROC: isize = 9;
    pub const SIGTERM: isize = 15;

    const KILL: isize = 0x3E;
    const UMOUNT2: isize = 0xa6;
    const SYNC: isize = 0xa2;

    pub const LAZY_UMOUNT: isize = 0x2;

    pub const GRACE_TIME: Timespec = Timespec {
        tv_sec: 1,
        tv_nsec: 0,
    };

    const TORVALDS_BIRTHDAY: u64 = 672274793;
    const FEELDEAD: u64 = 0xfee1dead;

    pub const POWER_OFF: u64 = 0x4321fedc;
    pub const REBOOT: u64 = 0x01234567;

    #[inline(always)]
    pub unsafe fn sync() -> isize {
        let mut res: isize = SYNC;
        unsafe {
            core::arch::asm!(
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
    pub unsafe fn umount(target: *const u8, flags: isize) -> isize {
        let mut res: isize = UMOUNT2;

        unsafe {
            arch::asm!(
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

    #[inline(always)]
    pub unsafe fn kill_all(sig: isize) {
        let mut _res: isize = KILL;
        unsafe {
            core::arch::asm!(
                "syscall",
                inout("rax") _res,
                in("rdi") -1isize,
                in("rsi") sig,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            )
        };
    }

    #[inline(always)]
    pub unsafe fn poweroff(cmd: u64) -> ! {
        unsafe {
            arch::asm!(
                "syscall",
                in("rax") 169u64,
                in("rdi") FEELDEAD,
                in("rsi") TORVALDS_BIRTHDAY,
                in("rdx") cmd,
                in("r10") 0u64,
                clobber_abi("system")
            );

            loop {
                core::arch::asm!(
                    "syscall",
                    in("rax") 34,
                    lateout("rax") _,
                    clobber_abi("system")
                );
            }
        }
    }
}

pub mod debug {
    use core::arch;
    const WRITE: isize = 0x01;

    #[inline(always)]
    pub unsafe fn write(str: &[u8], to: usize) -> isize {
        let mut res = WRITE;

        unsafe {
            arch::asm!(
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
