use std::mem;
use crate::database::{Database, SchemaRecord};
use crate::page::{self, Page};
use crate::record::Record;

struct Builder {
    current_page: Page,
    n_records_on_current_page: u16,
    leaf_pages: Vec<Page>,
    schema: Option<SchemaRecord>,
}


impl Builder {
    pub fn new() -> Self {
        Self {
            current_page: Page::new_leaf(),
            n_records_on_current_page: 0,
            leaf_pages: Vec::new(),
            schema: None,
        }
    }

    pub fn add_record(&mut self, record: Record) {
        if self.current_page_is_full(&record) {
            self.finish_current_page();
            self.leaf_pages.push(mem::replace(&mut self.current_page, Page::new_leaf()));
            self.n_records_on_current_page = 0;
        }

        self.current_page.key = record.rowid; //clone?
        self.current_page.put_vec_u8_bw(record.to_bytes());
        self.current_page.put_u16(self.current_page.get_bw_position() as u16);
        self.n_records_on_current_page += 1;
    }

    pub fn schema(&mut self, schema: SchemaRecord) {
        self.schema = Some(schema);
    }

    pub fn build(mut self) -> Database {
        self.current_page.set_fw_position(page::POSITION_CELL_COUNT);
        self.current_page.put_u16(self.n_records_on_current_page);

        if self.n_records_on_current_page > 0 {
            self.current_page.put_u16(self.current_page.get_bw_position());
        } else {
            self.current_page.put_u16(self.current_page.get_bw_position() - 1);
        }

        Database::new(self.schema.unwrap(), self.leaf_pages) //panics is schema is not set
    }

    fn current_page_is_full(&self, record: &Record) -> bool {
        self.current_page.get_bw_position() - record.get_length() <= self.current_page.get_fw_position() + 5
    }

    fn finish_current_page(&mut self) {
        self.current_page.set_fw_position(page::POSITION_CELL_COUNT);
        self.current_page.put_u16(self.n_records_on_current_page);
        self.current_page.put_u16(self.current_page.get_bw_position());
    }
}

