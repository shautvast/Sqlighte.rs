use crate::varint;
use byteorder::{BigEndian, ByteOrder};

pub struct Value {
    pub datatype: Vec<u8>,
    pub data: Vec<u8>,
}

impl Value {
    pub fn len(&self) -> u16 {
        (self.datatype.len() + self.data.len()) as u16
    }
}

pub fn string(value: &str) -> Value {
    let bytes = value.chars().map(|c| c as u8).collect::<Vec<_>>();
    Value {
        datatype: varint::write((bytes.len() * 2 + 13) as u64),
        data: bytes,
    }
}

pub fn blob(value: Vec<u8>) -> Value {
    Value {
        datatype: varint::write((value.len() * 2 + 12) as u64),
        data: value,
    }
}

pub fn integer(value: i64) -> Value {
    Value {
        datatype: get_int_type(value),
        data: sqlite_integer_to_bytes(value),
    }
}

pub fn float(value: f64) -> Value {
    let mut buffer = [0_u8; 8];
    BigEndian::write_f64(&mut buffer, value);
    Value {
        datatype: vec![7],
        data: buffer.to_vec(),
    }
}

pub fn len(value: &Value) -> usize {
    value.datatype.len() + value.data.len()
}

/// sqlite specific way to encode integers
/// returns a variable length Vec of u8
fn sqlite_integer_to_bytes(value: i64) -> Vec<u8> {
    if value == 0 || value == 1 {
        vec![]
    } else {
        i64_to_bytes(value, get_length_of_byte_encoding(value))
    }
}

fn i64_to_bytes(value: i64, len: u8) -> Vec<u8> {
    let mut bytes = vec![];
    for i in 0..len {
        bytes.push(((value >> ((len - i - 1) * 8)) & 0xFF) as u8);
    }

    bytes
}

fn get_int_type(value: i64) -> Vec<u8> {
    if value == 0 {
        vec![8]
    } else if value == 1 {
        vec![9]
    } else {
        let length = get_length_of_byte_encoding(value);
        if length < 5 {
            varint::write(u64::from(length))
        } else if length < 7 {
            varint::write(5)
        } else {
            varint::write(6)
        }
    }
}

fn get_length_of_byte_encoding(value: i64) -> u8 {
    let u = if value < 0 { !value } else { value };
    if u <= 127 {
        1
    } else if u <= 32_767 {
        2
    } else if u <= 8_388_607 {
        3
    } else if u <= 2_147_483_647 {
        4
    } else if u <= 140_737_488_355_327 {
        6
    } else {
        8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let v = string("hello");
        assert_eq!(v.datatype, vec![23]);
        assert_eq!(v.data, vec![0x68, 0x65, 0x6C, 0x6C, 0x6F]);
    }

    #[test]
    fn test_blob() {
        let v = blob(vec![1, 2, 3, 4, 5]);
        assert_eq!(v.datatype, vec![22]);
        assert_eq!(v.data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_float() {
        let v = float(1.1);
        assert_eq!(v.datatype, vec![7]);
        assert_eq!(v.data, vec![0x3f, 0xf1, 0x99, 0x99, 0x99, 0x99, 0x99, 0x9a]);
    }

    #[test]
    fn test_integer0() {
        let v = integer(0);
        assert_eq!(v.datatype, vec![8]);
        assert_eq!(v.data, vec![]);
    }

    #[test]
    fn test_integer1() {
        let v = integer(1);
        assert_eq!(v.datatype, vec![9]);
        assert_eq!(v.data, vec![]);
    }

    #[test]
    fn test_integer2() {
        let v = integer(2);
        assert_eq!(v.datatype, vec![1]);
        assert_eq!(v.data, vec![2]);
    }

    #[test]
    fn test_integer128() {
        let v = integer(128);
        assert_eq!(v.datatype, vec![2]);
        assert_eq!(v.data, vec![0, 128]);
    }
}
