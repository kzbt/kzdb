#[doc(inline)]
pub use std;
use std::path::PathBuf;

use anyhow::Result;
use storage::disk::DiskManager;

mod btree;
mod storage;

fn main() -> Result<()> {
    let db_path = PathBuf::from("/tmp/kzdb/test.db");
    let dm = DiskManager::new(db_path);
    Ok(())
}
