//This approach basically uses a buffer kind of thing.

pub const MAX_TRIES: u8 = 64;

//So ill make a new "mod" so that its all good.

use crate::new_asm::fs::{close, openat, read, renameat};
use crate::new_asm::symlinks::{symlinkat, unlinkat};

use crate::parser::i32::i32_to_null_terminated_bytes;

use crate::stage1::{ENABLED_DIR, ENTRY_DIR};

use crate::new_asm::debug::{write, *};

//Now i need to "morph"

pub fn create_entry(word: &[u8]) -> [u8; 3] {
    let mut join_buffer = [0u8; 60];
    join_buffer[..23].copy_from_slice(&ENTRY_DIR[..23]);

    join_buffer[22] = b'/';
    let len = word.len();

    join_buffer[23..(len + 23)].copy_from_slice(&word[..len]);

    let fd = unsafe { openat(-100, ENABLED_DIR.as_ptr(), 0, 0) };

    let name = [word[0], word[1], word[2], 0];

    let q = unsafe { symlinkat(join_buffer.as_ptr(), fd, name.as_ptr()) };

    if q < 0 {
        print("symlink not symlinking");
        unsafe { write(&i32_to_null_terminated_bytes(q as i32), 1) };
        print("");
    };

    unsafe { close(fd) };

    [name[0], name[1], name[2]]
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
    };

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
