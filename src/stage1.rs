use crate::asm::debug::print;
use crate::asm::prelude::*;
use crate::asm::stage1::*;

const PROC: &[u8; 5] = b"proc\0";
const SYSFS: &[u8; 6] = b"sysfs\0";
const TMPFS: &[u8; 6] = b"tmpfs\0";
const DEVTMPFS: &[u8; 9] = b"devtmpfs\0";

pub const SPROC: &[u8; 6] = b"/proc\0";
pub const SSYSFS: &[u8; 5] = b"/sys\0";
pub const STMPFS: &[u8; 5] = b"/tmp\0";
pub const SDEVTMPFS: &[u8; 5] = b"/dev\0";

#[inline(always)]
pub fn result(a: isize) {
    if a > -1 {
        print("Success")
    } else {
        print("Failure")
    }
}

//after that pseudo-things
const PTS: &[u8; 7] = b"devpts\0";
pub const SPTS: &[u8; 9] = b"/dev/pts\0";

pub const SSHM: &[u8; 9] = b"/dev/shm\0";
pub const SRUN: &[u8; 5] = b"/run\0";

pub fn stage1() {
    print("Octo-init launching...");

    let proc = PROC.as_ptr();
    let sysfs = SYSFS.as_ptr();
    let tmpfs = TMPFS.as_ptr();
    let devtmpfs = DEVTMPFS.as_ptr();

    let pts = PTS.as_ptr();

    unsafe {
        //proc, sysfs, tmpfs, devtmpfs.

        print("Attempting to mount proc...");
        result(mount(proc, SPROC.as_ptr(), proc, 0usize, VOID_PTR));

        print("Attempting to mount sysfs...");
        result(mount(sysfs, SSYSFS.as_ptr(), sysfs, 0usize, VOID_PTR));

        print("Attempting to mount devtmpfs...");
        result(mount(
            devtmpfs,
            SDEVTMPFS.as_ptr(),
            devtmpfs,
            0usize,
            VOID_PTR,
        ));

        //DISK
        print("Attempting to mount disk from R to RW");
        result(mount(
            VOID_PTR,
            b"/\0".as_ptr(),
            b"\0".as_ptr(),
            32usize,
            VOID_PTR,
        ));

        print("Attempting create tmpfs (IF IT PRINTS FAILURE IT DOESNT MEAN IT WONT WORK.)");
        result(mkdir(STMPFS.as_ptr(), 0o755));
        print("Attempting mount tmpfs");
        result(mount(tmpfs, STMPFS.as_ptr(), tmpfs, 0usize, VOID_PTR));

        //Now we mount pseudo systems (so that it works w apps.)
        print("Attempting to create pts (IF IT PRINTS FAILURE IT DOESNT MEAN IT WONT WORK.)");
        result(mkdir(SPTS.as_ptr(), 0o755));
        print("Attempting mount pts");
        result(mount(pts, SPTS.as_ptr(), pts, 0usize, VOID_PTR));

        print("Attempting to create shm (IF IT PRINTS FAILURE IT DOESNT MEAN IT WONT WORK.)");
        result(mkdir(SSHM.as_ptr(), 0o755));
        print("Attempting mount shm");
        result(mount(tmpfs, SSHM.as_ptr(), tmpfs, 0, VOID_PTR));

        print("Attempting to mount run");
        result(mount(tmpfs, SRUN.as_ptr(), tmpfs, 0, VOID_PTR));
    }
}
