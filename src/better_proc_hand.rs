//This approach basically uses a buffer kind of thing.

pub const MAX_TRIES: u8 = 3;

use crate::{
    asm::{
        debug::{print, write},
        fs::{close, openat, read},
    },
    parser::i32_to_null_terminated_bytes,
};

//So ill make a new "mod" so that its all good.

pub mod beta {
    use crate::asm::fs::{close, openat, read, renameat};
    use crate::asm::symlinks::{symlinkat, unlinkat};

    use crate::parser::i32_to_null_terminated_bytes;

    use crate::stage1::{ENABLED_DIR, ENTRY_DIR};

    //Now i need to "morph"

    pub fn create_entry(word: &[u8; 4]) {
        let mut join_buffer = [0u8; 30];

        for i in 0..25 {
            join_buffer[i] = ENTRY_DIR[i]
        }
        join_buffer[25] = b'/';

        join_buffer[26] = word[1];
        join_buffer[27] = word[2];
        join_buffer[28] = word[3];

        let fd = unsafe { openat(-100, ENABLED_DIR.as_ptr(), 0, 0) };

        unsafe { symlinkat(join_buffer.as_ptr(), fd, word.as_ptr()) };

        unsafe { close(fd) };
    }

    pub fn rephrase_entry(word: &[u8; 4], pid: i32) {
        let fd = unsafe { openat(-100, ENABLED_DIR.as_ptr(), 0, 0) };
        let value = i32_to_null_terminated_bytes(pid);
        unsafe {
            renameat(fd as i32, word.as_ptr(), fd as i32, value.as_ptr());
            close(fd);
        }
    }

    pub fn alter_proc(old_pid: i32, new_pid: i32) {
        let fd = unsafe { openat(-100, ENABLED_DIR.as_ptr(), 0, 0) };

        let value1 = i32_to_null_terminated_bytes(old_pid);
        let value2 = i32_to_null_terminated_bytes(new_pid);

        unsafe {
            renameat(fd as i32, value1.as_ptr(), fd as i32, value2.as_ptr());
            close(fd)
        };
    }

    pub fn read_proc(pid: i32, buf: &mut [u8]) -> isize {
        let fd = unsafe { openat(-100, ENABLED_DIR.as_ptr(), 0, 0) };

        let value = i32_to_null_terminated_bytes(pid);

        let cfd = unsafe { openat(fd as i32, value.as_ptr(), 0, 0) };

        let v = unsafe { read(cfd, buf) };

        unsafe {
            close(cfd);
            close(fd);
        }

        v
    }

    pub fn word_read_proc(word: &[u8], buf: &mut [u8]) -> isize {
        let fd = unsafe { openat(-100, ENABLED_DIR.as_ptr(), 0, 0) };

        let cfd = unsafe { openat(fd as i32, word.as_ptr(), 0, 0) };

        let v = unsafe { read(cfd, buf) };

        unsafe {
            close(cfd);
            close(fd);
        }

        v
    }

    pub fn kill_proc(pid: i32) {
        let fd = unsafe { openat(-100, ENABLED_DIR.as_ptr(), 0, 0) };
        let value = i32_to_null_terminated_bytes(pid);

        unsafe {
            unlinkat(fd as i32, value.as_ptr(), 0);
            close(fd)
        };
    }
}

const ENABLED_DIR: &[u8] = b"/run/octo-init/enabled/\0";

//Basically symlinks but better
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct LilTask {
    pub pid: i32,
    pub link: [u8; 4],
    pub tries: u8,
}

impl LilTask {
    #[inline(always)]
    pub fn establish(pid: i32, name: &[u8]) -> Self {
        LilTask {
            pid,
            link: [name[0], name[1], name[2], 0],
            tries: 0,
        }
    }
    #[inline(always)]
    pub fn get_info(&self, buffer: &mut [u8]) -> isize {
        let gfd = unsafe { openat(-100, ENABLED_DIR.as_ptr(), 0, 0) };

        let val = unsafe { openat(gfd as i32, self.link.as_ptr(), 0, 0) };

        unsafe { close(gfd) };

        let res = unsafe { read(val, buffer) };

        unsafe { close(val) };

        res
    }
}

#[inline(always)]
pub fn search_through(v: &[LilTask], pid: i32) -> Option<usize> {
    let mut i = 0;

    print("Matching ");
    unsafe { write(&i32_to_null_terminated_bytes(pid), 1) };
    print(":");

    'a: loop {
        let taskie = &v[i];

        if taskie.pid == pid {
            unsafe { write(&i32_to_null_terminated_bytes(taskie.pid), 1) };
            print(" +");
            return Some(i);
        } else if taskie.pid == 0 {
            break 'a;
        }
        unsafe { write(&i32_to_null_terminated_bytes(taskie.pid), 1) };
        print(" -");
        i += 1
    }

    None
}
