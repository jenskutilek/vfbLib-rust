use hex;
use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct VfbHeaderReserved {
    data: [u8; 34],
}

impl Serialize for VfbHeaderReserved {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.data.as_slice().serialize(serializer)
    }
}

#[derive(Serialize)]
struct VfbHeader {
    header00: u8,
    filetype: [u8; 5],
    header01: u16,
    header02: u16,
    reserved: VfbHeaderReserved,
    header03: u16,
    header04: u16,
    header05: u16,
    header06: u16,
    header07: u16,
    header08: u16,
    header09: (u8, i32),
    header10: (u8, i32),
    header11: (u8, i32),
    header12: u8,
    header13: u16,
    header14: u16,
}

#[derive(Serialize)]
struct VfbEntry {
    // VfbEntry<'a>
    key: u16,
    #[serde(skip_serializing)]
    offset: u64,
    size: u32,
    // bytes: Vec<u8>,
    // bytes: &'a Vec<u8>,
    data: String,
}

#[derive(Serialize)]
struct VfbObject<'a> {
    header: &'a VfbHeader,
    entries: Vec<VfbEntry>,
}

// internal functions

fn read_value<R>(r: &mut BufReader<R>) -> i32
where
    R: std::io::Read,
{
    let result: i32;
    let mut value = [0u8; std::mem::size_of::<u8>()];
    r.read_exact(&mut value).expect("ValueError");
    let val = value[0];
    if val < 0x20 {
        return 0; // FIXME: Raise ValueError
    } else if val < 0xF7 {
        // -107 to 107, represented by 1 byte
        result = (val - 0x8B).into();
    } else if val < 0xFF {
        // read a second byte
        let mut value2 = [0u8; std::mem::size_of::<u8>()];
        r.read_exact(&mut value2).expect("ValueError");
        let val2 = value2[0];
        if val < 0xFB {
            // 108 to 1131, represented by 2 bytes
            result = (val - 0x8B + (val - 0xF7) * 0xFF + val2).into();
        } else {
            // -108 to -1131, represented by 2 bytes
            result = (0x8F - val - (val - 0xFB) * 0xFF - val2).into();
        }
    } else if val == 0xFF {
        // 4-byte integer follows
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

/// Read a VfbEntry from the stream and return it
fn read_entry<R>(r: &mut BufReader<R>) -> VfbEntry
where
    R: std::io::Read,
    R: Seek,
{
    let offset: u64 = r.stream_position().expect("Could not read from stream");
    // println!("Reading entry at offset {:#?}", offset);
    let raw_key = read_u16(r);
    let key = raw_key & !0x8000;

    let size: u32;
    if raw_key & 0x8000 > 0 {
        size = read_u32(r);
    } else {
        size = read_u16(r).into();
    }
    let mut bytes: Vec<u8> = vec![0u8; size.try_into().unwrap()];
    r.read_exact(&mut bytes).expect("ValueError");
    // println!(
    //     "Entry: {:#?} at offset {:#?}, {:#?} bytes",
    //     key, offset, size
    // );
    // println!("    {:#?}", bytes);

    return VfbEntry {
        key,
        offset,
        size,
        // bytes,
        data: hex::encode(bytes),
    };
}

fn read_header<R>(r: &mut BufReader<R>) -> VfbHeader
where
    R: std::io::Read,
{
    let header00 = read_u8(r); // 26
    let mut filetype = [0u8; 5]; // WLF10
    r.read_exact(&mut filetype).expect("ValueError");
    let header01 = read_u16(r); // 3
    let header02 = read_u16(r); // 44
    let mut res = [0u8; 34]; // 0000000000000000000000000000000000
    r.read_exact(&mut res).expect("ValueError");
    let reserved = VfbHeaderReserved { data: res };
    let header03 = read_u16(r); // 1
    let header04 = read_u16(r); // 0
    let header05 = read_u16(r); // 4
    let header06 = read_u16(r); // 0
    let mut header07 = read_u16(r); // 10
    let header08: u16;
    let header09: (u8, i32);
    let header10: (u8, i32);
    let header11: (u8, i32);
    let header12: u8;
    if header07 == 10 {
        // FLS5 additions over the FLS2 format
        header08 = read_u16(r);
        header09 = (read_u8(r), read_value(r));
        header10 = (read_u8(r), read_value(r));
        header11 = (read_u8(r), read_value(r));
        header12 = read_u8(r); // or stop byte?
    } else {
        // Upgrade the header to FLS5
        header07 = 10;
        header08 = 0;
        header09 = (1u8, 1i32);
        header10 = (2u8, 0x05020280i32);
        header11 = (3u8, 0i32);
        header12 = 0;
    }
    let header13 = read_u16(r);
    let header14 = read_u16(r);

    return VfbHeader {
        header00,
        filetype,
        header01,
        header02,
        reserved,
        header03,
        header04,
        header05,
        header06,
        header07,
        header08,
        header09,
        header10,
        header11,
        header12,
        header13,
        header14,
    };
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

/// Read a u8 value from a buffer
fn read_u8<R>(r: &mut BufReader<R>) -> u8
where
    R: std::io::Read,
{
    let mut buf = [0u8; 1];
    r.read_exact(&mut buf).expect("ValueError");
    return buf[0];
}

/// Read a u16 value from a buffer
fn read_u16<R>(r: &mut BufReader<R>) -> u16
where
    R: std::io::Read,
{
    let mut buf = [0u8; 2];
    r.read_exact(&mut buf).expect("ValueError");
    return u16::from_le_bytes(buf);
}

/// Read a u32 value from a buffer
fn read_u32<R>(r: &mut BufReader<R>) -> u32
where
    R: std::io::Read,
{
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf).expect("ValueError");
    return u32::from_le_bytes(buf);
}

pub fn read_vfb(path: &str) {
    let file = File::open(path).expect("Failed to open file");
    let mut r = BufReader::new(file);
    let header: VfbHeader;
    header = read_header(&mut r);
    let mut vfb = VfbObject {
        header: &header,
        entries: Vec::new(),
    };
    let mut entry: VfbEntry;
    loop {
        entry = read_entry(&mut r);
        let key = entry.key;
        if key == 5 {
            // End of file
            break;
        }
        vfb.entries.push(entry);
    }
    let json = serde_json::to_string_pretty(&vfb).expect("Serialization failed");
    println!("JSON: {}", json);
    // return vfb;
}
