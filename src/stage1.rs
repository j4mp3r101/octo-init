use crate::asm::debug::print;
use crate::asm::prelude::*;
use crate::asm::stage1::*;
use crate::asm::stage2::{close, getdents64, openat};
use crate::parser::RAW_BUF_SIZE_GET;

const GENERAL_DIR: &[u8; 15] = b"/etc/octo-init\0";

const ENTRY_DIR: &[u8; 23] = b"/etc/octo-init/entries\0";
pub const ENABLED_DIR: &[u8; 23] = b"/etc/octo-init/enabled\0";

const PROC: &[u8; 5] = b"proc\0";
const SYSFS: &[u8; 6] = b"sysfs\0";
const TMPFS: &[u8; 6] = b"tmpfs\0";
const DEVTMPFS: &[u8; 9] = b"devtmpfs\0";

pub const SPROC: &[u8; 6] = b"/proc\0";
pub const SSYSFS: &[u8; 5] = b"/sys\0";
pub const STMPFS: &[u8; 5] = b"/tmp\0";
pub const SDEVTMPFS: &[u8; 5] = b"/dev\0";

//after that pseudo-things
const PTS: &[u8; 7] = b"devpts\0";
pub const SPTS: &[u8; 9] = b"/dev/pts\0";

pub const SSHM: &[u8; 9] = b"/dev/shm\0";
pub const SRUN: &[u8; 5] = b"/run\0";

pub fn stage1() -> [u32; 256] {
    print("Octo-init launching...");

    let proc = PROC.as_ptr();
    let sysfs = SYSFS.as_ptr();
    let tmpfs = TMPFS.as_ptr();
    let devtmpfs = DEVTMPFS.as_ptr();

    let pts = PTS.as_ptr();

    unsafe {
        //proc, sysfs, tmpfs, devtmpfs.

        print("Attempting to mount proc...");
        mount(proc, SPROC.as_ptr(), proc, 0usize, VOID_PTR);

        print("Attempting to mount sysfs...");
        mount(sysfs, SSYSFS.as_ptr(), sysfs, 0usize, VOID_PTR);

        print("Attempting to mount devtmpfs...");
        mount(devtmpfs, SDEVTMPFS.as_ptr(), devtmpfs, 0usize, VOID_PTR);

        //DISK
        print("Attempting to mount disk from R to RW");
        mount(VOID_PTR, b"/\0".as_ptr(), b"\0".as_ptr(), 32usize, VOID_PTR);

        print("Attempting create tmpfs (IF IT PRINTS FAILURE IT DOESNT MEAN IT WONT WORK.)");
        mkdir(STMPFS.as_ptr(), 0o755);
        print("Attempting mount tmpfs");
        mount(tmpfs, STMPFS.as_ptr(), tmpfs, 0usize, VOID_PTR);

        //Now we mount pseudo systems (so that it works w apps.)
        print("Mounting run, pts, shm");
        mkdir(SPTS.as_ptr(), 0o755);
        mount(pts, SPTS.as_ptr(), pts, 0usize, VOID_PTR);

        mkdir(SSHM.as_ptr(), 0o755);
        mount(tmpfs, SSHM.as_ptr(), tmpfs, 0, VOID_PTR);

        mount(tmpfs, SRUN.as_ptr(), tmpfs, 0, VOID_PTR);

        print("Creating octo-init directories / checking files.");

        mkdir(GENERAL_DIR.as_ptr(), 0o755);

        mkdir(ENTRY_DIR.as_ptr(), 0o755);

        mkdir(ENABLED_DIR.as_ptr(), 0o755);
    }
    //So this is the amount of total ids which CAN be stored. (in ram so that i dont have to get em later.)
    let mut ids = [0u32; 256];
    let fd = unsafe { openat(-100, ENTRY_DIR.as_ptr(), 0, 0) };
    if fd > 0 {
        let mut buffer = [0u8; RAW_BUF_SIZE_GET];
        //dents are the amount of bytes read now.
        let dents = unsafe { getdents64(fd, buffer.as_mut_ptr(), buffer.len()) };
        let mut i = 0;

        let mut entry_i = 0;

        if dents > 0 {
            let mut le_path = [0u8; 128];
            let mut be_path = [0u8; 128];
            let le_len = ENABLED_DIR.len();
            let be_len = ENTRY_DIR.len();

            for v in 0..le_len {
                if ENABLED_DIR[v] == b'\0' {
                    le_path[v] = b'/';
                } else {
                    le_path[v] = ENABLED_DIR[v]
                }
            }
            for v in 0..be_len {
                if ENTRY_DIR[v] == b'\0' {
                    be_path[v] = b'/';
                } else {
                    be_path[v] = ENTRY_DIR[v];
                }
            }

            loop {
                if i as isize >= dents {
                    break;
                }

                let big_ref = &mut ids;

                let d_reclen = buffer[i + 16] as u16 + ((buffer[i + 17] as u16) << 8);

                let word = &buffer[(i + 19)..(i + (d_reclen as usize))];

                let name = [word[0], word[1], word[2], 0];

                let value = ((word[0] as u32) << 16) | ((word[1] as u32) << 8) | (word[2] as u32);

                //now its time to uh to check da word.
                if !word.starts_with(b".") || !word.starts_with(b"..") {
                    for i in 0..word.len() {
                        be_path[i + be_len] = word[i];
                    }
                    big_ref[entry_i] = value;
                    entry_i += 1;
                }

                for i in 0..3 {
                    le_path[i + le_len] = name[i];
                }

                let z = unsafe { symlinkat(be_path.as_ptr(), -100, le_path.as_ptr()) };

                if z < 0 {
                    print("File found symlink not so much.");
                }

                i += d_reclen as usize;
            }
        } else {
            print("Either its empty or it failed to read it :/")
        }

        let _ = buffer;
    } else {
        print("Failed to open a directory. WE`RE DOOMED!")
    }

    unsafe { close(fd) };

    ids.sort_unstable_by(|a, b| b.cmp(a));

    ids
}
