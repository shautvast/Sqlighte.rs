#![allow(dead_code)]

mod builder;
mod database;
mod page;
mod record;
mod values;
mod varint;

#[cfg(test)]
mod tests {
    use crate::builder::DatabaseBuilder;
    use crate::database::{write_sqlite, Database};
    use crate::record::Record;
    use crate::values;
    use std::fs::File;
    use std::io::{BufWriter, Error};

    #[test]
    fn test_build() -> Result<(), Error> {
        let mut builder = DatabaseBuilder::new();
        builder.schema("foo", "create table foo(bar varchar(10))");
        for i in 0..10000 {
            let mut record = Record::new(i);
            record.add_value(values::string("helloworld"));
            builder.add_record(record);
        }
        let database: Database = builder.into();
        let file = File::create("foo.db")?;
        let writer = BufWriter::new(file);
        write_sqlite(database, writer)?;
        Ok(())
    }
}
