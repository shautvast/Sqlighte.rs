use crate::values::*;

struct Record {
    rowid: i64,
    values: Vec<Value>,
}

impl Record {
    fn new(rowid: i64) -> Self {
        Self {
            rowid,
            values: vec![],
        }
    }

    fn add_value(&mut self, value: Value) {
        self.values.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut record = Record::new(1);
        record.add_value(Value::String("hello".to_owned()));
    }
}