use std::collections::{HashMap, LinkedList};

use super::{page::Page, FrameId, PageId};

const BUFFER_POOL_SIZE: usize = 2000;

pub struct BufferPool {
    pages: Vec<Page>,
    page_table: HashMap<PageId, FrameId>,
    free_frames: LinkedList<FrameId>,
}

impl BufferPool {
    pub fn new() -> Self {
        let free_frames = (0..BUFFER_POOL_SIZE)
            .into_iter()
            .map(|i| FrameId(i as u32))
            .collect();

        Self {
            pages: Vec::with_capacity(BUFFER_POOL_SIZE),
            page_table: HashMap::with_capacity(BUFFER_POOL_SIZE),
            free_frames,
        }
    }
}
