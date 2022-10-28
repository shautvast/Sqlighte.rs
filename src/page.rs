use crate::bytebuffer::ByteBuffer;
use crate::database;
const POSITION_CELL_COUNT: u32 = 3;
const START_OF_CONTENT_AREA: u32 = 5;

pub enum PageType {
    Leaf,
    Interior,
}

/// Represents an SQLite page
pub struct Page {
    data: ByteBuffer,
    key: i64,
    children: Vec<Page>,
    number: u32,
    page_type: PageType,
}

impl Page {
    fn with_capacity(size: u16, page_type: PageType) -> Self {
        Self {
            data: ByteBuffer::new(size as usize),
            key: 0,
            children: Vec::new(),
            number: 0,
            page_type,
        }
    }

    fn new_leaf() -> Self {
        let mut page = Page::with_capacity(database::DEFAULT_PAGE_SIZE, PageType::Leaf);
        page.put_u8(database::TABLE_LEAF_PAGE);
        page
    }

    fn new_interior() -> Self {
        let mut page = Page::with_capacity(database::DEFAULT_PAGE_SIZE, PageType::Interior);
        page.put_u8(database::TABLE_LEAF_PAGE);
        page
    }

    pub fn add_child(&mut self, child: Self) {
        self.children.push(child);
    }

    pub fn fw_position(&mut self, new_position: usize) {
        self.data.fw_position = new_position;
    }

    pub fn bw_position(&mut self, new_position: usize) {
        self.data.bw_position = new_position;
    }

    pub fn put_u8a(&mut self, value: &[u8]) {
        self.data.put_u8a(value);
    }

    pub fn put_u8(&mut self, value: u8) {
        self.data.put_u8(value);
    }

    pub fn put_u16(&mut self, value: u16) {
        self.data.put_u16(value);
    }

    pub fn put_u32(&mut self, value: u32) {
        self.data.put_u32(value);
    }

    // may panic
    pub fn get_page_nr_last_child(self) -> u32 {
        self.children[self.children.len()-1].number
    }
}