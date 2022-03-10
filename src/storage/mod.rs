pub mod disk;

use color_eyre::Result;

const PAGE_SIZE: u32 = 8192;

pub struct PageId(u32);
pub struct SlotId(u32);
pub struct RecordId(PageId, SlotId);

pub struct PageHeader;

pub struct Page {
    pub id: PageId,
    pub header: PageHeader,
    pub slots: Vec<u32>,
    pub tuples: Vec<Vec<u8>>,
}

pub trait Storage {
    fn get_page(id: PageId) -> Result<Page>;

    fn flush_page(page: Page) -> Result<()>;
}
