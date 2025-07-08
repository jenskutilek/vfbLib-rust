use crate::{buffer, error::VfbError};

use serde::Serialize;
use std::{collections::HashMap, io::BufReader};

struct Chunk {
    // We only store the raw data here for now, because the internals of the chunk
    // format are unknown.
    data: Vec<u8>,
}

impl Serialize for Chunk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.data.as_slice().serialize(serializer)
    }
}

/// The header of a VFB
#[derive(Serialize)]
pub struct Header {
    header0: u8,
    filetype: String,
    header1: u16,
    chunk1: Chunk,
    creator: HashMap<u8, i32>,
    end0: u8,
    end1: u8,
    end2: u16,
}

/// Read the header from the buffer and return it as a struct
pub fn read<R>(r: &mut BufReader<R>) -> Result<Header, VfbError>
where
    R: std::io::Read,
{
    let header0 = buffer::read_u8(r)?;
    let filetype = buffer::read_str(r, 5)?;
    let header1 = buffer::read_u16(r)?;
    let chunk1_size: u64 = buffer::read_u16(r)?.into();
    let res = buffer::read_bytes(r, chunk1_size)?;
    let chunk1 = Chunk { data: res };
    let chunk1_usize: usize = chunk1_size.try_into().unwrap();
    let last = chunk1.data.as_slice()[chunk1_usize - 1];
    let last2 = chunk1.data.as_slice()[chunk1_usize - 2];
    let creator: HashMap<u8, i32>;
    let end0: u8;
    let end1: u8;
    if [last2, last] == [10, 0] {
        // FL4+ additions over FL3

        // The size of the creator chunk is specified in the header, but it contains a
        // key-value map that is terminated by a null byte. So it seems overspecified,
        // and we don't really need the `creator_size`.
        let _creator_size = buffer::read_u16(r);

        // We could read `creator_size` bytes here, but then `buffer::read_key_value_map()`
        // would need to work on bytes, not on the buffered reader as it does now:
        // let _creator_bytes = buffer::read_bytes(r, creator_size.try_into().unwrap());

        // So we ignore all this and just read the key-value map directly from the buffer
        // which terminates at the null byte key:
        creator = buffer::read_key_value_map(r)?;

        // Two more u8 fields follow:
        end0 = buffer::read_u8(r)?;
        end1 = buffer::read_u8(r)?;
    } else {
        // Older header format, upgrade it. We use a custom version, 5.3.0.1, here.
        creator = HashMap::from([(1, 1), (2, 0x05030001), (3, 0)]);
        end0 = 6;
        end1 = 1;
    }
    // And the final u16 of the header:
    let end2 = buffer::read_u16(r)?;

    Ok(Header {
        header0,
        filetype,
        header1,
        chunk1,
        creator,
        end0,
        end1,
        end2,
    })
}
