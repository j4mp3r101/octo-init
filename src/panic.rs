use crate::new_asm::debug::print;
use crate::new_asm::prelude::VOID_PTR;
use crate::new_asm::procs::wait4;
use crate::new_asm::signals::{rt_sigtimedwait, sigprocmask};
use crate::parser::parsy::ParsingResult;
use crate::parser::resp_hand::spawn;
use crate::parser::types::SpecialType;
use crate::stage3::{POWER_OFF, REBOOT};
use crate::stage4::stage4;

use crate::stage3::BITMASK;

//I mean i could do an emergency shell.

#[inline(always)]
pub fn panic() -> ! {
    print("Launching emergency shell");

    let name = b"/bin/sh\0";

    let mut args = [VOID_PTR; 64];

    args[0] = name.as_ptr();

    let child_process = ParsingResult {
        args,
        envs: [VOID_PTR; 64],
        r#type: SpecialType::Daemon,
        path: name.as_ptr(),
    };

    let mut track_pid = spawn(child_process);

    //So basically the thing above was stage 2, now stage 3.

    print("Entered stage 3");
    let mut signal: isize;

    unsafe {
        sigprocmask(BITMASK);
    };

    loop {
        signal = unsafe { rt_sigtimedwait(BITMASK) };
        if signal > 0 {
            match signal {
                17 => 'ze: loop {
                    let pid = unsafe { wait4(-1, 1) };

                    if pid < 0 {
                        break 'ze;
                    }

                    if pid == track_pid {
                        track_pid = spawn(child_process);
                    }
                },

                10 => stage4(POWER_OFF),

                15 => stage4(REBOOT),
                12 => stage4(REBOOT),

                _ => {}
            }
        }
    }
}
