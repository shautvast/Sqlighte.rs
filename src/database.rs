use crate::page::Page;


fn write_header(mut rootpage: Page, n_pages: u32) {
    rootpage.put_u8a(&MAGIC_HEADER);
    rootpage.put_u16(DEFAULT_PAGE_SIZE);
    rootpage.put_u8(FILE_FORMAT_WRITE_VERSION);
    rootpage.put_u8(FILE_FORMAT_READ_VERSION);
    rootpage.put_u8(RESERVED_SIZE);
    rootpage.put_u8(MAX_EMBED_PAYLOAD_FRACTION);
    rootpage.put_u8(MIN_EMBED_PAYLOAD_FRACTION);
    rootpage.put_u8(LEAF_PAYLOAD_FRACTION);
    rootpage.put_u32(FILECHANGE_COUNTER);
    rootpage.put_u32(n_pages);// file size in pages
    rootpage.put_u32(FREELIST_TRUNK_PAGE_HUMBER);// Page number of the first freelist trunk page.
    rootpage.put_u32(TOTAL_N_FREELIST_PAGES);
    rootpage.put_u32(SCHEMA_COOKIE);
    rootpage.put_u32(SQLITE_SCHEMAVERSION);
    rootpage.put_u32(SUGGESTED_CACHESIZE);
    rootpage.put_u32(LARGEST_ROOT_BTREE_PAGE);
    rootpage.put_u32(ENCODING_UTF8);
    rootpage.put_u32(USER_VERSION);
    rootpage.put_u32(VACUUM_MODE_OFF);// True (non-zero) for incremental-vacuum mode. False (zero) otherwise.
    rootpage.put_u32(APP_ID);// Application ID
    rootpage.put_u8a(&FILLER);// Reserved for expansion. Must be zero.
    rootpage.put_u8a(&VERSION_VALID_FOR);// The version-valid-for number
    rootpage.put_u8a(&SQLITE_VERSION);// SQLITE_VERSION_NUMBER
    rootpage.put_u8(TABLE_LEAF_PAGE); // leaf table b-tree page for schema
    rootpage.put_u16(NO_FREE_BLOCKS); // zero if there are no freeblocks
    rootpage.put_u16(1); // the number of cells on this page
}

const MAGIC_HEADER: [u8; 16] = [0x53, 0x51, 0x4c, 0x69, 0x74, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x33, 0x00];
pub const DEFAULT_PAGE_SIZE: u16 = 4096;
const FILE_FORMAT_WRITE_VERSION: u8 = 1;
const FILE_FORMAT_READ_VERSION: u8 = 1;
const RESERVED_SIZE: u8 = 0;
const MAX_EMBED_PAYLOAD_FRACTION: u8 = 0x40;
const MIN_EMBED_PAYLOAD_FRACTION: u8 = 0x20;
const LEAF_PAYLOAD_FRACTION: u8 = 0x20;
const FILECHANGE_COUNTER: u32 = 1;
const FREELIST_TRUNK_PAGE_HUMBER: u32 = 0;
const TOTAL_N_FREELIST_PAGES: u32 = 0;
const SCHEMA_COOKIE: u32 = 1;
const SQLITE_SCHEMAVERSION: u32 = 4;
const SUGGESTED_CACHESIZE: u32 = 0;
const LARGEST_ROOT_BTREE_PAGE: u32 = 0;
const ENCODING_UTF8: u32 = 1;
const USER_VERSION: u32 = 0;
const VACUUM_MODE_OFF: u32 = 0;
const APP_ID: u32 = 0;
const FILLER: [u8; 20] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
const VERSION_VALID_FOR: [u8; 4] = [0, 0, 0x03, 250];
const SQLITE_VERSION: [u8; 4] = [0x00, 0x2e, 0x5F, 0x1A];
const NO_FREE_BLOCKS: u16 = 0;
pub const TABLE_LEAF_PAGE: u8 = 0x0d;
pub const TABLE_INTERIOR_PAGE: u8 = 0x05;
const INDEX_LEAF_PAGE: u8 = 0x0a;
const INDEX_INTERIOR_PAGE: u8 = 0x02;