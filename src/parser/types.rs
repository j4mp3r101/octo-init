#[derive(Clone, Copy)]
pub enum SpecialType {
    Oneshot,
    Daemon,
    Waitfor,
    Waitford,
    NoType,
}
#[derive(Clone, Copy)]
pub enum CoreType {
    Type,
    Path,
    Arg,
    Env,
    Undefined,
}
