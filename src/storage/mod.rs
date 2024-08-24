pub mod filesystem;

use anyhow::Result;
pub trait Store {
    fn init_dot_git(&self) -> Result<()>;
    fn set_head(file: &str, ref_content: &str) -> Result<()>;
}
