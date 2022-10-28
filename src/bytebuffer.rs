use byteorder::{BigEndian, ByteOrder};

/// bytebuffer that supports forward and backward writing (this is not endianness)
/// Reason: SQLite pages are written in 2 directions: from the front for the cell-pointers and from the back for the cells
/// - fixed size
/// - big endian only
pub struct ByteBuffer {
    pub data: Vec<u8>,
    pub fw_position: usize,
    pub bw_position: usize,
}

impl ByteBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
            fw_position: 0,
            bw_position: size,
        }
    }

    /// forward put unsigned byte array
    pub fn put_u8a(&mut self, bytes: &[u8]) {
        for v in bytes {
            self.data[self.fw_position] = *v;
            self.fw_position += 1;
        }
    }

    pub fn put_u8v(&mut self, bytes: &Vec<u8>) {
        for v in bytes {
            self.data[self.fw_position] = *v;
            self.fw_position += 1;
        }
    }

    /// backward put unsigned byte array
    pub fn put_u8a_bw(&mut self, bytes: &[u8]) {
        self.bw_position -= bytes.len();
        for v in bytes {
            self.data[self.bw_position] = *v;
            self.bw_position += 1;
        }
    }

    /// forward put unsigned byte
    pub fn put_u8(&mut self, byte: u8) {
        self.put_u8a(&[byte]);
    }

    /// backward put unsigned byte
    pub fn put_u8_bw(&mut self, byte: u8) {
        self.put_u8a_bw(&[byte]);
    }

    /// forward put unsigned 16bit integer
    pub fn put_u16(&mut self, val: u16) {
        let mut buf = [0; 2];
        BigEndian::write_u16(&mut buf, val);
        self.put_u8a(&buf);
    }

    /// backward put unsigned 16bit integer
    pub fn put_u16_bw(&mut self, val: u16) {
        let mut buf = [0; 2];
        BigEndian::write_u16(&mut buf, val);
        self.put_u8a_bw(&buf);
    }

    /// forward put unsigned 16bit integer
    pub fn put_u32(&mut self, val: u32) {
        let mut buf = [0; 4];
        BigEndian::write_u32(&mut buf, val);
        self.put_u8a(&buf);
    }

    /// backward put unsigned 32bit integer
    pub fn put_u32_bw(&mut self, val: u32) {
        let mut buf = [0; 4];
        BigEndian::write_u32(&mut buf, val);
        self.put_u8a_bw(&buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8() {
        let mut b = ByteBuffer::new(1);
        b.put_u8(64_u8);
        assert_eq!(b.data[0], 64);
    }

    #[test]
    fn test_u8a() {
        let mut b = ByteBuffer::new(2);
        b.put_u8a(&[1, 2]);
        assert_eq!(b.data[0], 1);
        assert_eq!(b.data[1], 2);
    }

    #[test]
    fn test_u16() {
        let mut b = ByteBuffer::new(2);
        b.put_u16(4096);
        assert_eq!(b.data[0], 16);
        assert_eq!(b.data[1], 0);
    }

    #[test]
    fn test_u32() {
        let mut b = ByteBuffer::new(4);
        b.put_u32(0xFFFFFFFF);
        assert_eq!(b.data[0], 0xFF);
        assert_eq!(b.data[1], 0xFF);
        assert_eq!(b.data[2], 0xFF);
        assert_eq!(b.data[3], 0xFF);
    }

    #[test]
    fn test_u16_position() {
        let mut b = ByteBuffer::new(4);
        b.fw_position = 2;
        b.put_u16(4096);
        assert_eq!(b.data[0], 0);
        assert_eq!(b.data[1], 0);
        assert_eq!(b.data[2], 16);
        assert_eq!(b.data[3], 0);
    }

    #[test]
    fn test_u16_backwards() {
        let mut b = ByteBuffer::new(4);
        b.put_u16_bw(0x1000);
        assert_eq!(b.data[0], 0);
        assert_eq!(b.data[1], 0);
        assert_eq!(b.data[2], 0x10);
        assert_eq!(b.data[3], 0x00);
    }

    #[test]
    fn test_u16_2_directions() {
        let mut b = ByteBuffer::new(5);
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
        let mut b = ByteBuffer::new(9);
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