pub mod filesys;
pub mod filesystem;

use anyhow::Result;
pub trait Store {
    fn init_dot_git(&self) -> Result<()>;
}
