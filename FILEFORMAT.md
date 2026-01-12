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

The file begins with a header that is be version-specific, but the version has apparently stayed at 3.0 even in FontLab Studio 5.2, the latest version before the major rewrite (FL 6+) which introduced a completely new file format (VFC).

The actual version of the app which created the file is stored further down in the file data.

```
1A 57 4C 46         u32     FL for Windows 3.0 signature
31                  u8      49 app version
30                  u8      48 file version
03                  u8      version major: 3
00                  u8      version minor: 0
2C 00 00 00         u32     data offset from start of file: 44 bytes
00 00 00 00                 padding with zero bytes until data start
00 00 00 00
00 00 00 00
00 00 00 00
00 00 00 00
00 00 00 00
00 00 00 00
00 00 00 00
```

## Data

After the header, the data is organized in type-length-value “entries”, which consist of an integer key, detailing the [type](vfb-reader/src/vfb_constants.rs) and data format of the entry, a data length value, and the actual data.

Most multi-byte values in the VFB are little-endian.

### Entry without data

It seems to be perfectly OK if an entry has a data size of 0. In that case, no data bytes are present. In fact, the first entries after the header are structural markers that contain no data.

```
01 00               u16     type: 1 (Start of block: File)
00 00               u16     length: 0
                            value: null
--------------- next entry
04 00               u16     type: 4 (Start of block: Font)
00 00               u16     length: 0
                            value: null
--------------- next entry
...
--------------- next entry
06 01               u16     type: 262 (Start of block: Names)
00 00               u16     length: 0
                            value: null
--------------- next entry
...
--------------- next entry
06 02               u16     type: 518 (262 + 256, End of block: Names)
00 00               u16     length: 0
                            value: null
--------------- next entry
01 01               u16     type: 257 (Start of block: Font Info)
00 00               u16     length: 0
                            value: null
--------------- next entry
...
--------------- next entry
01 02               u16     type: 513 (257 + 256, End of block: Font Info)
00 00               u16     length: 0
                            value: null
--------------- next entry
0F 01               u16     type: 271 (Start of block: MM Font Info)
00 00               u16     length: 0
                            value: null
--------------- next entry
...
--------------- next entry
0F 02               u16     type: 527 (271 + 256, End of block: MM Font Info)
00 00               u16     length: 0
                            value: null
--------------- next entry
...
--------------- next entry
05 00               u16     type: 5 (End of block: Font)
00 00               u16     length: 0
                            value: null
--------------- next entry
02 00               u16     type: 2 (End of block: File)
00 00               u16     length: 0
                            value: null
--------------- end of file
```

The presence of those structural blocks hints at that one file might contain multiple fonts, but app support of this feature is unknown. The structural blocks can be used to visualize the conceptual structure of the VFB data:

```
Entry                          Type
file_start                        1
    font_start                    4
        FLVersion                10
        names_start             262
            encoding_default   1501
            encoding_default   1501
            encoding_default   1501
            ...
            encoding           1500
            encoding           1500
            encoding           1500
            ...
            mm_encoding_type   1502
        names_end               518 = 262 + 256
        font_info_start         257
            font_name          1026
            MasterCount        1503
            weight_vector      1517
            ...
        font_info_end           513 = 257 + 256
        mm_font_info_start      271
            AxisCount          1513
            AxisName          1514
            ...
        mm_font_info_end        527 = 271 + 256
        GlobalGuides           1294
        GlobalGuideProperties  1296
        GlobalMask             1295
        default_character      1066
        ---- repeat for each glyph:
            Glyph              2001
            Links              2008
            image              2007
            Bitmaps            2013
            VSB                2023
            Sketch             2019
            ...
        ----- end of glyph
        mm_kerning_start        272
            MMKernPair         1410
            MMKernPair         1410
            MMKernPair         1410
            ...
        mm_kerning_end          528 = 272 + 256
    font_end                      5
file_end                          2
```

### Entry with a data size greater than 0 and less than or equal to `u16::MAX` (65535)

```
DD 05               u16     1501  entry key (1501: "Encoding Default")
06 00               u16     6     entry data length
--------------- start of 6-byte chunk (entry-specific format)
34 00               u16     52 (glyph index in encoding)
66 6F 75 72         string  "four" (glyph name in encoding)
--------------- end of 6-byte chunk
```

### Entry with a data size larger than `u16::MAX` (65535)

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

## Entry order

As there are some aspects of a font source file that are represented by multiple entries in the VFB file, (e.g. Glyph, Links, image, Glyph Bitmaps, Glyph Sketch, etc. taken all together represent a glyph), we must assume that the order of entries is important.

## Reading and parsing the data

As not every entry’s data format is known, parsing the data from the binary file into a data structure should be done in several steps:

1. Read and parse the header. The header must always be parsed because its size in the binary file isn't known before it is parsed.
2. Continue reading at the data offset contained in the header, split the rest of the file into entries (key, size, data)
3. Parse the known entries’ data

When the original data of an entry is kept around, it can be written back to a new binary file as-is, even if other entries have been modified, and even if the purpose and the format of the entry in question is unknown.

## The FLVersion entry

The FLVersion entry is special in that it comes directly after the "start of font" marker, and before the "start of names" marker. It is of type 11.

When trying to write VFBs, the entry is relevant because it contains the version of the app that wrote the file, and a platform ID.

The entry is not present in files written with app version 3, but present in app version 4.5, and the 5.x versions. Up until version 5.0.4, it sometimes (?) contained the serial number of the app, but in later versions, the serial number field always contains a 0. This is not the number needed to activate the app, but rather an actual sequential serial number.

Here are some examples of real-world entries:

### FLVersion from FLS 5.2.2-5714 on Windows

Note that the platform is given as macOS even though the file was written on Windows.

```
0A 00               u16     type: 10
0B 00               u16     length: 11
--------------- start of 11-byte chunk in "key--encoded-value" format
01                  u8      key: 1 (platform)
8C                  EV      value: 1 (macOS)
02                  u8      key: 2 (app version)
FF 05 02 02 80      EV      value: 84017792 (5, 2, 2, 128)
03                  u8      key: 3 (serial number)
8B                  EV      value: 0
00                  u8      key: 0 (end marker)
--------------- end of 11-byte chunk
```

### FLVersion from FLS 5.0.4 on macOS

```
0A 00               u16     type: 10
0F 00               u16     length: 15
--------------- start of 15-byte chunk in "key--encoded-value" format
01                  u8      key: 1 (platform)
8C                  EV      value: 1 (macOS)
02                  u8      key: 2 (app version)
FF 05 00 04 80      EV      value: 83887232 (5, 0, 4, 128)
03                  u8      key: 3 (serial number)
FF 00 00 12 08      EV      value: 4616 (yep, that's my serial number leaked)
00                  u8      key: 0 (end marker)
--------------- end of 15-byte chunk
```

### FLVersion from FLS 4.5 on Windows

Contains this app version:

```
FF 04 05 04 01      EV      value: (4, 5, 4, 1)
```

### FLVersion from FLS 5.0.4 on Windows

Contains this app version:

```
FF 05 00 00 01      EV      value: (5, 0, 0, 1)
```
