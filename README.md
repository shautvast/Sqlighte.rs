# Sqlighte.Rs

* rust version of https://gitlab.com/sander-hautvast/sqlighter
* still work in progress

Creating a database will be as simple as: 
```rust
fn test_build() -> Result<(), Error> {
        let mut builder = Builder::new();
        builder.schema(
            "foo",
            "create table foo(bar varchar(10))",
        );
        let mut record = Record::new(1);
        record.add_value(values::string("helloworld"));
        builder.add_record(record);

        let database: Database = builder.into();
        let file = File::create("foo.db")?;
        let writer = BufWriter::new(file);
        write(database, writer)?;
        Ok(())
    }
```