use crate::asm::debug::*;
use crate::asm::stage2::{execve, exit, fork};
use crate::asm::stage3::*;
use crate::asm::stage4::{POWER_OFF, REBOOT};
use crate::stage2::{Task, ENTRIES};

const MASK_POWEROFF: u64 = 1 << (10 - 1);
const MASK_REBOOT: u64 = 1 << (12 - 1);
const MASK_SIGTERM: u64 = 1 << (15 - 1);

const BITMASK: u64 = MASK_POWEROFF | MASK_REBOOT | MASK_SIGTERM;

pub fn stage3(entries: &mut [Task; ENTRIES]) -> u64 {
    print("Entered stage 3");
    let mut signal: u64;
    //This part should not stop unless forced (shutdown)
    unsafe {
        sigprocmask(BITMASK);
    };
    'a: loop {
        let pid = unsafe { wait4(-1, 1) };

        if pid > 0 {
            print("TTY DOWN.");

            'b: for v in &mut *entries {
                if v.pid == pid {
                    v.post_reap.run();
                    let fork = unsafe { fork() };
                    if fork == 0 {
                        
                        v.pre_reap.run();

                        let str_as_ptr = v.path.as_ptr();
                        let argv: [*const u8; 2] = [str_as_ptr, core::ptr::null()];
                        let envp: [*const u8; 1] = [core::ptr::null()];

                        unsafe {
                            execve(argv.as_ptr(), envp.as_ptr(), str_as_ptr);

                            write(&v.path, 1);
                            print(" FAILED");

                            exit(1);
                        }
                    } else if fork > 0 {
                        v.pid = fork;
                    }
                    break 'b;
                }
            }
        }

        signal = unsafe { rtsigpending() };

        if (signal & BITMASK) != 0 {
            break 'a;
        }

        unsafe {
            nanosleep(&WAIT_TIME);
        }
    }

    if (signal & MASK_REBOOT) != 0 {
        REBOOT
    } else {
        POWER_OFF
    }
}