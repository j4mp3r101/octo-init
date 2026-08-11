//parser.rs was made to easily add entries to the parses w/o changing stage2.

use crate::asm::stage2::{close, dup2, ioctl, open, setsid};

//Add whatever u want in this enum. needs a matching function tho.
#[derive(Copy, Clone)]
pub enum Action {
    None, //as it says -> none
    Tty,  //the tty() function
}

impl Action {
    pub fn run(&self) {
        match &self {
            Action::None => droppy(),
            Action::Tty => tty(),
        }
    }
}

const DEFAULT: ParserEntry = ParserEntry {
    keyword: *b"ONESHOT\0",
    pre_spawn_action: Action::None,
    pre_reap_action: Action::None,

    post_spawn_action: Action::None,
    post_reap_action: Action::None,

    special: 0,
};
//Each entry needs to be in the form of this struct
#[repr(C, packed)]
pub struct ParserEntry {
    pub keyword: [u8; 8],

    //runs in stage 2.
    pub pre_spawn_action: Action,
    //runs after the proccess is executed in stage 2.
    pub post_spawn_action: Action,

    //runs before the proccess is ran in stage 3.
    pub pre_reap_action: Action,

    //runs after the proccess is ran in stage 3.
    pub post_reap_action: Action,

    //Special is unique so ill give a bit of documentation.
    //special = 0 means the entry ISNT SAVED and ISNT WAITED FOR.
    //special = 1 means the entry IS SAVED but ISNT WAITED FOR.
    //special = 2 means the entry ISNT SAVED but IS WAITED FOR.
    //special = 3 means the entry IS SAVED and IS WAITER FOR.
    pub special: u8,
}

//Some basic functions.

//place holder for null.
pub fn droppy() {}

//for TTYs
pub fn tty() {
    unsafe { setsid() };

    let mut tty_fd = unsafe { open(b"/dev/console\0".as_ptr(), 2, 0) };

    if tty_fd < 0 {
        tty_fd = unsafe { open(b"/dev/tty1\0".as_ptr(), 2, 0) };
    }

    if tty_fd >= 0 {
        unsafe {
            ioctl(tty_fd, 0x540E, 0);

            dup2(tty_fd, 0);
            dup2(tty_fd, 1);
            dup2(tty_fd, 2);
        };

        if tty_fd > 2 {
            unsafe { close(tty_fd) };
        }
    }
}
//now the parser list.
//this will be acc loaded into the init.

pub const PARSER_LIST: [ParserEntry; 5] = [
    ParserEntry {
        keyword: *b"ONESHOT\0",
        pre_spawn_action: Action::None,
        pre_reap_action: Action::None,

        post_spawn_action: Action::None,
        post_reap_action: Action::None,
        special: 0,
    },
    ParserEntry {
        keyword: *b"DAEMON\0\0",
        pre_spawn_action: Action::None,
        pre_reap_action: Action::None,

        post_spawn_action: Action::None,
        post_reap_action: Action::None,

        special: 1,
    },
    ParserEntry {
        keyword: *b"TTY\0\0\0\0\0",
        pre_spawn_action: Action::Tty,
        pre_reap_action: Action::Tty,

        post_spawn_action: Action::None,
        post_reap_action: Action::None,

        special: 1,
    },
    ParserEntry {
        keyword: *b"WAITFOR\0",
        pre_spawn_action: Action::None,
        pre_reap_action: Action::None,

        post_spawn_action: Action::None,
        post_reap_action: Action::None,

        special: 2,
    },
    ParserEntry {
        keyword: *b"WAITFORD",
        pre_spawn_action: Action::None,
        pre_reap_action: Action::None,

        post_spawn_action: Action::None,
        post_reap_action: Action::None,

        special: 3,
    },
];
