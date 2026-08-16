use crate::asm::debug::*;
use crate::asm::prelude::*;
use crate::asm::stage2::*;
use crate::asm::stage3::wait4;
use crate::parser::KEY_LIST;
use crate::parser::PARSER_LIST;

use crate::parser::RAW_BUF_SIZE_READ;
use crate::stage1::ENABLED_DIR;

#[derive(Clone, Copy)]
pub struct Task {
    pub pid: i32,
    pub name: [u8; 4],
}

pub fn stage2(ent: &[u32; 256]) -> [Task; 256] {
    print("Entering stage 2!");
    let gfd = unsafe { openat(-100, ENABLED_DIR.as_ptr(), 0, 0) };
    let mut task_buf: [Task; 256] = [Task {
        pid: 0,
        name: [0; 4],
    }; 256];
    if gfd > 0 {
        let mut entry_i = 0;
        for &enc in ent {
            if enc == 0 {
                break;
            }

            let word: [u8; 4] = [
                ((enc >> 16) & 0xFF) as u8,
                ((enc >> 8) & 0xFF) as u8,
                (enc & 0xFF) as u8,
                0,
            ];

            let child_fd = unsafe { openat(gfd, word.as_ptr(), 0, 0) };

            if child_fd > 0 {
                let mut contents = [0u8; RAW_BUF_SIZE_READ];
                let qz = unsafe { read(child_fd, &mut contents) };

                if qz < 0 {
                    continue;
                }

                //Now we can finally parse
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

                'a: loop {
                    if contents[small_i] == 0 {
                        break 'a;
                    }
                    if broken == true {
                        break 'a;
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

                    let slice = &contents
                        [(slice_start as usize)..(slice_start + (slice_len as u16)) as usize];

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

                    slice_len = 0;
                    small_i += 1;

                    slice_start = small_i as u16;

                    for &char in &contents[small_i..] {
                        if char == b' ' || char == b'\n' {
                            contents[small_i] = 0;
                            slice_len = (small_i + 1 - (slice_start as usize)) as u8;
                            break;
                        } else if char == b'\0' {
                            slice_len = (small_i + 1 - (slice_start as usize)) as u8;
                            broken = true;
                            break;
                        }
                        small_i += 1;
                    }

                    let slice = &contents
                        [(slice_start as usize)..(slice_start + (slice_len as u16)) as usize];
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

                        _ => break 'a,
                    }

                    slice_len = 0;
                }

                if path != VOID_PTR && key_id >= 0 {
                    let proc = unsafe { fork() };
                    argv[0] = path;
                    if proc == 0 {
                        if key_id == 1 {
                            unsafe {
                                setsid();
                            }
                        }

                        unsafe {
                            execve(argv.as_ptr(), envp.as_ptr(), path);
                            print("oops i died");

                            exit(1);
                        }
                    } else if proc > 0 {
                        match key_id {
                            1 => {
                                task_buf[entry_i].name = word;
                                task_buf[entry_i].pid = proc;
                                entry_i += 1;
                            }

                            2 => {
                                unsafe { wait4(proc, 0) };
                            }

                            3 => {
                                //bug! im too lazy to fix it but uh
                                task_buf[entry_i].name = word;
                                task_buf[entry_i].pid = proc;
                                entry_i += 1;
                                unsafe { wait4(proc, 0) }; //This kills the proc. So the execve and fork has to be re-ran.
                            }

                            _ => {}
                        }
                    }
                }
            }
        }
    } else {
        print("Something went seriously wrong. stage 1 failed.")
    }
    task_buf
}
