/// varints as implemented in `SQLite`
pub fn write(value: u64) -> Vec<u8> {
    let mut v = value;
    if (v & ((0xff00_0000) << 32)) == 0 {
        if v == 0 {
            return vec![0];
        }
        let mut result = Vec::new();
        while v != 0 {
            result.push(((v & 0x7f) | 0x80) as u8);
            v >>= 7;
        }
        result[0] &= 0x7f;

        result.reverse();
        result
    } else {
        let mut result = vec![0_u8; 9];
        result[8] = v as u8;
        v >>= 8;
        for i in (0..=7).rev() {
            result[i] = ((v & 0x7f) | 0x80) as u8;
            v >>= 7;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
            write(0xffffffffffffffff)
        );
    }

    #[test]
    fn test_write1() {
        assert_eq!(vec![1], write(0x01));
    }

    #[test]
    fn test_write0() {
        assert_eq!(vec![0], write(0));
    }
}
