use byteorder::{BigEndian, ByteOrder};
use crate::varint;

pub enum Value {
    String(String),
    Blob(Vec<u8>),
    Integer(i64),
    Float(f64),
}

/// returns (datatype, value)
pub fn get_bytes(value: Value) -> (Vec<u8>, Vec<u8>) {
    match value {
        Value::String(value) => {
            let bytes = value.chars().map(|c| c as u8).collect::<Vec<_>>();
            (varint::write((bytes.len() * 2 + 13) as u64), bytes)
        }
        Value::Blob(value) => {
            (varint::write((value.len() * 2 + 12) as u64), value)
        }
        Value::Integer(value) => {
            (get_int_type(value), integer_to_bytes(value))
        }
        Value::Float(value) => {
            let mut buffer = [0 as u8; 8];
            BigEndian::write_f64(&mut buffer, value);
            (vec![7], buffer.to_vec())
        }
    }
}

/// returns a variable length Vec of u8
fn integer_to_bytes(value: i64) -> Vec<u8> {
    if value == 0 || value == 1 {
        vec![]
    } else {
        return long_to_bytes(value, get_length_of_byte_encoding(value));
    }
}

fn long_to_bytes(n: i64, nbytes: u8) -> Vec<u8> {
    let mut bytes = vec![];
    for i in 0..nbytes {
        bytes.push(((n >> (nbytes - i - 1) * 8) & 0xFF) as u8);
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
            varint::write(length as u64)
        } else if length < 7 {
            varint::write(5)
        } else {
            varint::write(5)
        }
    }
}

fn get_length_of_byte_encoding(value: i64) -> u8 {
    let u =
        if value < 0 {
            !value
        } else {
            value
        };
    if u <= 127 {
        1
    } else if u <= 32767 {
        2
    } else if u <= 8388607 {
        3
    } else if u <= 2147483647 {
        4
    } else if u <= 140737488355327 {
        6
    } else {
        8
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use crate::values::{get_bytes, Value};

    #[test]
    fn test_string() {
        let v = Value::String("hello".to_owned());
        let byte_rep = get_bytes(v);
        assert_eq!(byte_rep.0, vec![23]);
        assert_eq!(byte_rep.1, vec![0x68, 0x65, 0x6C, 0x6C, 0x6F]);
    }

    #[test]
    fn test_blob() {
        let v = Value::Blob(vec![1, 2, 3, 4, 5]);
        let byte_rep = get_bytes(v);
        assert_eq!(byte_rep.0, vec![22]);
        assert_eq!(byte_rep.1, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_float() {
        let v = Value::Float(1.1);
        let byte_rep = get_bytes(v);
        assert_eq!(byte_rep.0, vec![7]);
        assert_eq!(byte_rep.1, vec![0x3f, 0xf1, 0x99, 0x99, 0x99, 0x99, 0x99, 0x9a]);
    }

    #[test]
    fn test_integer0() {
        let v = Value::Integer(0);
        let byte_rep = get_bytes(v);
        assert_eq!(byte_rep.0, vec![8]);
        assert_eq!(byte_rep.1, vec![]);
    }

    #[test]
    fn test_integer1() {
        let v = Value::Integer(1);
        let byte_rep = get_bytes(v);
        assert_eq!(byte_rep.0, vec![9]);
        assert_eq!(byte_rep.1, vec![]);
    }

    #[test]
    fn test_integer2() {
        let v = Value::Integer(2);
        let byte_rep = get_bytes(v);
        assert_eq!(byte_rep.0, vec![1]);
        assert_eq!(byte_rep.1, vec![2]);
    }

    #[test]
    fn test_integer128() {
        let v = Value::Integer(128);
        let byte_rep = get_bytes(v);
        assert_eq!(byte_rep.0, vec![2]);
        assert_eq!(byte_rep.1, vec![0, 128]);
    }
}