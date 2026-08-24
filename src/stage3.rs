use crate::asm::debug::*;
use crate::asm::stage2::{close, openat, read};
use crate::asm::stage3::*;
use crate::asm::stage4::{POWER_OFF, REBOOT};
use crate::better_proc_hand::{LilTask, MAX_TRIES, search_through};
use crate::stage1::COMMUNICATION_FILE;

use crate::parser::{RAW_BUF_SIZE_READ, full_parse};

const MASK_POWEROFF: u64 = 1 << (10 - 1);
const MASK_REBOOT: u64 = 1 << (12 - 1);
const MASK_SIGTERM: u64 = 1 << (15 - 1);
const MASK_SIGCHLD: u64 = 1 << (17 - 1);

//Custom signals

//const MASK_SPAWNPROC: u64 = 1 << (36 - 1);

const BITMASK: u64 = MASK_POWEROFF | MASK_REBOOT | MASK_SIGTERM | MASK_SIGCHLD;

fn match_task_stuff(list: &mut [LilTask], pos: usize, info: (i32, u8)) {
    if info.0 > 0 {
        match info.1 {
            1 => list[pos].pid = info.0,
            3 => list[pos].pid = info.0,
            _ => list[pos].pid = 0,
        }
    }
}

pub fn stage3(mut tasks: [LilTask; 256]) -> u64 {
    print("Entered stage 3");
    let mut signal: isize;

    unsafe {
        sigprocmask(BITMASK);
    };
    let mut contents = [0u8; RAW_BUF_SIZE_READ];

    'a: loop {
        signal = unsafe { rt_sigtimedwait(BITMASK) };
        if signal > 0 {
            match signal {
                17 => {
                    print("So some procs died!");

                    'ze: loop {
                        let pid = unsafe { wait4(-1, 1) };

                        if pid > 0 {
                            let pos = search_through(&tasks, pid);
                            match pos {
                                Some(position) => {
                                    tasks[position].get_info(&mut contents);
                                    let info = full_parse(&mut contents);

                                    if info.0 < 0 {
                                        tasks[position].tries += 1;

                                        if tasks[position].tries > MAX_TRIES {
                                            tasks[position].pid = 0;
                                        }
                                    } else {
                                        tasks[position].tries = 0;
                                    }

                                    match_task_stuff(&mut tasks, position, info);
                                }
                                None => {
                                    continue 'a;
                                }
                            }
                        } else {
                            break 'ze;
                        }
                    }
                }

                45 => {
                    let fd = unsafe { openat(-100, COMMUNICATION_FILE.as_ptr(), 0, 0) };

                    unsafe { read(fd, &mut contents) };

                    let name = [contents[0], contents[1], contents[2], 0];

                    //so now i have to find a free spot

                    let free_spot = search_through(&tasks, 0);

                    match free_spot {
                        Some(value) => {
                            tasks[value].link = name;
                            tasks[value].get_info(&mut contents);

                            let info = full_parse(&mut contents);

                            match_task_stuff(&mut tasks, value, info);
                        }
                        None => {}
                    };

                    unsafe { close(fd) };
                }

                12 => {
                    return REBOOT;
                }

                10 => {
                    return POWER_OFF;
                }

                _ => {}
            }
        } else {
            print("Yo smth wrong twin");
        }
    }
}
