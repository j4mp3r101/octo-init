use crate::asm::debug::*;
use crate::asm::prelude::*;
use crate::asm::stage2::*;
use crate::asm::stage3::wait4;
use crate::parser::{Action, PARSER_LIST};

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct Task {
    pub pid: i32,
    pub path: [u8; PATH_SIZE],

    pub special: u8,

    pub pre_reap: Action,
    pub post_reap: Action,
}

//some configs
const CONFIG_FILE_PATH: &[u8] = b"/etc/octo-init.conf\0";

const PATH_SIZE: usize = 28;
const ARG_SIZE: usize = 8;
pub const ENTRIES: usize = 16;
const RAW_BUF_SIZE: usize = PATH_SIZE * ENTRIES;

const NLINE_SIGN: u8 = b'\n';

pub fn stage2() -> [Task; 16] {
    let mut entries: [Task; 16] = [Task {
        pid: 0,
        path: [0u8; PATH_SIZE],
        special: 0,
        post_reap: Action::None,
        pre_reap: Action::None,
    }; ENTRIES];

    print("Entering stage 2");

    let fd = unsafe { open(CONFIG_FILE_PATH.as_ptr(), 0usize, 0usize) };
    if fd > 0 {
        let mut buffer = [0u8; RAW_BUF_SIZE];

        let res = unsafe { read(fd, &mut buffer) };
        if res > 0 {
            let mut path = [0u8; PATH_SIZE];
            let mut keyword = [0u8; ARG_SIZE];

            let mut local_i = 0;
            let mut entry_i = 0;
            let mut general_i = 0;

            let mut special = 0;
            let mut pre_spawn = Action::None;
            let mut post_spawn = Action::None;

            'z: loop {
                let cur_entry = &mut entries[entry_i];
                //Finding the keyword.
                for &v in &buffer[general_i..] {
                    match v {
                        0 => break 'z,
                        b' ' => break,
                        _ => {
                            keyword[local_i] = v;
                            local_i += 1
                        }
                    }
                    general_i += 1;
                }
                general_i += 1;
                local_i = 0;

                for &v in &buffer[general_i..] {
                    match v {
                        NLINE_SIGN => {
                            break;
                        }
                        0 => break 'z,
                        _ => {
                            path[local_i] = v;
                            local_i += 1;
                        }
                    }

                    general_i += 1;
                }
                path[local_i] = 0;
                cur_entry.path = path;
                local_i = 0;
                general_i += 1;

                for v in PARSER_LIST {
                    if keyword == v.keyword {
                        special = v.special;
                        cur_entry.special = v.special;

                        pre_spawn = v.pre_spawn_action;
                        post_spawn = v.post_spawn_action;

                        cur_entry.post_reap = v.post_reap_action;
                        cur_entry.pre_reap = v.pre_reap_action;

                        break;
                    }
                }
                keyword = [0u8; 8];
                //Now we have every piece of data needed to fork!

                let fork = unsafe { fork() };
                if fork == 0 {
                    pre_spawn.run();

                    let name_as_ptr = path.as_ptr();

                    let argv = [name_as_ptr, VOID_PTR];
                    let envp = [VOID_PTR];

                    unsafe {
                        let z = execve(argv.as_ptr(), envp.as_ptr(), name_as_ptr);

                        write(&path, 1);
                        print(" FAILED WITH ERROR CODE:");

                        match z {
                            -2 => print("-2"),
                            -8 => print("-8"),
                            -12 => print("-12"),
                            -13 => print("-13"),
                            -14 => print("-14"),
                            _ => print("Some other error"),
                        };

                        exit(1);
                    }
                } else if fork > 0 {
                    match special {
                        1 => {
                            cur_entry.pid = fork;
                            entry_i += 1;
                        }
                        2 => unsafe {
                            wait4(fork, 0);
                        },
                        3 => {
                            cur_entry.pid = fork;
                            entry_i += 1;

                            unsafe {
                                wait4(fork, 0);
                            }
                        }
                        _ => {}
                    }

                    post_spawn.run();
                }
            }
        }

        unsafe { close(fd) };
    }

    entries
}
