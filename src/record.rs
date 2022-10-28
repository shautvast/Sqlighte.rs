use crate::bytebuffer::ByteBuffer;
use crate::values::*;
use crate::varint;

struct Record {
    rowid: u64,
    //or should it be i64??
    values: Vec<Value>,
}

impl Record {
    fn new(rowid: u64) -> Self {
        Self {
            rowid,
            values: vec![],
        }
    }

    fn add_value(&mut self, value: Value) {
        self.values.push(value);
    }

    fn to_bytes(&self) -> Vec<u8> {
        let record_length: usize = self.values.iter()
            .map(|v| v.get_length())
            .sum();
        let length_bytes = varint::write(record_length as u64);
        let rowid_bytes = varint::write(self.rowid);

        let mut buffer = ByteBuffer::new(length_bytes.len() + rowid_bytes.len() + record_length);
        buffer.put_u8v(&length_bytes);
        buffer.put_u8v(&rowid_bytes);

        // 'The initial portion of the payload that does not spill to overflow pages.'
        let length_of_encoded_column_types: usize = self.values.iter()
            .map(|v| v.datatype.len())
            .sum();
        buffer.put_u8v(&varint::write((length_of_encoded_column_types + 1) as u64));

        //write all types
        for v in self.values.iter() {
            buffer.put_u8v(&v.datatype)
        }

        //  write all values
        for v in self.values.iter() {
            buffer.put_u8v(&v.data) //copies individual bytes into a buffer...should I avoid copying?
        }
        buffer.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut record = Record::new(1);
        record.add_value(string("hello"));
    }
}