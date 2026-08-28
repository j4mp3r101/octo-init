pub mod i32;
pub mod parsy;
pub mod resp_hand;
pub mod types;

pub const MAX_ARGS: usize = 64;
pub const MAX_ENVS: usize = 64;

pub const RAW_BUF_SIZE_GET: usize = 2048;
pub const RAW_BUF_SIZE_READ: usize = 2048;
