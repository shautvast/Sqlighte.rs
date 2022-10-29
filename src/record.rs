use crate::values::*;
use crate::varint;

pub struct Record {
    pub rowid: u64,
    //or should it be i64??
    values: Vec<Value>,
}

impl Record {
    pub fn new(rowid: u64) -> Self {
        Self {
            rowid,
            values: vec![],
        }
    }

    pub fn add_value(&mut self, value: Value) {
        self.values.push(value);
    }

    /// length of the byte representation
    pub fn bytes_len(&self) -> u16 {
        let record_length: u16 = self.values.iter()
            .map(|v| v.len())
            .sum();
        record_length
    }
}

impl Into<Vec<u8>> for Record{
    fn into(mut self) -> Vec<u8> {
        let record_length = self.bytes_len();
        let mut length_bytes = varint::write(record_length as u64);
        let mut rowid_bytes = varint::write(self.rowid);

        let mut buffer = Vec::with_capacity(length_bytes.len() + rowid_bytes.len() + record_length as usize);
        buffer.append(&mut length_bytes);
        buffer.append(&mut rowid_bytes);

        // 'The initial portion of the payload that does not spill to overflow pages.'
        let length_of_encoded_column_types: usize = self.values.iter()
            .map(|v| v.datatype.len())
            .sum();
        buffer.append(&mut varint::write((length_of_encoded_column_types + 1) as u64));

        //write all types
        for v in self.values.iter_mut() {
            buffer.append(&mut v.datatype)
        }

        //  write all values
        for v in self.values.iter_mut() {
            buffer.append(&mut v.data)
        }
        buffer
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