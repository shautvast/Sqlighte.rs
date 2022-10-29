use std::mem;
use crate::database::{Database, SchemaRecord};
use crate::page::{self, Page};
use crate::record::Record;

pub struct Builder {
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
        let bytes: Vec<u8> = record.into();
        self.current_page.put_bytes_bw(&bytes);
        self.current_page.put_u16(self.current_page.bw_position as u16);
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

impl Into<Database> for Builder {
    fn into(mut self) -> Database {
        self.current_page.fw_position = page::POSITION_CELL_COUNT;
        self.current_page.put_u16(self.n_records_on_current_page);

        if self.n_records_on_current_page > 0 {
            self.current_page.put_u16(self.current_page.bw_position);
        } else {
            self.current_page.put_u16(self.current_page.bw_position - 1);
        }

        Database::new(self.schema.unwrap(), self.leaf_pages) //panics is schema is not set
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufWriter, Error};
    use crate::database::write;
    use crate::values;
    use super::*;

    #[test]
    fn test_build() -> Result<(), Error>{
        let mut builder = Builder::new();
        builder.schema(
            "foo",
            "create table foo(bar varchar(10))",
        );
        let mut record = Record::new(1);
        record.add_value(values::string("helloworld"));
        builder.add_record(record);

        let database: Database = builder.into();
        let file = File::create("foo.txt")?;
        let writer = BufWriter::new(file);
        write(database, writer)?;
        Ok(())
    }
}