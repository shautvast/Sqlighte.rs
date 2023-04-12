# Sqlighte.Rs

https://crates.io/crates/sqlighters

**Sqlighter**
* Inspired by a new feature in .Net blazor (see https://www.youtube.com/watch?v=lP_qdhAHFlg&t=300s)
* Creates a SQLite database file from any tabular data.
* So instead of a rest api serving json, enables binary download of data in native SQLite format
* So that SQLite in running in WASM (so in the browser) can be used to query the data.

**Why not use the official SQLite library serverside for this purpose?**

*excellent question!*

But, then I would have to first create and SQLite database and fill it with results and then load the database file and serve it for http requests. While this should also work, it sounds as more overhead. In Sqlighter the data stays in memory. (yes, that's a problem if the data gets reallly BIG; considering offload to file)

**Usable when:**
* you have quite a lot of (tabular) data, that is read-only, or does not need to be (ultra) realtime.
* and your users need to quickly apply different search criteria on it.
* Using Sqlighter avoids server roundtrips and improves the user experience.
* Bear in mind that, while you, as a developer, cannot directly read the payload, like JSON allows, SQLite is available on pretty much any platform,
  and then you can leverage the power of SQL to inspect the data.

* Thing to note: Sqlite is really relaxed when it comes to schema validation.
  That means that 2 records in the same table can contain values of totally different types(!). The number of values can also vary. All perfectly legal from the standpoint of Sqlighter.
  And maybe not when writing to Sqlite itself, but perfectly readable!

**About the name**
* It lights up an SQLite database :)


Creating a database is as simple as: 
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
        write_sqlite(database, writer)?;
        Ok(())
    }
```

**Be aware**
* The schema and the actual data don't have to match! But that is how SQLite itself also works, pretty much.
* And: 2 records in the same table can contain values of totally different types(!). The number of values can also vary. All perfectly legal from the standpoint of Sqlighter.
  And maybe not when writing to SQLite itself (using sql), but perfectly readable from the file.
   


**Current status**
* It works for tables of any size. Indexes are not supported, but you can always add them client-side.
