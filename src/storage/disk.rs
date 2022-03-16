use super::PageId;
use super::PAGE_SIZE;
use color_eyre::{eyre::Context, Result};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    os::unix::prelude::FileExt,
    path::PathBuf,
};

pub struct DiskManager {
    file: File,
}

impl DiskManager {
    pub fn new(path: PathBuf) -> Self {
        return Self {
            file: OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path)
                .expect("Failed to open db file"),
        };
    }

    pub fn read_page(&mut self, pgid: PageId, buf: &mut [u8]) -> Result<()> {
        let offset = pgid.0 * PAGE_SIZE;
        self.file
            .read_exact_at(buf, offset as u64)
            .wrap_err_with(|| format!("Failed to read page id: {}", pgid.0))
    }

    pub fn write_page(&mut self, pgid: PageId, buf: &[u8]) -> Result<()> {
        let offset = pgid.0 * PAGE_SIZE;
        let _ = self.file.write_at(buf, offset as u64)?;
        self.file
            .flush()
            .wrap_err_with(|| format!("Failed to flush while writing page id: {}", pgid.0))
    }
}

#[cfg(test)]
mod tests {

    use assert_fs::{fixture::PathChild, TempDir};

    use crate::storage::{PageId, PAGE_SIZE};

    use super::DiskManager;

    fn with_disk_manager<F>(mut func: F)
    where
        F: FnMut(DiskManager),
    {
        let tmp = TempDir::new().unwrap();
        let file = tmp.child("test.db");
        let dm = DiskManager::new(file.to_path_buf());
        func(dm);
        tmp.close().unwrap()
    }

    #[test]
    fn read_and_write_page() {
        with_disk_manager(|mut dm| {
            let buf1: [u8; PAGE_SIZE as usize] = [b'a'; PAGE_SIZE as usize];
            let buf2: [u8; PAGE_SIZE as usize] = [b'b'; PAGE_SIZE as usize];
            dm.write_page(PageId(1), &buf1).unwrap();
            dm.write_page(PageId(2), &buf2).unwrap();

            let mut read1 = [0; PAGE_SIZE as usize];
            let mut read2 = [0; PAGE_SIZE as usize];
            dm.read_page(PageId(1), &mut read1).unwrap();
            dm.read_page(PageId(2), &mut read2).unwrap();

            assert_eq!(read1, buf1);
            assert_eq!(read2, buf2);
        })
    }
}
