use crate::builder::DatabaseBuilder;
use crate::page;
use crate::page::{Page, PageType};
use crate::record::Record;
use crate::varint;
use std::io::{BufWriter, Error, Write};
use std::mem;

pub struct Database {
    schema: SchemaRecord,
    leaf_pages: Vec<Page>,
}

impl Database {
    pub fn new(schema: SchemaRecord, leaf_pages: Vec<Page>) -> Self {
        Self { schema, leaf_pages }
    }
}

impl From<DatabaseBuilder> for Database {
    fn from(mut dbb: DatabaseBuilder) -> Self {
        dbb.current_page.fw_position = page::POSITION_CELL_COUNT;
        dbb.current_page.put_u16(dbb.n_records_on_current_page);

        if dbb.n_records_on_current_page > 0 {
            dbb.current_page.put_u16(dbb.current_page.bw_position);
        } else {
            dbb.current_page.put_u16(dbb.current_page.bw_position - 1);
        }

        dbb.leaf_pages.push(dbb.current_page);
        Database::new(dbb.schema.unwrap_or_default(), dbb.leaf_pages)
    }
}

pub fn write_sqlite<W: Write>(database: Database, mut writer: BufWriter<W>) -> Result<(), Error> {
    let mut current_top_layer = database.leaf_pages;
    let mut n_pages = current_top_layer.len();
    while current_top_layer.len() > 1 {
        // db needs interior pages?
        current_top_layer = create_interior_pages(current_top_layer);
        n_pages += current_top_layer.len();
    }

    // assert(won't panic)
    let table_root_page = current_top_layer.get_mut(0).unwrap();
    writer.write_all(&create_header_page((n_pages + 1) as u32, database.schema).data)?; // 1 for header page

    set_childrefs_write(table_root_page, &mut writer, 3)
}

fn set_childrefs_write<W: Write>(
    page: &mut Page,
    writer: &mut BufWriter<W>,
    mut page_counter: u32,
) -> Result<(), Error> {
    if let PageType::Interior = page.page_type {
        page.fw_position = page::POSITION_CELL_COUNT;
        page.put_u16((page.children.len() - 1) as u16);

        for index in 0..page.children.len() - 1 {
            page.fw_position = page::START_OF_INTERIOR_PAGE + (index as u16) * 2;
            page.fw_position = page.get_u16();
            page.put_u32(page_counter);
            page_counter += 1;
        }

        page.fw_position = page::POSITION_RIGHTMOST_POINTER_LEAFPAGES;
        page.put_u32(page_counter);
        page_counter += 1;
    }

    writer.write_all(&page.data)?;

    for child in &mut page.children {
        set_childrefs_write(child, writer, page_counter)?;
    }
    Ok(())
}

fn create_header_page(n_pages: u32, schema: SchemaRecord) -> Page {
    let mut header_page = Page::new_root();
    write_header(&mut header_page, n_pages);

    let payload_location_write_location = header_page.fw_position; // mark current position

    let payload_location = write_schema(&mut header_page, schema); //write schema payload from the end
    header_page.fw_position = payload_location_write_location; // go back to marked position
    header_page.put_u16(payload_location); //payload start
    header_page.put_u8(0); // the number of fragmented free bytes within the cell content area
    header_page.put_u16(payload_location); // first cell
    header_page
}

fn write_schema(root_page: &mut Page, schema_record: SchemaRecord) -> u16 {
    let record: Record = schema_record.into();
    let bytes: Vec<u8> = record.into();
    root_page.put_bytes_bw(&bytes);
    root_page.bw_position
}

fn create_interior_pages(child_pages: Vec<Page>) -> Vec<Page> {
    let mut interior_pages = Vec::new();
    let mut interior_page = Page::new_interior();
    interior_page.key = child_pages.iter().map(|p| p.key).max().unwrap_or(0);
    interior_page.fw_position = page::START_OF_INTERIOR_PAGE;
    let children_length = child_pages.len();
    let mut last_leaf: Page = Page::new_leaf(); // have to assign :(
    for (child_count, leaf_page) in child_pages.into_iter().enumerate() {
        if child_count < children_length - 1 {
            if interior_page.bw_position <= interior_page.fw_position + 15 {
                // 15 is somewhat arbitrary, but safe
                interior_page.fw_position = page::START_OF_CONTENT_AREA;
                interior_page.put_u16(interior_page.bw_position);
                interior_page.fw_position += 5;
                interior_pages.push(mem::replace(&mut interior_page, Page::new_interior()));
                interior_page.fw_position = page::START_OF_INTERIOR_PAGE;
            }
            create_cell(&mut interior_page, &leaf_page);
            interior_page.add_child(leaf_page);
        } else {
            last_leaf = leaf_page;
        }
    }

    interior_page.fw_position = page::START_OF_CONTENT_AREA;
    interior_page.put_u16(interior_page.bw_position);
    interior_page.fw_position += 5;
    interior_page.add_child(last_leaf);
    interior_pages.push(interior_page);
    interior_pages
}

fn create_cell(interior_page: &mut Page, child_page: &Page) {
    let mut cell: Vec<u8> = vec![0; 5];
    cell.append(&mut varint::write(child_page.key));

    interior_page.put_bytes_bw(&cell);
    interior_page.put_u16(interior_page.bw_position);
}

fn write_header(rootpage: &mut Page, n_pages: u32) {
    rootpage.put_bytes(&MAGIC_HEADER);
    rootpage.put_u16(DEFAULT_PAGE_SIZE);
    rootpage.put_u8(FILE_FORMAT_WRITE_VERSION);
    rootpage.put_u8(FILE_FORMAT_READ_VERSION);
    rootpage.put_u8(RESERVED_SIZE);
    rootpage.put_u8(MAX_EMBED_PAYLOAD_FRACTION);
    rootpage.put_u8(MIN_EMBED_PAYLOAD_FRACTION);
    rootpage.put_u8(LEAF_PAYLOAD_FRACTION);
    rootpage.put_u32(FILECHANGE_COUNTER);
    rootpage.put_u32(n_pages); // file size in pages
    rootpage.put_u32(FREELIST_TRUNK_PAGE_HUMBER); // Page number of the first freelist trunk page.
    rootpage.put_u32(TOTAL_N_FREELIST_PAGES);
    rootpage.put_u32(SCHEMA_COOKIE);
    rootpage.put_u32(SQLITE_SCHEMAVERSION);
    rootpage.put_u32(SUGGESTED_CACHESIZE);
    rootpage.put_u32(LARGEST_ROOT_BTREE_PAGE);
    rootpage.put_u32(ENCODING_UTF8);
    rootpage.put_u32(USER_VERSION);
    rootpage.put_u32(VACUUM_MODE_OFF); // True (non-zero) for incremental-vacuum mode. False (zero) otherwise.
    rootpage.put_u32(APP_ID); // Application ID
    rootpage.put_bytes(&FILLER); // Reserved for expansion. Must be zero.
    rootpage.put_bytes(&VERSION_VALID_FOR); // The version-valid-for number
    rootpage.put_bytes(&SQLITE_VERSION); // SQLITE_VERSION_NUMBER
    rootpage.put_u8(TABLE_LEAF_PAGE); // leaf table b-tree page for schema
    rootpage.put_u16(NO_FREE_BLOCKS); // zero if there are no freeblocks
    rootpage.put_u16(1); // the number of cells on this page
}

pub struct SchemaRecord {
    pub rowid: u64,
    pub table_name: String,
    pub root_page: u32,
    pub sql: String,
}

impl Default for SchemaRecord {
    fn default() -> Self {
        Self {
            rowid: 0,
            table_name: "".to_owned(),
            root_page: 3,
            sql: "".to_owned(),
        }
    }
}

impl SchemaRecord {
    pub fn new(rowid: u64, table_name: &str, root_page: u32, sql: &str) -> Self {
        Self {
            rowid,
            table_name: table_name.to_owned(),
            root_page,
            sql: sql.to_owned(),
        }
    }
}

const MAGIC_HEADER: [u8; 16] = [
    0x53, 0x51, 0x4c, 0x69, 0x74, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x33, 0x00,
];
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
const FILLER: [u8; 20] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];
const VERSION_VALID_FOR: [u8; 4] = [0, 0, 0x03, 250];
const SQLITE_VERSION: [u8; 4] = [0x00, 0x2e, 0x5F, 0x1A];
const NO_FREE_BLOCKS: u16 = 0;
pub const TABLE_LEAF_PAGE: u8 = 0x0d;
pub const TABLE_INTERIOR_PAGE: u8 = 0x05;
const INDEX_LEAF_PAGE: u8 = 0x0a;
const INDEX_INTERIOR_PAGE: u8 = 0x02;
