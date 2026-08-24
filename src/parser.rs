use crate::asm::debug::{print, write};
//parser.rs was made to easily add entries to the parses w/o changing stage2.
use crate::asm::prelude::VOID_PTR;

use crate::asm::stage2::{execve, exit, fork, setsid};

pub const RAW_BUF_SIZE_GET: usize = 2048;
pub const RAW_BUF_SIZE_READ: usize = 2048;

pub fn pid_to_word(pid: u32) -> [u8; 12] {
    let mut out = [0u8; 12];

    if pid == 0 {
        out[0] = b'0';
        return out;
    }

    let mut temp = pid;
    let mut len = 0;
    while temp > 0 {
        temp /= 10;
        len += 1;
    }

    let mut temp = pid;
    for i in (0..len).rev() {
        out[i] = (temp % 10) as u8 + b'0';
        temp /= 10;
    }

    out
}

//example
/*ParserEntry {
    keyword: b"ONESHOT\0",

    special: 0,
}; */
//Each entry needs to be in the form of this struct
#[repr(C)]
pub struct ParserEntry {
    pub keyword: &'static [u8],

    //Special is unique so ill give a bit of documentation.
    //special = 0 means the entry ISNT SAVED and ISNT WAITED FOR.
    //special = 1 means the entry IS SAVED but ISNT WAITED FOR.
    //special = 2 means the entry ISNT SAVED but IS WAITED FOR.
    //special = 3 means the entry IS SAVED and IS WAITER FOR.
    pub special: u8,
}

pub const PARSER_LIST: [ParserEntry; 4] = [
    ParserEntry {
        keyword: b"ONESHOT",
        special: 0,
    },
    ParserEntry {
        keyword: b"DAEMON",

        special: 1,
    },
    ParserEntry {
        keyword: b"WAITFOR",

        special: 2,
    },
    ParserEntry {
        keyword: b"WAITFORD",

        special: 3,
    },
];

//some settings

const MAX_ARGS: usize = 64;
const MAX_ENVS: usize = 64;

#[inline(always)]
pub fn pos_to_new_or_null<'a>(pos: usize, into: &'a mut [u8]) -> (&'a [u8], usize, bool) {
    let mut mvd = pos;
    let mut null = false;

    'a: loop {
        match into[mvd] {
            b'\0' => {
                null = true;
                break 'a;
            }

            b'\n' => {
                into[mvd] = 0;
                break 'a;
            }

            _ => mvd += 1,
        }
    }

    let length = mvd - pos;

    (&into[pos..mvd], length, null)
}
#[inline(always)]
pub fn get_entry_from_file(buffer: &mut [u8]) -> (u8, *const u8, [*const u8; 64], [*const u8; 64]) {
    print("Parsing attempt...");
    let mut args = [VOID_PTR; MAX_ARGS];
    let mut envs = [VOID_PTR; MAX_ENVS];

    let mut argi = 1;
    let mut envi = 0;

    let mut path = VOID_PTR;

    let mut r#type = 0;

    let mut i = 0;

    let bl_ptr = buffer.as_ptr();

    'z: loop {
        let arg_word = [buffer[i], buffer[i + 1], buffer[i + 2], buffer[i + 3]];

        let val_word = pos_to_new_or_null(i + 5, buffer);

        match &arg_word {
            b"ARGS" => {
                args[argi] = unsafe { bl_ptr.add(i + 5) };
                argi += 1;
            }

            b"ENVS" => {
                envs[envi] = unsafe { bl_ptr.add(i + 5) };
                envi += 1;
            }

            b"PATH" => {
                args[0] = unsafe { bl_ptr.add(i + 5) };
                path = unsafe { bl_ptr.add(i + 5) };
            }

            b"TYPE" => {
                r#type = i + 5;
            }

            _ => {}
        }

        if val_word.2 == true {
            break 'z;
        }

        i += val_word.1 + 6;
    }

    let mut ntype = 255;

    let val = pos_to_new_or_null(r#type, buffer).0;

    for v in PARSER_LIST {
        if val == v.keyword {
            ntype = v.special
        }
    }

    (ntype, path, args, envs)
}

pub fn i32_to_null_terminated_bytes(val: i32) -> [u8; 12] {
    let mut buf = [0u8; 12];

    let mut temp = [0u8; 11];
    let mut len = 0;

    let is_negative = val < 0;
    let mut n = if is_negative {
        val.wrapping_neg() as u32
    } else {
        val as u32
    };

    if n == 0 {
        temp[0] = b'0';
        len = 1;
    } else {
        while n > 0 {
            temp[len] = b'0' + (n % 10) as u8;
            n /= 10;
            len += 1;
        }
    }

    if is_negative {
        temp[len] = b'-';
        len += 1;
    }

    for i in 0..len {
        buf[i] = temp[len - 1 - i];
    }

    buf[len] = b'\0';

    buf
}

pub fn execve_error_str(errno: i32) -> &'static str {
    match errno.abs() {
        7 => "E2BIG: Argument list or environment list is too large.",
        13 => {
            "EACCES: Permission denied (lacks execute permission, path search denied, or mounted noexec)."
        }
        11 => "EAGAIN: Resource temporarily unavailable (RLIMIT_NPROC limit reached).",
        14 => "EFAULT: Bad address (filename, argv, or envp points outside accessible memory).",
        22 => "EINVAL: Invalid argument (e.g., ELF executable has multiple PT_INTERP segments).",
        5 => "EIO: An I/O error occurred while reading from the filesystem.",
        21 => "EISDIR: An ELF interpreter was specified as a directory.",
        80 => "ELIBBAD: An ELF interpreter was not in a recognized format.",
        40 => "ELOOP: Too many symbolic links encountered in resolving the path or interpreter.",
        24 => "EMFILE: The per-process limit on open file descriptors (RLIMIT_NOFILE) was reached.",
        36 => "ENAMETOOLONG: Filename or path string exceeds maximum allowed length.",
        23 => "ENFILE: The system-wide limit on the total number of open files was reached.",
        2 => "ENOENT: No such file or directory (executable or script interpreter does not exist).",
        8 => {
            "ENOEXEC: Exec format error (unrecognized binary format, wrong architecture, or invalid magic bytes)."
        }
        12 => "ENOMEM: Out of memory (insufficient kernel memory available).",
        20 => "ENOTDIR: A component of the path prefix is not a directory.",
        1 => {
            "EPERM: Operation not permitted (filesystem mounted nosuid, process is traced, or missing capabilities)."
        }
        26 => {
            "ETXTBSY: Text file busy (the executable is currently open for writing by another process)."
        }
        _ => "Unknown or undocumented execve error.",
    }
}

#[inline(always)]
pub fn blueprint_fork_and_execve(
    info: (u8, *const u8, [*const u8; MAX_ARGS], [*const u8; MAX_ENVS]),
) -> i32 {
    let pid = unsafe { fork() };
    if pid == 0 {
        if info.0 == 1 || info.0 == 3 {
            unsafe { setsid() };
        }
        unsafe {
            let q = execve(info.2.as_ptr(), info.3.as_ptr(), info.1);

            print("something died due to: ");
            print(execve_error_str(q as i32));

            exit(1);
        }
    };

    if pid < 0 {
        print("Oops");
    }

    pid
}
#[inline(always)]
pub fn full_parse(buffer: &mut [u8]) -> (i32, u8) {
    let entry = get_entry_from_file(buffer);
    print("entry read");

    let forked = blueprint_fork_and_execve(entry);
    print("forked");

    (forked, entry.0)
}
