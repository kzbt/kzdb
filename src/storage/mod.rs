mod bufferpool;
pub mod disk;
mod page;

use anyhow::Result;

use self::page::Page;

const PAGE_SIZE: u32 = 8192;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PageId(u32);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrameId(u32);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SlotId(u32);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecordId(PageId, SlotId);

pub trait Storage {
    fn get_page(pgid: PageId) -> Result<Page>;

    fn flush_page(page: Page) -> Result<()>;
}
