#![allow(clippy::manual_c_str_literals)]

use crate::asm::debug::print;
use crate::asm::fs::{mount, sync, umount};
use crate::asm::prelude::*;
use crate::asm::procs::kill;
use crate::asm::procs::wait4;
use crate::asm::signals::poweroff;
use crate::asm::time::{Timespec, nanosleep};
use crate::stage1::{SDEVTMPFS, SPROC, SPTS, SRUN, SSHM, SSYSFS, STMPFS};

const LAZY_UMOUNT: isize = 0x02;

const SIGTERM: isize = 15;
const SIGKILL: isize = 9;

const GRACE_TIME: Timespec = Timespec {
    tv_sec: 1,
    tv_nsec: 0,
};

pub fn stage4(int: u64) -> ! {
    print("So we decided to shutdown huh?");
    unsafe {
        print("Sending (merciful) kill signal");
        kill(SIGTERM, -1);

        nanosleep(&GRACE_TIME);
        while wait4(-1, 1) > 0 {}
        print("Sending (forceful) kill signal");
        kill(SIGKILL, -1);
        while wait4(-1, 1) > 0 {}

        print("umount");

        umount(SPTS.as_ptr(), LAZY_UMOUNT);
        umount(SRUN.as_ptr(), LAZY_UMOUNT);
        umount(SSHM.as_ptr(), LAZY_UMOUNT);

        umount(STMPFS.as_ptr(), LAZY_UMOUNT);
        umount(SDEVTMPFS.as_ptr(), LAZY_UMOUNT);
        umount(SSYSFS.as_ptr(), LAZY_UMOUNT);
        umount(SPROC.as_ptr(), LAZY_UMOUNT);

        print("root from RW to R");
        mount(VOID_PTR, b"/\0".as_ptr(), b"\0".as_ptr(), 1usize, VOID_PTR);

        print("sync");

        sync();

        print("Aand shutdown");

        poweroff(int)
    }
}
