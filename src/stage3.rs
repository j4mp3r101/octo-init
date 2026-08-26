use crate::asm::debug::*;
use crate::better_proc_hand::{MAX_TRIES, alter_proc, kill_proc, read_proc, rephrase_entry};

use crate::asm::fs::{close, openat, read};

use crate::asm::procs::wait4;
use crate::asm::signals::{rt_sigtimedwait, sigprocmask};

pub const POWER_OFF: u64 = 0x4321fedc;
pub const REBOOT: u64 = 0x01234567;

use crate::parser::{RAW_BUF_SIZE_READ, full_parse};
use crate::stage1::COMMUNICATION_FILE;

const MASK_POWEROFF: u64 = 1 << (10 - 1);
const MASK_REBOOT: u64 = 1 << (12 - 1);
const MASK_SIGTERM: u64 = 1 << (15 - 1);
const MASK_SIGCHLD: u64 = 1 << (17 - 1);

const MASK_SPAWNPROC: u64 = 1 << (36 - 1);

const BITMASK: u64 = MASK_POWEROFF | MASK_REBOOT | MASK_SIGTERM | MASK_SIGCHLD | MASK_SPAWNPROC;

fn match_task_stuff(reaped_pid: i32, info: (i32, u8)) {
    if info.0 > 0 {
        match info.1 {
            1 => alter_proc(reaped_pid, info.0),
            3 => alter_proc(reaped_pid, info.0),
            _ => kill_proc(reaped_pid),
        }
    } else {
        kill_proc(reaped_pid);
    }
}

pub fn stage3() -> u64 {
    print("Entered stage 3");
    let mut signal: isize;

    unsafe {
        sigprocmask(BITMASK);
    };
    let mut contents = [0u8; RAW_BUF_SIZE_READ];

    loop {
        signal = unsafe { rt_sigtimedwait(BITMASK) };
        if signal > 0 {
            match signal {
                17 => {
                    let mut retries: u8 = MAX_TRIES;

                    'ze: loop {
                        let pid = unsafe { wait4(-1, 1) };

                        if pid > 0 {
                            if retries == 0 {
                                kill_proc(pid);
                            } else {
                                let q = read_proc(pid, &mut contents);

                                if q < 0 {
                                } else {
                                    let info = full_parse(&mut contents);
                                    match_task_stuff(pid, info);
                                }
                                retries -= 1;
                            }
                        } else {
                            break 'ze;
                        }
                    }
                }

                15 => return REBOOT,

                12 => {
                    return REBOOT;
                }

                10 => {
                    return POWER_OFF;
                }

                36 => {
                    let file_fd = unsafe { openat(-100, COMMUNICATION_FILE.as_ptr(), 0, 0) };

                    if file_fd < 0 {
                        continue;
                    }

                    let read = unsafe { read(file_fd, &mut contents) };

                    if read > 0 {
                        let word = [contents[0], contents[1], contents[2], 0];

                        //Now after we got the name its easy.
                        //Ill just handle it the same way stage2 does.

                        let info = full_parse(&mut contents);

                        match info.1 {
                            1 => {
                                rephrase_entry(&word, info.0);
                            }
                            2 => unsafe {
                                wait4(info.0, 0);
                                kill_proc(info.0);
                            },
                            3 => {
                                rephrase_entry(&word, info.0);

                                unsafe {
                                    wait4(info.0, 0);
                                };
                            }
                            _ => {
                                kill_proc(info.0);
                            }
                        };
                    }

                    unsafe { close(file_fd) };
                }

                _ => {}
            }
        } else {
            print("Yo smth wrong twin");
        }
    }
}
