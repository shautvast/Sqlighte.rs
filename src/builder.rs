use crate::database::SchemaRecord;
use crate::page::{self, Page};
use crate::record::Record;
use std::mem;

pub struct DatabaseBuilder {
    pub current_page: Page,
    pub n_records_on_current_page: u16,
    pub leaf_pages: Vec<Page>,
    pub schema: Option<SchemaRecord>,
}

fn new_page() -> Page {
    let mut page = Page::new_leaf();
    page.fw_position = 8;
    page
}

impl DatabaseBuilder {
    pub fn new() -> Self {
        Self {
            current_page: new_page(),
            n_records_on_current_page: 0,
            leaf_pages: Vec::new(),
            schema: None,
        }
    }

    pub fn add_record(&mut self, record: Record) {
        if self.current_page_is_full(&record) {
            self.finish_current_page();
            self.leaf_pages
                .push(mem::replace(&mut self.current_page, new_page()));
            self.n_records_on_current_page = 0;
        }

        self.current_page.key = record.rowid; //clone?
        let bytes: Vec<u8> = record.into();
        self.current_page.put_bytes_bw(&bytes);
        self.current_page
            .put_u16(self.current_page.bw_position as u16);
        self.n_records_on_current_page += 1;
    }

    pub fn schema(&mut self, table_name: &str, sql: &str) {
        self.schema = Some(SchemaRecord::new(1, table_name, 2, sql));
    }

    fn current_page_is_full(&self, record: &Record) -> bool {
        self.current_page.bw_position - record.bytes_len() <= self.current_page.fw_position + 5
    }

    fn finish_current_page(&mut self) {
        self.current_page.fw_position = page::POSITION_CELL_COUNT;
        self.current_page.put_u16(self.n_records_on_current_page);
        self.current_page.put_u16(self.current_page.bw_position);
    }
}
