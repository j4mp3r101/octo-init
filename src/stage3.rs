use crate::asm::debug::*;
use crate::asm::prelude::VOID_PTR;
use crate::asm::stage2::{close, execve, exit, fork, openat, read, setsid};
use crate::asm::stage3::*;
use crate::asm::stage4::{POWER_OFF, REBOOT};
use crate::parser::{KEY_LIST, PARSER_LIST, RAW_BUF_SIZE_READ};
use crate::stage1::ENABLED_DIR;
use crate::stage2::Task;

const MASK_POWEROFF: u64 = 1 << (10 - 1);
const MASK_REBOOT: u64 = 1 << (12 - 1);
const MASK_SIGTERM: u64 = 1 << (15 - 1);
const MASK_SIGCHILD: u64 = 1 << (17 - 1);

const BITMASK: u64 = MASK_POWEROFF | MASK_REBOOT | MASK_SIGTERM | MASK_SIGCHILD;

pub fn stage3(entries: &mut [Task; 256]) -> u64 {
    print("Entered stage 3");
    let mut signal: isize;

    let mut acc_len = entries.len();
    for i in 0..acc_len {
        if entries[i].pid == 0 {
            acc_len = i;
            break;
        }
    }

    unsafe {
        sigprocmask(BITMASK);
    };
    let gfd = unsafe { openat(-100, ENABLED_DIR.as_ptr(), 0, 0) };

    let mut contents = [0u8; RAW_BUF_SIZE_READ];

    'a: loop {
        signal = unsafe { rt_sigtimedwait(BITMASK) };
        if signal > 0 {
            match signal {
                17 => {
                    let pid = unsafe { wait4(-1, 1) };
                    if pid > 0 {
                        for v in 0..acc_len {
                            if entries[v].pid == pid {
                                let fd = unsafe { openat(gfd, entries[v].name.as_ptr(), 0, 0) };

                                let z = unsafe { read(fd, &mut contents) };

                                if z < 0 {
                                    print("Failed to read, removing from entries.");
                                    entries[v].pid = 0;
                                    unsafe { close(fd) };
                                    continue 'a;
                                }

                                let mut key_id: i8 = -1;
                                let mut path = VOID_PTR;

                                let mut argi = 1;
                                let mut envi = 0;

                                let mut argv = [VOID_PTR; 64];
                                let mut envp = [VOID_PTR; 64];

                                let mut slice_start: u16;
                                let mut slice_len: u8 = 0;

                                let mut small_i = 0;

                                let mut broken = false;

                                'qze: while small_i < contents.len() {
                                    if contents[small_i] == 0 {
                                        break 'qze;
                                    }
                                    if broken == true {
                                        break 'qze;
                                    }
                                    slice_start = small_i as u16;
                                    for &char in &contents[small_i..] {
                                        if char == b' ' || char == b'\n' {
                                            slice_len = (small_i - (slice_start as usize)) as u8;
                                            break;
                                        }
                                        small_i += 1;
                                    }
                                    //now we find our target.

                                    let slice = &contents[(slice_start as usize)
                                        ..(slice_start + (slice_len as u16)) as usize];

                                    let mut core_type: i8 = -1;

                                    'q: for vi in 0..KEY_LIST.len() {
                                        let v = KEY_LIST[vi];
                                        if slice.len() == v.len() {
                                            if slice == v {
                                                core_type = vi as i8;
                                                break 'q;
                                            }
                                            //a little more weird but its aight.
                                        }
                                    }

                                    let _ = slice;

                                    //now this shit will be done each fucking time and i cant stand it...
                                    //kidding we have the core type we reset and get the value!
                                    slice_len = 0;
                                    small_i += 1;

                                    slice_start = small_i as u16;

                                    for &char in &contents[small_i..] {
                                        if char == b' ' || char == b'\n' {
                                            contents[small_i] = 0;
                                            slice_len =
                                                (small_i + 1 - (slice_start as usize)) as u8;
                                            break;
                                        } else if char == b'\0' {
                                            slice_len =
                                                (small_i + 1 - (slice_start as usize)) as u8;
                                            broken = true;
                                            break;
                                        }
                                        small_i += 1;
                                    }

                                    let slice = &contents[(slice_start as usize)
                                        ..(slice_start + (slice_len as u16)) as usize];
                                    small_i += 1;
                                    //Okay so now we have everything we need time to match the action.

                                    match core_type {
                                        0 => {
                                            envp[envi] = contents[(slice_start as usize)
                                                ..(slice_start + (slice_len as u16)) as usize]
                                                .as_ptr();

                                            envi += 1;
                                        }

                                        1 => {
                                            argv[argi] = slice.as_ptr();

                                            argi += 1;
                                        }

                                        2 => {
                                            path = slice.as_ptr();
                                        }

                                        3 => {
                                            'q: for value in 0..PARSER_LIST.len() {
                                                let v = &PARSER_LIST[value];
                                                if v.keyword == slice {
                                                    key_id = v.special as i8;
                                                    break 'q;
                                                }
                                            }
                                        }

                                        _ => break 'qze,
                                    }

                                    slice_len = 0;
                                }

                                if path != VOID_PTR && key_id >= 0 {
                                    let proc = unsafe { fork() };
                                    argv[0] = path;
                                    if proc == 0 {
                                        if key_id == 1 || key_id == 3 {
                                            unsafe { setsid() };
                                        }

                                        unsafe {
                                            execve(argv.as_ptr(), envp.as_ptr(), path);
                                            print("oops i died");

                                            exit(1);
                                        }
                                    } else if proc > 0 {
                                        match key_id {
                                            1 => {
                                                entries[v].pid = proc;
                                            }

                                            2 => {
                                                unsafe { wait4(proc, 0) };
                                            }

                                            3 => {
                                                entries[v].pid = proc;
                                                unsafe { wait4(proc, 0) };
                                            }

                                            _ => {}
                                        }
                                    }
                                }

                                unsafe { close(fd) };
                            }
                        }
                    }
                }

                15 => {
                    unsafe { close(gfd) };
                    return REBOOT;
                }

                12 => {
                    unsafe { close(gfd) };
                    return POWER_OFF;
                }

                10 => {
                    unsafe { close(gfd) };
                    return POWER_OFF;
                }

                _ => {}
            }
        } else {
            print("Yo smth wrong twin");
        }
    }
}
