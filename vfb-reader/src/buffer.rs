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

// TODO: We don't actually need this to read from a buffer, but from Vec<u8>
pub fn read_value<R>(r: &mut BufReader<R>) -> i32
where
    R: std::io::Read,
{
    let result: i32;
    let val = read_u8(r);
    if val < 0x20 {
        return 0; // FIXME: Raise ValueError
    } else if val < 0xF7 {
        // -107 to 107, represented by 1 byte
        result = (val - 0x8B).into();
    } else if val < 0xFF {
        // read a second byte
        let val2 = read_u8(r);
        if val < 0xFB {
            // 108 to 1131, represented by 2 bytes
            result = (val - 0x8B + (val - 0xF7) * 0xFF + val2).into();
        } else {
            // -108 to -1131, represented by 2 bytes
            result = (0x8F - val - (val - 0xFB) * 0xFF - val2).into();
        }
    } else if val == 0xFF {
        // 4-byte big-endian integer follows
        let mut value2 = [0u8; std::mem::size_of::<i32>()];
        r.read_exact(&mut value2).expect("ValueError");
        result = i32::from_be_bytes(value2);
    } else {
        // Can't happen
        result = 0; // FIXME: Raise ValueError
    }
    // println!("Raw: {}, result: {}", val, result);
    return result;
}
