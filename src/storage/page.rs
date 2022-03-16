use std::mem::{size_of, size_of_val};

use super::{PageId, RecordId, PAGE_SIZE};

const ZERO_U32: [u8; 4] = 0u32.to_be_bytes();
const HEADER_SIZE: usize = size_of::<PageHeader>();

pub struct PageHeader {
    pgid: PageId,
    prev_page: Option<PageId>,
    next_page: Option<PageId>,
    free_offset: u32,
    num_tuples: u32,
}

impl PageHeader {
    fn write_to(&self, buffer: &mut [u8]) {
        let pgid = self.pgid.0.to_be_bytes();
        let prev_page = self.prev_page.map_or(ZERO_U32, |p| p.0.to_be_bytes());
        let next_page = self.next_page.map_or(ZERO_U32, |p| p.0.to_be_bytes());
        let free_offset = self.free_offset.to_be_bytes();
        let num_tuples = self.num_tuples.to_be_bytes();

        let mut start = 0;
        let mut end = 0;
        [pgid, prev_page, next_page, free_offset, num_tuples]
            .iter()
            .for_each(|item| {
                let item_size = size_of_val(item);
                end += item_size;
                buffer[start..end].copy_from_slice(item);
                start += item_size;
            });
    }
}

pub struct Tuple {
    rid: RecordId,
    data: Vec<u8>,
}

pub struct Page {
    pub header: PageHeader,
    pub slots: Vec<u32>,
    pub data: Vec<u8>,
}

impl Page {
    fn new(pgid: PageId, prev_page: Option<PageId>) -> Self {
        let data = vec![0; PAGE_SIZE as usize - HEADER_SIZE];

        Self {
            header: PageHeader {
                pgid,
                prev_page,
                next_page: None,
                free_offset: data.len() as u32,
                num_tuples: 0,
            },
            slots: Vec::new(),
            data,
        }
    }

    fn insert_tuple(&mut self, tuple: &Tuple) {
        let size = tuple.data.len();
        let offset = self.header.free_offset as usize - size;

        self.data[offset..self.header.free_offset as usize].copy_from_slice(&tuple.data);

        self.header.free_offset = offset as u32;
        self.header.num_tuples += 1;
        self.update_header();
    }

    fn update_header(&mut self) {
        self.header.write_to(&mut self.data[..HEADER_SIZE]);
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::{page::HEADER_SIZE, PageId, RecordId, SlotId, PAGE_SIZE};

    use super::{Page, Tuple};

    #[test]
    fn insert_tuple() {
        let mut page = Page::new(PageId(0), None);
        let tuple_size = 50;
        let tuple = Tuple {
            rid: RecordId(PageId(0), SlotId(0)),
            data: vec![1; tuple_size],
        };

        page.insert_tuple(&tuple);

        let page_data_size = PAGE_SIZE as usize - HEADER_SIZE;
        assert_eq!(
            page.header.free_offset as usize,
            page_data_size - tuple_size
        );
        assert_eq!(page.header.num_tuples, 1);

        let tuple2 = Tuple {
            rid: RecordId(PageId(0), SlotId(1)),
            data: vec![2; tuple_size],
        };
        page.insert_tuple(&tuple2);
        assert_eq!(
            page.header.free_offset as usize,
            page_data_size - (tuple_size * 2)
        );
        assert_eq!(page.header.num_tuples, 2);

        assert_eq!(
            page.data[page.header.free_offset as usize..(page_data_size - tuple_size)],
            tuple2.data
        );
        assert_eq!(page.data[(page_data_size - tuple_size)..], tuple.data)
    }

    #[test]
    fn update_header() {
        let mut page = Page::new(PageId(42), None);
        page.update_header();

        let mut expected = vec![0; HEADER_SIZE];
        expected[..4].copy_from_slice(&42u32.to_be_bytes());
        expected[12..16].copy_from_slice(&(page.data.len() as u32).to_be_bytes());

        let actual = &page.data[..HEADER_SIZE];
        assert_eq!(actual, expected)
    }
}
