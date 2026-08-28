use crate::new_asm::debug::*;

use crate::parser::parsy::parse;
use crate::parser::resp_hand::{match_as_spawner, spawn};

use crate::parser::RAW_BUF_SIZE_READ;

use crate::better_proc_hand::word_read_proc;
use crate::parser::i32::i32_to_null_terminated_bytes;

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
            print("failed to read due to:");
            unsafe { write(&i32_to_null_terminated_bytes(qz as i32), 1) };
            print("");
            continue;
        }

        let entry = parse(&mut contents);

        let info = (spawn(entry), entry.r#type);

        match_as_spawner(info, &word);
    }
}
