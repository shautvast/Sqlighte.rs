use crate::database;
use byteorder::{BigEndian, ByteOrder};

pub const POSITION_CELL_COUNT: u16 = 3;
pub const START_OF_CONTENT_AREA: u16 = 5;
pub const START_OF_INTERIOR_PAGE: u16 = 12;
pub const POSITION_RIGHTMOST_POINTER_LEAFPAGES: u16 = 8;

pub enum PageType {
    Leaf,
    Interior,
    Root,
    Other,
}

/// Represents an SQLite page
pub struct Page {
    pub data: Vec<u8>,
    pub fw_position: u16,
    pub bw_position: u16,
    pub key: u64,
    pub children: Vec<Page>,
    pub number: u32,
    pub page_type: PageType,
}

impl Page {
    fn with_capacity(size: u16, page_type: PageType) -> Self {
        Self {
            data: vec![0; size as usize],
            fw_position: 0,
            bw_position: size,
            key: 0,
            children: Vec::new(),
            number: 0,
            page_type,
        }
    }

    fn default(size: usize) -> Self {
        Self {
            data: vec![0; size],
            fw_position: 0,
            bw_position: size as u16,
            key: 0,
            children: Vec::new(),
            number: 0,
            page_type: PageType::Other,
        }
    }

    pub fn new_root() -> Self {
        Page::with_capacity(database::DEFAULT_PAGE_SIZE, PageType::Leaf)
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

    pub fn put_bytes(&mut self, bytes: &[u8]) {
        for v in bytes {
            self.data[self.fw_position as usize] = *v;
            self.fw_position += 1;
        }
    }

    pub fn put_bytes_bw(&mut self, bytes: &[u8]) {
        self.bw_position -= bytes.len() as u16;
        for v in bytes {
            self.data[self.bw_position as usize] = *v;
            self.bw_position += 1;
        }
        self.bw_position -= bytes.len() as u16;
    }

    pub fn put_u8(&mut self, value: u8) {
        self.put_bytes(&[value]);
    }

    pub fn put_u8_bw(&mut self, value: u8) {
        self.put_bytes_bw(&[value]);
    }

    pub fn put_u16(&mut self, value: u16) {
        self.put_bytes(&u16_to_bytes(value));
    }

    pub fn put_u16_bw(&mut self, value: u16) {
        self.put_bytes_bw(&u16_to_bytes(value));
    }

    pub fn put_u32(&mut self, value: u32) {
        self.put_bytes(&u32_to_bytes(value));
    }

    pub fn put_u32_bw(&mut self, value: u32) {
        self.put_bytes_bw(&u32_to_bytes(value));
    }

    // may panic
    pub fn get_page_nr_last_child(&self) -> u32 {
        self.children[self.children.len() - 1].number
    }

    pub fn get_u16(&self) -> u16 {
        let position = self.fw_position as usize;
        ((self.data[position] as u16) << 8) + (self.data[position + 1]) as u16
        // does not increase the fw pointerr
    }
}

fn u16_to_bytes(value: u16) -> [u8; 2] {
    let mut buf = [0; 2];
    BigEndian::write_u16(&mut buf, value);
    buf
}

fn u32_to_bytes(value: u32) -> [u8; 4] {
    let mut buf = [0; 4];
    BigEndian::write_u32(&mut buf, value);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8() {
        let mut b = Page::default(1);
        b.put_u8(64_u8);
        assert_eq!(b.data[0], 64);
    }

    #[test]
    fn test_u8a() {
        let mut b = Page::default(2);
        b.put_bytes(&[1, 2]);
        assert_eq!(b.data[0], 1);
        assert_eq!(b.data[1], 2);
    }

    #[test]
    fn test_u16() {
        let mut b = Page::default(2);
        b.put_u16(4096);
        assert_eq!(b.data[0], 16);
        assert_eq!(b.data[1], 0);
    }

    #[test]
    fn test_u32() {
        let mut b = Page::default(4);
        b.put_u32(0xFFFFFFFF);
        assert_eq!(b.data[0], 0xFF);
        assert_eq!(b.data[1], 0xFF);
        assert_eq!(b.data[2], 0xFF);
        assert_eq!(b.data[3], 0xFF);
    }

    #[test]
    fn test_u16_position() {
        let mut b = Page::default(4);
        b.fw_position = 2;
        b.put_u16(4096);
        assert_eq!(b.data[0], 0);
        assert_eq!(b.data[1], 0);
        assert_eq!(b.data[2], 16);
        assert_eq!(b.data[3], 0);
    }

    #[test]
    fn test_u16_backwards() {
        let mut b = Page::default(4);
        b.put_u16_bw(0x1000);
        assert_eq!(b.data[0], 0);
        assert_eq!(b.data[1], 0);
        assert_eq!(b.data[2], 0x10);
        assert_eq!(b.data[3], 0x00);
    }

    #[test]
    fn test_u16_2_directions() {
        let mut b = Page::default(5);
        b.put_u16(0x1001);
        b.put_u16_bw(0x1000);
        assert_eq!(b.data[0], 0x10);
        assert_eq!(b.data[1], 0x01);
        assert_eq!(b.data[2], 0); // decimal suggests this value has not been written
        assert_eq!(b.data[3], 0x10);
        assert_eq!(b.data[4], 0x00);
    }

    #[test]
    fn test_u32_2_directions() {
        let mut b = Page::default(9);
        b.put_u32(0x1001);
        b.put_u32_bw(0x1002);
        assert_eq!(b.data[0], 0x00);
        assert_eq!(b.data[1], 0x00);
        assert_eq!(b.data[2], 0x10);
        assert_eq!(b.data[3], 0x01);
        assert_eq!(b.data[4], 0);
        assert_eq!(b.data[5], 0x00);
        assert_eq!(b.data[6], 0x00);
        assert_eq!(b.data[7], 0x10);
        assert_eq!(b.data[8], 0x02);
    }
}
