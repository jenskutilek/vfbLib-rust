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
    }
}
