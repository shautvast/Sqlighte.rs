use crate::{DEFAULT_PAGE_SIZE, TABLE_LEAF_PAGE};
use crate::bytebuffer::ByteBuffer;

const POSITION_CELL_COUNT: u32 = 3;
const START_OF_CONTENT_AREA: u32 = 5;

pub enum PageType {
    Leaf,
    Interior,
}

/// Represents an SQLite page
struct Page {
    data: ByteBuffer,
    key: i64,
    children: Vec<Page>,
    number: u32,
    page_type: PageType,
}

impl Page {
    fn with_capacity(size: usize, page_type: PageType) -> Self {
        Self {
            data: ByteBuffer::new(size),
            key: 0,
            children: Vec::new(),
            number: 0,
            page_type,
        }
    }

    fn new_leaf() -> Self {
        let mut page = Page::with_capacity(DEFAULT_PAGE_SIZE, PageType::Leaf);
        page.put_u8(TABLE_LEAF_PAGE);
        page
    }

    fn new_interior() -> Self {
        let mut page = Page::with_capacity(DEFAULT_PAGE_SIZE, PageType::Interior);
        page.put_u8(TABLE_LEAF_PAGE);
        page
    }

    fn add_child(&mut self, child: Self) {
        self.children.push(child);
    }

    fn fw_position(&mut self, new_position: usize) {
        self.data.fw_position = new_position;
    }

    fn bw_position(&mut self, new_position: usize) {
        self.data.bw_position = new_position;
    }

    fn put_u8a(&mut self, value: &[u8]) {
        self.data.put_u8a(value);
    }

    fn put_u8(&mut self, value: u8) {
        self.data.put_u8(value);
    }

    fn put_u16(&mut self, value: u16) {
        self.data.put_u16(value);
    }

    fn put_u32(&mut self, value: u32) {
        self.data.put_u32(value);
    }

    // may panic
    fn get_page_nr_last_child(self) -> u32 {
        self.children[self.children.len()-1].number
    }
}