pub const HEAD: &'static str = "HEAD";
pub const MASTER: &'static str = "refs/heads/master";
pub const MAIN: &'static str = "main";
pub const MAIN_REF: &'static str = "refs/heads/main";

pub mod commit;
pub mod filemode;
pub mod hash;
pub mod index;
pub mod object;
pub mod reference;
pub mod store;
pub mod tree;
