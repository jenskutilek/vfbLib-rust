use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;

/// Read the specified number of bytes from a buffer
pub fn read_bytes<R>(r: &mut BufReader<R>, bytes_to_read: u64) -> Vec<u8>
where
    R: std::io::Read,
{
    let mut buf = vec![];
    let mut chunk = r.take(bytes_to_read);
    let n = chunk.read_to_end(&mut buf).expect("Didn't read enough");
    assert_eq!(bytes_to_read as usize, n);
    return buf;
}

/// Read a key-value map from a buffer. The keys are u8, the values are
/// "encoded values". A key of 0 means the end of the map is reached.
/// 
/// Example:
/// 
/// 01 | 8c
/// 02 | ff 05 00 04 80
/// 03 | ff 00 00 12 08
/// 00
/// The final 0 key is not included in the returned HashMap.
pub fn read_key_value_map<R>(r: &mut BufReader<R>) -> HashMap<u8, i32>
where
    R: std::io::Read,
{
    let mut map = HashMap::new();
    let mut k = read_u8(r);
    while k != 0 {
        let v = read_value(r);
        map.insert(k, v);
        k = read_u8(r);
    }
    return map;
}

// TODO: Do we need this?
/// Read n u8 values from a buffer
// fn read_n_u8<R>(r: &mut BufReader<R>, n: u8) -> u8
// where
//     R: std::io::Read,
// {
//     let mut buf = [0u8; n];
//     r.read_exact(&mut buf).expect("ValueError");
//     return buf[0];
// }

/// Read the specified number of bytes from a buffer and return them as a string
pub fn read_str<R>(r: &mut BufReader<R>, bytes_to_read: u64) -> String
where
    R: std::io::Read,
{
    let buf = read_bytes(r, bytes_to_read);
    let s = match std::str::from_utf8(&buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    return s.to_string();
}

/// Read a u8 value from a buffer
pub fn read_u8<R>(r: &mut BufReader<R>) -> u8
where
    R: std::io::Read,
{
    let mut buf = [0u8; 1];
    r.read_exact(&mut buf).expect("ValueError");
    return buf[0];
}

/// Read a u16 value from a buffer
pub fn read_u16<R>(r: &mut BufReader<R>) -> u16
where
    R: std::io::Read,
{
    let mut buf = [0u8; 2];
    r.read_exact(&mut buf).expect("ValueError");
    return u16::from_le_bytes(buf);
}

/// Read a u32 value from a buffer
pub fn read_u32<R>(r: &mut BufReader<R>) -> u32
where
    R: std::io::Read,
{
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf).expect("ValueError");
    return u32::from_le_bytes(buf);
}

/// Read an "encoded value" from a buffer
pub fn read_value<R>(r: &mut BufReader<R>) -> i32
where
    R: std::io::Read,
{
    let value: i32 = read_u8(r).into();
    if value < 0x20 {
        return 0; // FIXME: Raise ValueError
    } else if value < 0xF7 {
        // -107 to 107, represented by 1 byte
        return value - 0x8B;
    } else if value < 0xFF {
        // read a second byte
        let value2: i32 = read_u8(r).into();
        if value < 0xFB {
            // 108 to 1131, represented by 2 bytes
            return value - 0x8B + (value - 0xF7) * 0xFF + value2;
        } else {
            // -108 to -1131, represented by 2 bytes
            return 0x8F - value - (value - 0xFB) * 0xFF - value2;
        }
    } else if value == 0xFF {
        // 4-byte big-endian integer follows
        let mut value2 = [0u8; std::mem::size_of::<i32>()];
        r.read_exact(&mut value2).expect("ValueError");
        return i32::from_be_bytes(value2);
    }
    return value;
}

#[cfg(test)]
mod tests {
    use crate::buffer::{read_key_value_map, read_value};
    use std::{collections::HashMap, io::BufReader};

    fn get_reader(bytes: &[u8]) -> BufReader<&[u8]> {
        return BufReader::new(&bytes[..]);
    }

    #[test]
    fn test_value_1b_0x20() {
        assert_eq!(read_value(&mut get_reader(&[0x20])), -107i32);
    }

    #[test]
    fn test_value_1b_0x8a() {
        assert_eq!(read_value(&mut get_reader(&[0x8a])), -1i32);
    }

    #[test]
    fn test_value_1b_0x8b() {
        assert_eq!(read_value(&mut get_reader(&[0x8b])), 0i32);
    }

    #[test]
    fn test_value_1b_0xf6() {
        assert_eq!(read_value(&mut get_reader(&[0xf6])), 107i32);
    }

    #[test]
    fn test_value_2b_0xf700() {
        assert_eq!(read_value(&mut get_reader(&[0xf7, 0x00])), 108i32);
    }

    #[test]
    fn test_value_2b_0xf701() {
        assert_eq!(read_value(&mut get_reader(&[0xf7, 0x01])), 109i32);
    }

    #[test]
    fn test_value_2b_0xf7ff() {
        assert_eq!(read_value(&mut get_reader(&[0xf7, 0xff])), 363i32);
    }

    #[test]
    fn test_value_2b_0xf800() {
        assert_eq!(read_value(&mut get_reader(&[0xf8, 0x00])), 364i32);
    }

    #[test]
    fn test_value_2b_0xf801() {
        assert_eq!(read_value(&mut get_reader(&[0xf8, 0x01])), 365i32);
    }

    #[test]
    fn test_value_2b_0xfa00() {
        assert_eq!(read_value(&mut get_reader(&[0xfa, 0x00])), 876i32);
    }

    #[test]
    fn test_value_2b_0xfaff() {
        assert_eq!(read_value(&mut get_reader(&[0xfa, 0xff])), 1131i32);
    }

    #[test]
    fn test_value_2b_0xfb00() {
        assert_eq!(read_value(&mut get_reader(&[0xfb, 0x00])), -108i32);
    }

    #[test]
    fn test_value_2b_0xfb01() {
        assert_eq!(read_value(&mut get_reader(&[0xfb, 0x01])), -109i32);
    }

    #[test]
    fn test_value_2b_0xfe00() {
        assert_eq!(read_value(&mut get_reader(&[0xfe, 0x00])), -876i32);
    }

    #[test]
    fn test_value_2b_0xfeff() {
        assert_eq!(read_value(&mut get_reader(&[0xfe, 0xff])), -1131i32);
    }

    #[test]
    fn test_value_5b_0xff00000000() {
        assert_eq!(
            read_value(&mut get_reader(&[0xff, 0x00, 0x00, 0x00, 0x00])),
            0i32
        );
    }

    #[test]
    fn test_value_5b_0xff00001000() {
        assert_eq!(
            read_value(&mut get_reader(&[0xff, 0x00, 0x00, 0x10, 0x00])),
            4096i32
        );
    }

    #[test]
    fn test_value_5b_0xffffffffff() {
        assert_eq!(
            read_value(&mut get_reader(&[0xff, 0xff, 0xff, 0xff, 0xff])),
            -1i32
        );
    }

    #[test]
    fn test_value_5b_0xffffffefff() {
        assert_eq!(
            read_value(&mut get_reader(&[0xff, 0xff, 0xff, 0xef, 0xff])),
            -4097i32
        );
    }

    #[test]
    fn test_key_value_map() {
        assert_eq!(
            read_key_value_map(&mut get_reader(&[
                0x01, 0x8c, 0x02, 0xff, 0x05, 0x00, 0x04, 0x80, 0x03, 0xff, 0x00, 0x00, 0x12, 0x08,
                0x00
            ])),
            HashMap::from([(1, 1), (2, 0x05000480), (3, 4616)])
        );
    }
}
