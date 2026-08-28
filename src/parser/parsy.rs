use super::super::new_asm::prelude::VOID_PTR;

//Parsing via the idiomatic way (so no pointers)
use super::types::*;
use super::{MAX_ARGS, MAX_ENVS};
#[inline(always)]
fn get_word(array: &mut [u8]) -> (&[u8], usize, bool) {
    let len = array
        .iter()
        .position(|&b| b == b'\n' || b == b'\0')
        .unwrap_or(array.len());

    let dead = {
        if array[len] == b'\n' {
            array[len] = b'\0';
            false
        } else {
            true
        }
    };

    (&array[..len], len + 1, dead)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ParsingResult {
    pub r#type: SpecialType,
    pub path: *const u8,
    pub args: [*const u8; MAX_ARGS],
    pub envs: [*const u8; MAX_ENVS],
}

//This matches THE CORE TYPE
#[inline(always)]
fn match_keyword(keyword: &[u8]) -> CoreType {
    match keyword {
        b"ARGS" => CoreType::Arg,
        b"ENVS" => CoreType::Env,
        b"TYPE" => CoreType::Type,
        b"PATH" => CoreType::Path,
        _ => CoreType::Undefined,
    }
}

//This matches the TYPE (so daemon, oneshot, waitfor etc...)
#[inline(always)]
fn match_type(value: &[u8]) -> SpecialType {
    match value {
        b"ONESHOT" => SpecialType::Oneshot,
        b"WAITFOR" => SpecialType::Waitfor,
        b"DAEMON" => SpecialType::Daemon,
        b"WAITFORD" => SpecialType::Waitford,
        _ => SpecialType::NoType,
    }
}

#[inline(always)]
pub fn parse(buf: &mut [u8]) -> ParsingResult {
    let mut result = ParsingResult {
        r#type: SpecialType::NoType,
        path: VOID_PTR,
        args: [VOID_PTR; MAX_ARGS],
        envs: [VOID_PTR; MAX_ENVS],
    };

    let mut i = 0;

    let mut argi = 1;
    let mut envi = 0;

    'z: loop {
        let keyword = (&buf[i..(i + 4)], 5, buf[i + 4] == 0);
        if keyword.2 {
            break 'z;
        }
        i += keyword.1;

        let core_type = match_keyword(keyword.0);

        let value = get_word(&mut buf[i..]);
        i += value.1;

        match core_type {
            CoreType::Arg => {
                result.args[argi] = value.0.as_ptr();
                argi += 1;
            }
            CoreType::Env => {
                result.envs[envi] = value.0.as_ptr();
                envi += 1;
            }
            CoreType::Path => {
                let ptr = value.0.as_ptr();
                result.path = ptr;
                result.args[0] = ptr;
            }
            CoreType::Type => {
                let find_out = match_type(value.0);
                result.r#type = find_out;
            }
            CoreType::Undefined => {}
        }
        if value.2 {
            break 'z;
        }
    }

    result
}
