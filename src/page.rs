use crate::bytebuffer::ByteBuffer;
use crate::database;

pub const POSITION_CELL_COUNT: u16 = 3;
const START_OF_CONTENT_AREA: u32 = 5;

pub enum PageType {
    Leaf,
    Interior,
}

/// Represents an SQLite page
pub struct Page {
    data: ByteBuffer,
    pub key: u64,
    children: Vec<Page>,
    number: u32,
    page_type: PageType,
}

impl Page {
    pub fn with_capacity(size: u16, page_type: PageType) -> Self {
        Self {
            data: ByteBuffer::new(size as u16),
            key: 0,
            children: Vec::new(),
            number: 0,
            page_type,
        }
    }

    pub fn new_leaf() -> Self {
        let mut page = Page::with_capacity(database::DEFAULT_PAGE_SIZE, PageType::Leaf);
        page.put_u8(database::TABLE_LEAF_PAGE);
        page
    }

    pub fn new_interior() -> Self {
        let mut page = Page::with_capacity(database::DEFAULT_PAGE_SIZE, PageType::Interior);
        page.put_u8(database::TABLE_LEAF_PAGE);
        page
    }

    pub fn add_child(&mut self, child: Self) {
        self.children.push(child);
    }

    pub fn set_fw_position(&mut self, new_position: u16) {
        self.data.fw_position = new_position;
    }

    pub fn get_fw_position(&self) -> u16 {
        self.data.fw_position
    }
    pub fn set_bw_position(&mut self, new_position: u16) {
        self.data.bw_position = new_position;
    }

    pub fn get_bw_position(&self) -> u16 {
        self.data.bw_position
    }

    pub fn put_u8a(&mut self, value: &[u8]) {
        self.data.put_bytes(value);
    }

    pub fn put_u8a_bw(&mut self, value: &[u8]) {
        self.data.put_bytes_bw(value);
    }

    pub fn put_vec_u8_bw(&mut self, value: Vec<u8>) {
        self.data.put_bytes_bw(&value);
    }

    pub fn put_u8(&mut self, value: u8) {
        self.data.put_u8(value);
    }

    pub fn put_u8_bw(&mut self, value: u8) {
        self.data.put_u8_bw(value);
    }

    pub fn put_u16(&mut self, value: u16) {
        self.data.put_u16(value);
    }

    pub fn put_u32(&mut self, value: u32) {
        self.data.put_u32(value);
    }

    // may panic
    pub fn get_page_nr_last_child(self) -> u32 {
        self.children[self.children.len() - 1].number
    }
}