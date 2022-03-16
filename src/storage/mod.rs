pub mod disk;
mod page;

use color_eyre::Result;

use self::page::Page;

const PAGE_SIZE: u32 = 8192;

#[derive(Copy, Clone)]
pub struct PageId(u32);

#[derive(Copy, Clone)]
pub struct SlotId(u32);

#[derive(Copy, Clone)]
pub struct RecordId(PageId, SlotId);

pub trait Storage {
    fn get_page(pgid: PageId) -> Result<Page>;

    fn flush_page(page: Page) -> Result<()>;
}
