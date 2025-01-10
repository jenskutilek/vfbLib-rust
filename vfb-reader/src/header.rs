use crate::buffer;

use serde::Serialize;
use std::io::BufReader;

struct VfbHeaderChunk {
    // TODO: Two-step decompilation here as in entries.
    // Now only the raw data is stored, but the internals of the chunk format are unknown.
    data: Vec<u8>,
}

impl Serialize for VfbHeaderChunk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.data.as_slice().serialize(serializer)
    }
}

#[derive(Serialize)]
pub struct VfbHeaderCreator {
    value0: i32,
    app_version: i32,
    value1: i32,
}

#[derive(Serialize)]
pub struct VfbHeader {
    header0: u8,
    filetype: String,
    header1: u16,
    chunk1: VfbHeaderChunk,
    creator: VfbHeaderCreator,
    end0: u8,
    end1: u8,
    end2: u16,
}

pub fn read<R>(r: &mut BufReader<R>) -> VfbHeader
where
    R: std::io::Read,
{
    let header0 = buffer::read_u8(r);
    let filetype = buffer::read_str(r, 5);
    let header1 = buffer::read_u16(r);
    let chunk1_size: u64 = buffer::read_u16(r).try_into().unwrap();
    let res = buffer::read_bytes(r, chunk1_size);
    let chunk1 = VfbHeaderChunk { data: res };
    let chunk1_usize: usize = chunk1_size.try_into().unwrap();
    let last = chunk1.data.as_slice()[chunk1_usize - 1];
    let last2 = chunk1.data.as_slice()[chunk1_usize - 2];
    let creator: VfbHeaderCreator;
    let end0: u8;
    let end1: u8;
    if [last2, last] == [10, 0] {
        // FL4+ additions over FL3
        let creator_size = buffer::read_u16(r);
        let _creator_bytes = buffer::read_bytes(r, creator_size.try_into().unwrap());
        // TODO:
        // read key, value pairs from creator_bytes until key == 0
        // We need a buffer::read_value that reads from Vec<u8> instead for that
        // For now, set the creator header to a constant:
        creator = VfbHeaderCreator {
            value0: 0,
            app_version: 0x05020280i32,
            value1: 0,
        };
        end0 = buffer::read_u8(r);
        end1 = buffer::read_u8(r);
    } else {
        // Older header format, upgrade it
        creator = VfbHeaderCreator {
            value0: 0,
            app_version: 0x05020280i32,
            value1: 0,
        };
        end0 = 6;
        end1 = 1;
    }
    let end2 = buffer::read_u16(r);

    return VfbHeader {
        header0,
        filetype,
        header1,
        chunk1,
        creator,
        end0,
        end1,
        end2,
    };
}
