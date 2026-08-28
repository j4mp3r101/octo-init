use super::super::new_asm::debug::*;
use super::super::new_asm::procs::{execve, exit, fork, setsid, wait4};
use super::i32::i32_to_null_terminated_bytes;
use super::parsy::ParsingResult;
use super::types::SpecialType;

use crate::better_proc_hand::{alter_proc, kill_proc, rephrase_entry};

#[inline(always)]
pub fn spawn(info: ParsingResult) -> i32 {
    let pid = unsafe { fork() };
    if pid == 0 {
        match info.r#type {
            SpecialType::NoType => return 0,
            SpecialType::Daemon | SpecialType::Oneshot => unsafe {
                setsid();
            },
            _ => {}
        }
        unsafe {
            let q = execve(info.args.as_ptr(), info.envs.as_ptr(), info.path);

            write(&i32_to_null_terminated_bytes(q as i32), 1);
            print(" is the exit code of proc.");

            exit(1);
        }
    };

    if pid < 0 {
        print("Oops");
    }

    pid
}

pub fn match_as_spawner(info: (i32, SpecialType), word: &[u8; 4]) {
    match info.1 {
        SpecialType::Daemon => {
            rephrase_entry(word, info.0);
        }
        SpecialType::Waitfor => unsafe {
            wait4(info.0, 0);
            kill_proc(info.0);
        },
        SpecialType::Waitford => {
            rephrase_entry(word, info.0);

            unsafe {
                wait4(info.0, 0);
            };
        }
        SpecialType::Oneshot | SpecialType::NoType => {
            kill_proc(info.0);
        }
    };
}

pub fn match_as_reaper(reaped_pid: i32, info: (i32, SpecialType)) {
    if info.0 > 0 {
        match info.1 {
            SpecialType::Daemon | SpecialType::Waitford => alter_proc(reaped_pid, info.0),
            _ => kill_proc(reaped_pid),
        }
    } else {
        kill_proc(reaped_pid);
    }
}
