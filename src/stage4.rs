use crate::asm::debug::print;
use crate::asm::prelude::*;
use crate::asm::stage1::mount;
use crate::asm::stage3::nanosleep;
use crate::asm::stage3::wait4;
use crate::asm::stage4::*;
use crate::stage1::{SDEVTMPFS, SPROC, SPTS, SRUN, SSHM, SSYSFS, STMPFS};

pub fn stage4(int: u64) -> ! {
    print("So we decided to shutdown huh?");
    unsafe {
        print("Sending (merciful) kill signal");
        kill_all(SIGTERM);

        nanosleep(&GRACE_TIME);
        while wait4(-1, 1) > 0 {}
        print("Sending (forceful) kill signal");
        kill_all(KILL_PROC);
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

        sys_poweroff(int);
    }
}
