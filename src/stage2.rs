use crate::asm::debug::*;
use crate::asm::stage3::wait4;

use crate::parser::full_parse;

use crate::parser::RAW_BUF_SIZE_READ;

use crate::better_proc_hand::LilTask;

#[derive(Clone, Copy)]
pub struct Task {
    pub pid: i32,
    pub name: [u8; 4],
}

pub fn stage2(ent: [u32; 256]) -> [LilTask; 256] {
    print("Entering stage 2!");

    let mut tasks = [LilTask::establish(0, b"\0\0\0\0"); 256];

    let mut curtask = 0;

    print("gfd opened");
    for enc in ent {
        if enc == 0 {
            break;
        }
        let word: [u8; 4] = [
            ((enc >> 16) & 0xFF) as u8,
            ((enc >> 8) & 0xFF) as u8,
            (enc & 0xFF) as u8,
            0,
        ];

        tasks[curtask].link = word;

        let mut contents = [0u8; RAW_BUF_SIZE_READ];

        let qz = tasks[curtask].get_info(&mut contents);

        if qz < 0 {
            print("failed to read");
            continue;
        }

        let info = full_parse(&mut contents);

        match info.1 {
            1 => {
                tasks[curtask].pid = info.0;
                curtask += 1;
            }
            2 => unsafe {
                wait4(info.0, 0);
            },
            3 => {
                tasks[curtask].pid = info.0;
                curtask += 1;

                unsafe {
                    wait4(info.0, 0);
                };
            }
            _ => {}
        };
    }

    tasks
}
