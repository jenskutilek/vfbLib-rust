use encoding_rs::WINDOWS_1252;
use error_stack::{Report, ResultExt};
use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
};

use crate::{error::VfbError, VfbEntry};

const VFB_UNICODE_STRINGS: bool = false;

pub struct VfbReader<R: std::io::Read + std::io::Seek> {
    reader: BufReader<R>,
    pub(crate) number_of_masters: usize,
}

pub struct EntryReader<'a, R: std::io::Read + std::io::Seek> {
    pub(crate) inner: std::io::Take<&'a mut BufReader<R>>,
    pub(crate) number_of_masters: usize,
    base_position: u64,
}

impl<R: std::io::Read + std::io::Seek> EntryReader<'_, R> {
    pub fn new(reader: &'_ mut VfbReader<R>, size: u64) -> EntryReader<'_, R> {
        let number_of_masters = reader.number_of_masters;
        let base_position = reader.stream_position().unwrap_or(0);
        EntryReader {
            inner: reader.reader().take(size),
            number_of_masters,
            base_position,
        }
    }
}

pub(crate) trait ReadExt {
    fn reader(&mut self) -> &mut dyn Read;
    fn stream_position(&mut self) -> Result<u64, std::io::Error>;
    fn read_i32(&mut self) -> Result<i32, Report<VfbError>> {
        let mut buf = [0u8; 4];
        self.reader()
            .read_exact(&mut buf)
            .map_err(VfbError::ReadError)?;
        Ok(i32::from_le_bytes(buf))
    }

    fn read_u16(&mut self) -> Result<u16, Report<VfbError>> {
        let mut buf = [0u8; 2];
        self.reader()
            .read_exact(&mut buf)
            .map_err(VfbError::ReadError)?;
        Ok(u16::from_le_bytes(buf))
    }

    /// Read an f64 value from a buffer
    fn read_f64(&mut self) -> Result<f64, Report<VfbError>> {
        let mut buf = [0u8; 8];
        self.reader()
            .read_exact(&mut buf)
            .map_err(VfbError::ReadError)?;
        Ok(f64::from_le_bytes(buf))
    }

    /// Read a i16 value from a buffer
    fn read_i16(&mut self) -> Result<i16, Report<VfbError>> {
        let mut buf = [0u8; 2];
        self.reader()
            .read_exact(&mut buf)
            .map_err(VfbError::ReadError)?;
        Ok(i16::from_le_bytes(buf))
    }

    /// Read a u8 value from a buffer
    fn read_u8(&mut self) -> Result<u8, Report<VfbError>> {
        let mut buf = [0u8; 1];
        self.reader()
            .read_exact(&mut buf)
            .map_err(VfbError::ReadError)?;
        Ok(buf[0])
    }

    /// Read a u32 value from a buffer
    fn read_u32(&mut self) -> Result<u32, Report<VfbError>> {
        let mut buf = [0u8; 4];
        self.reader()
            .read_exact(&mut buf)
            .map_err(VfbError::ReadError)?;
        Ok(u32::from_le_bytes(buf))
    }

    /// Read a u64 value from a buffer
    fn read_u64(&mut self) -> Result<u64, Report<VfbError>> {
        let mut buf = [0u8; 8];
        self.reader()
            .read_exact(&mut buf)
            .map_err(VfbError::ReadError)?;
        Ok(u64::from_le_bytes(buf))
    }

    /// Read the specified number of bytes from a buffer and return them as a string
    fn read_str(&mut self, bytes_to_read: u64) -> Result<String, Report<VfbError>> {
        let buf = self.read_bytes(bytes_to_read)?;

        if VFB_UNICODE_STRINGS {
            let s = std::str::from_utf8(&buf).map_err(VfbError::InvalidUtf8)?;
            Ok(s.to_string())
        } else {
            let (s, _, _) = WINDOWS_1252.decode(&buf);
            Ok(s.to_string())
        }
    }

    /// Read the length of a string from a buffer and then read the string
    fn read_str_with_len(&mut self) -> Result<String, Report<VfbError>> {
        let len = self.read_value()?;
        let buf = self.read_bytes(len as u64)?;
        if VFB_UNICODE_STRINGS {
            let s = std::str::from_utf8(&buf).map_err(VfbError::InvalidUtf8)?;
            Ok(s.to_string())
        } else {
            let (s, _, _) = WINDOWS_1252.decode(&buf);
            Ok(s.to_string())
        }
    }

    /// Read the remaining bytes from a buffer and return them as a string
    fn read_str_remainder(&mut self) -> Result<String, Report<VfbError>> {
        let buf = self.read_bytes_remainder()?;
        if VFB_UNICODE_STRINGS {
            let s = std::str::from_utf8(&buf).map_err(VfbError::InvalidUtf8)?;
            Ok(s.to_string())
        } else {
            let (s, _, _) = WINDOWS_1252.decode(&buf);
            Ok(s.to_string())
        }
    }

    // /// Read a i64 value from a buffer
    // pub fn read_i64(&mut self) -> Result<i64, Report<VfbError>> {
    //     let mut buf = [0u8; 8];
    //     self.reader().read_exact(&mut buf)?;
    //     Ok(i64::from_le_bytes(buf))
    // }

    /// Read an "encoded value" from a buffer
    ///
    /// Lifted from the Type 1 font spec:
    /// https://adobe-type-tools.github.io/font-tech-notes/pdfs/T1_SPEC.pdf
    /// Page 48, 6.2 Charstring Number Encoding
    fn read_value(&mut self) -> Result<i32, Report<VfbError>> {
        // A charstring byte containing the values from 32 through 255 inclusive indicates
        // an integer. These values are decoded in four ranges.
        let v: i32 = self.read_u8()?.into();
        if v < 32 {
            let report: Report<VfbError> = VfbError::BadValue(
                format!("Invalid charstring byte value: {}", v),
                "value between 32 and 255".to_string(),
            )
            .into();
            return Err(report.attach_printable(format!(
                "at byte index {}",
                self.stream_position().unwrap_or(0) - 1
            )));
        } else if v <= 246 {
            // A charstring byte containing a value, v, between 32 and 246 inclusive,
            // indicates the integer v − 139. Thus, the integer values from −107 through 107
            // inclusive may be encoded in a single byte.
            return Ok(v - 139);
        } else if v <= 254 {
            // read a second byte
            let w: i32 = self.read_u8()?.into();
            if v <= 250 {
                // A charstring byte containing a value, v, between 247 and 250 inclusive,
                // indicates an integer involving the next byte, w, according to the
                // formula: [(v − 247) × 256] + w + 108 Thus, the integer values between 108
                // and 1131 inclusive can be encoded in 2 bytes in this manner.
                return Ok((v - 247) * 256 + w + 108);
            } else {
                // A charstring byte containing a value, v, between 251 and 254 inclusive,
                // indicates an integer involving the next byte, w, according to the
                // formula: − [(v − 251) × 256] − w − 108 Thus, the integer values between
                // −1131 and −108 inclusive can be encoded in 2 bytes in this manner.
                return Ok(-((v - 251) * 256) - w - 108);
            }
        } else if v == 255 {
            // Finally, if the charstring byte contains the value 255, the next four bytes
            // indicate a two’s complement signed integer. The first of these four bytes
            // contains the highest order bits, the second byte contains the next higher
            // order bits and the fourth byte contains the lowest order bits. Thus, any
            // 32-bit signed integer may be encoded in 5 bytes in this manner (the 255 byte
            // plus 4 more bytes).
            let mut transgender = [0u8; std::mem::size_of::<i32>()];
            self.reader()
                .read_exact(&mut transgender)
                .map_err(VfbError::ReadError)?;
            return Ok(i32::from_be_bytes(transgender));
        }
        Ok(v)
    }

    /// A parser that reads data as Yuri's optimized encoded values. The list of values is
    /// preceded by a count value that specifies how many values should be read.
    fn read_encoded_value_list(&mut self) -> Result<Vec<i32>, Report<VfbError>> {
        let count = self.read_value()?;
        let mut values = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let v = self.read_value()?;
            values.push(v);
        }
        Ok(values)
    }

    /// Read the specified number of bytes from a buffer
    fn read_bytes(&mut self, bytes_to_read: u64) -> Result<Vec<u8>, Report<VfbError>> {
        let mut buf = vec![];
        let mut chunk = self.reader().take(bytes_to_read);
        let _ = chunk.read_to_end(&mut buf).map_err(VfbError::ReadError)?;
        Ok(buf)
    }

    /// Read the remaining bytes from a buffer
    fn read_bytes_remainder(&mut self) -> Result<Vec<u8>, Report<VfbError>> {
        let mut buf = vec![];
        let mut chunk = self.reader().take(0xFFFF);
        let _ = chunk.read_to_end(&mut buf);
        Ok(buf)
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
    fn read_key_value_map(&mut self) -> Result<HashMap<u8, i32>, Report<VfbError>> {
        let mut map = HashMap::new();
        let mut k = self.read_u8()?;
        while k != 0 {
            let v = self.read_value()?;
            map.insert(k, v);
            k = self.read_u8()?;
        }
        Ok(map)
    }
}

impl<R: std::io::Read + std::io::Seek> ReadExt for EntryReader<'_, R> {
    fn reader(&mut self) -> &mut dyn Read {
        &mut self.inner
    }

    fn stream_position(&mut self) -> Result<u64, std::io::Error> {
        // Get position within this Take reader and add the base position
        let relative_pos = std::io::Seek::stream_position(&mut self.inner)?;
        Ok(self.base_position + relative_pos)
    }
}

impl<R: std::io::Read + std::io::Seek> ReadExt for VfbReader<R> {
    fn reader(&mut self) -> &mut dyn Read {
        &mut self.reader
    }

    fn stream_position(&mut self) -> Result<u64, std::io::Error> {
        std::io::Seek::stream_position(&mut self.reader)
    }
}

impl<R: std::io::Read + std::io::Seek> VfbReader<R> {
    pub fn new(reader: R) -> Self {
        VfbReader {
            reader: BufReader::new(reader),
            number_of_masters: 1,
        }
    }

    pub(crate) fn reader(&mut self) -> &mut BufReader<R> {
        &mut self.reader
    }

    pub(crate) fn scoped(&mut self, size: u64) -> EntryReader<'_, R> {
        EntryReader::new(self, size)
    }

    /// Read a VfbEntry from the stream and return it along with its key.
    /// Returns (key, Option<VfbEntry>) where None indicates an unknown or empty entry.
    pub fn read_entry(&mut self) -> Result<(u16, Option<VfbEntry>), Report<VfbError>> {
        // Read the key
        let raw_key = self.read_u16()?;
        // The raw key may be masked with 0x8000 to indicate a u32 data size
        let key = raw_key & !0x8000;

        // Read the size
        let size: u32 = if raw_key & 0x8000 > 0 {
            self.read_u32()?
        } else {
            self.read_u16()?.into()
        };

        // Create a new scoped reader
        let mut entry_reader = self.scoped(size as u64);

        // Parse the entry
        let entry = VfbEntry::new_from_reader(key, &mut entry_reader).attach_printable(format!(
            "while reading {}",
            VfbEntry::key_to_variant(key).unwrap_or("an unknown key")
        ))?;

        Ok((key, entry))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::buffer::{ReadExt, VfbReader};
    use std::io::Cursor;

    fn get_reader(bytes: &[u8]) -> VfbReader<Cursor<&[u8]>> {
        VfbReader::new(Cursor::new(bytes))
    }

    #[test]
    fn test_value_1b_0x20() {
        assert_eq!(get_reader(&[0x20]).read_value().unwrap(), -107i32);
    }

    #[test]
    fn test_value_1b_0x8a() {
        assert_eq!(get_reader(&[0x8a]).read_value().unwrap(), -1i32);
    }

    #[test]
    fn test_value_1b_0x8b() {
        assert_eq!(get_reader(&[0x8b]).read_value().unwrap(), 0i32);
    }

    #[test]
    fn test_value_1b_0xf6() {
        assert_eq!(get_reader(&[0xf6]).read_value().unwrap(), 107i32);
    }

    #[test]
    fn test_value_2b_0xf700() {
        assert_eq!(get_reader(&[0xf7, 0x00]).read_value().unwrap(), 108i32);
    }

    #[test]
    fn test_value_2b_0xf701() {
        assert_eq!(get_reader(&[0xf7, 0x01]).read_value().unwrap(), 109i32);
    }

    #[test]
    fn test_value_2b_0xf7ff() {
        assert_eq!(get_reader(&[0xf7, 0xff]).read_value().unwrap(), 363i32);
    }

    #[test]
    fn test_value_2b_0xf800() {
        assert_eq!(get_reader(&[0xf8, 0x00]).read_value().unwrap(), 364i32);
    }

    #[test]
    fn test_value_2b_0xf801() {
        assert_eq!(get_reader(&[0xf8, 0x01]).read_value().unwrap(), 365i32);
    }

    #[test]
    fn test_value_2b_0xfa00() {
        assert_eq!(get_reader(&[0xfa, 0x00]).read_value().unwrap(), 876i32);
    }

    #[test]
    fn test_value_2b_0xfaff() {
        assert_eq!(get_reader(&[0xfa, 0xff]).read_value().unwrap(), 1131i32);
    }

    #[test]
    fn test_value_2b_0xfb00() {
        assert_eq!(get_reader(&[0xfb, 0x00]).read_value().unwrap(), -108i32);
    }

    #[test]
    fn test_value_2b_0xfb01() {
        assert_eq!(get_reader(&[0xfb, 0x01]).read_value().unwrap(), -109i32);
    }

    #[test]
    fn test_value_2b_0xfe00() {
        assert_eq!(get_reader(&[0xfe, 0x00]).read_value().unwrap(), -876i32);
    }

    #[test]
    fn test_value_2b_0xfeff() {
        assert_eq!(get_reader(&[0xfe, 0xff]).read_value().unwrap(), -1131i32);
    }

    #[test]
    fn test_value_5b_0xff00000000() {
        assert_eq!(
            get_reader(&[0xff, 0x00, 0x00, 0x00, 0x00])
                .read_value()
                .unwrap(),
            0i32
        );
    }

    #[test]
    fn test_value_5b_0xff00001000() {
        assert_eq!(
            get_reader(&[0xff, 0x00, 0x00, 0x10, 0x00])
                .read_value()
                .unwrap(),
            4096i32
        );
    }

    #[test]
    fn test_value_5b_0xffffffffff() {
        assert_eq!(
            get_reader(&[0xff, 0xff, 0xff, 0xff, 0xff])
                .read_value()
                .unwrap(),
            -1i32
        );
    }

    #[test]
    fn test_value_5b_0xffffffefff() {
        assert_eq!(
            get_reader(&[0xff, 0xff, 0xff, 0xef, 0xff])
                .read_value()
                .unwrap(),
            -4097i32
        );
    }

    #[test]
    fn test_key_value_map() {
        assert_eq!(
            get_reader(&[
                0x01, 0x8c, 0x02, 0xff, 0x05, 0x00, 0x04, 0x80, 0x03, 0xff, 0x00, 0x00, 0x12, 0x08,
                0x00
            ])
            .read_key_value_map()
            .unwrap(),
            HashMap::from([(1, 1), (2, 0x05000480), (3, 4616)])
        );
    }
}
