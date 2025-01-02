# The VFB file format

VFB stands for _Vector Format B_. Don’t ask me where I read this specific piece of information.

## “Encoded Value” format

A representation of numerical values that is used a lot in the rest of the file is what I called the “Encoded Value” format. This format is designed to use as few bytes as possible, and is lifted from the Type 1 font spec ([Adobe Type 1 Font Format](https://adobe-type-tools.github.io/font-tech-notes/pdfs/T1_SPEC.pdf): p.48, 6.2 Charstring Number Encoding):

> A charstring byte containing the values from 32 through 255 inclusive indicates an integer. These values are decoded in four ranges.
>
> 1. A charstring byte containing a value, _v,_ between 32 and 246 inclusive, indicates the integer _v_ − 139. Thus, the integer values from −107 through 107 inclusive may be encoded in a single byte.
> 2. A charstring byte containing a value, _v,_ between 247 and 250 inclusive, indicates an integer involving the next byte, _w,_ according to the formula: [(_v_ − 247) × 256] + _w_ + 108
>    Thus, the integer values between 108 and 1131 inclusive can be encoded in 2 bytes in this manner.
> 3. A charstring byte containing a value, _v,_ between 251 and 254 inclusive, indicates an integer involving the next byte, _w,_ according to the formula: − [(_v_ − 251) × 256] − _w_ − 108
>    Thus, the integer values between −1131 and −108 inclusive can be encoded in 2 bytes in this manner.
> 4. Finally, if the charstring byte contains the value 255, the next four bytes indicate a two’s complement signed integer. The first of these four bytes contains the highest order bits, the second byte contains the next higher order bits and the fourth byte contains the lowest order bits. Thus, any 32-bit signed integer may be encoded in 5 bytes in this manner (the 255 byte plus 4 more bytes).

If you look at a VFB with a hex editor, you will see `8B` in many places, which is an encoded 0.

## Header

The file begins with a header that appears to be version-specific (when comparing files saved by FontLab Studio 5 and older versions), but is constant, unless there are settings or data in the application that affect the header which I haven’t discovered so far.

Most multi-byte values in the VFB header are little-endian.

```
1A                  u8      26
57 4C 46 31 30      string  "WLF10"
03 00               u16     3
2C 00               u16     44 (length in bytes of the following chunk)
--------------- start of 44-byte chunk
00 00 00 00                 34 0-bytes
00 00 00 00
00 00 00 00
00 00 00 00
00 00 00 00
00 00 00 00
00 00 00 00
00 00 00 00
00 00
01 00               u16     1
00 00               u16     0
04 00               u16     4
00 00               u16     0
06 01               u16     262
--------------- end of 44-byte chunk
```

If the last u16 of the 44-byte chunk was 0x0601, two more bytes follow, and the header ends:

```
00 00               u16     0
```

If the last u16 of the 44-byte chunk was 0x0a00, the VFB is in the format written by FLS5, and we read and interpret the next values like this instead:

```
0B 00               u16     11 (length in bytes of the following chunk)
--------------- start of 11-byte chunk in "key-value" format
01                  u8      key: 1
8C                  EV      value: 1
02                  u8      key: 2
FF 05 02 02 80      EV      value: 84017792
03                  u8      key: 3
8B                  EV      value: 0
00                  u8      key: 0 (end marker?)
--------------- end of 11-byte chunk
```

After that, those values follow as in the earlier format:

```
06 01               u16     262
00 00
```

## Data entries

After the header, the data is organized in “entries”, which consist of a key, detailing the [meaning](vfb-reader/src/vfb_constants.rs) and data format of the entry, a data length value, and the actual data.

### Entry with a data size greater than 0 and smaller than `u16::MAX` (65535)

```
DD 05               u16     1501  entry key (1501: "Encoding Default")
06 00               u16     6     entry data length
--------------- start of 6-byte chunk (entry-specific format)
34 00               u16     52 (glyph index in encoding)
66 6F 75 72         string  "four" (glyph name in encoding)
--------------- end of 6-byte chunk
```

### Entry with a larger data size

If bit 15 is set in the entry key, it means that the entry data length that follows is a u32 number instead of a u16 number:

```
DD 85               u16     32768 + 1501  entry key (1501: "Encoding Default")
06 00 00 00         u32     6             entry data length
--------------- start of 6-byte chunk (entry-specific format)
34 00               u16     52 (glyph index in encoding)
66 6F 75 72         string  "four" (glyph name in encoding)
--------------- end of 6-byte chunk
```

An entry that often uses a u32 data length is the "features" entry which contains the OpenType feature code in AFDKO syntax, and is larger than 65535 bytes in many fonts.

### Entry without data

It seems to be perfectly OK if an entry has a data size of 0. In that case, no data bytes are present.

```
06 02               u16     518   entry key
00 00               u16     0     entry data length
--------------- next entry
01 01               u16     257   entry key
00 00               u16     0     entry data length
--------------- next entry
02 04               u16     1026  entry key (1026: "font_name")
0C 00               u16     12    entry data length
54 68 65 53         string  TheS
61 6E 73 4D         string  ansM
4D 37 37 33         string  M773
--------------- next entry
```

Speculative, those zero-length entries could serve as a marker for the start or end of a section of the file. E.g. the header ended with `06 01 00 00`, and the encodings ended with `06 02 00 00`.

## Entry order

As there are some aspects of a font source file that are represented by multiple entries in the VFB file, (e.g. Glyph, Links, image, Glyph Bitmaps, Glyph Sketch, etc. taken all together represent a glyph), we can assume that the order of entries is important.

## Reading and parsing the data

As not every entry’s data format is known, parsing the data from the binary file into a data structure should be done in two steps:

1. Split the file into entries (key, size, data)
2. Parse the known entries’ data

When the original data of an entry is kept around, it can be written back to a new binary file as-is, even if other entries have been modified, and even if the purpose and the format of the entry in question is unknown.
