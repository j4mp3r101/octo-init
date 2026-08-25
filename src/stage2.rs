use crate::asm::debug::*;
use crate::asm::procs::wait4;

use crate::parser::full_parse;

use crate::parser::RAW_BUF_SIZE_READ;

use crate::better_proc_hand::beta::{kill_proc, rephrase_entry, word_read_proc};

pub fn stage2(ent: [[u8; 3]; 256]) {
    print("Entering stage 2!");

    let mut enc_i = 0;

    let mut contents = [0u8; RAW_BUF_SIZE_READ];

    'q: loop {
        let word = [ent[enc_i][0], ent[enc_i][1], ent[enc_i][2], 0];
        enc_i += 1;
        if word[0] == 0 || word[1] == 0 || word[2] == 0 {
            break 'q;
        }

        let qz = word_read_proc(&word, &mut contents);

        if qz < 0 {
            print("failed to read");
            continue;
        }

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
}
