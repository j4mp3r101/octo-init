//parser.rs was made to easily add entries to the parses w/o changing stage2.

pub const RAW_BUF_SIZE_GET: usize = 2048;
pub const RAW_BUF_SIZE_READ: usize = 2048;

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

pub const PARSER_LIST: [ParserEntry; 5] = [
    ParserEntry {
        keyword: b"ONESHOT\0",
        special: 0,
    },
    ParserEntry {
        keyword: b"DAEMON\0",

        special: 1,
    },
    ParserEntry {
        keyword: b"TTY\0",

        special: 1,
    },
    ParserEntry {
        keyword: b"WAITFOR\0",

        special: 2,
    },
    ParserEntry {
        keyword: b"WAITFORD",

        special: 3,
    },
];

//Some keys.

pub const TYPE_KEY: &'static [u8; 4] = b"TYPE";
pub const ENVP_KEY: &'static [u8; 3] = b"ENV";
pub const ARGS_KEY: &'static [u8; 3] = b"ARG";
pub const PATH_KEY: &'static [u8; 4] = b"PATH";

pub const KEY_LIST: [&'static [u8]; 4] = [ENVP_KEY, ARGS_KEY, PATH_KEY, TYPE_KEY];
