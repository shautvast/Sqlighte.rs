mod page;
mod database;
mod bytebuffer;
mod values;
mod varint;
mod record;

const DEFAULT_PAGE_SIZE: usize = 4096;
const TABLE_INTERIOR_PAGE: u8 = 0x05;
const TABLE_LEAF_PAGE: u8 = 0x0D;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
