# Changes

## v0.1.7

- Fix an issue with string decoding; fallback to Windows-1252 for old pre-Unicode VFBs.
- Parse postscript info, kerning classes, masks, guide properties, GDEF
- Guide counts can be negative, handle that case
- Coordinate deltas need a per-master cursor
- Remove unused phf crate

## v0.1.6

- Make the `truetype` module public (#18)
- Improve string encoding detection logic (#17)
- Decode most NameRecord strings correctly (#17)
- Don't crash on entries longer than 0xFFFF bytes (#16)
- Propagate number of masters so MM VFBs can be parsed (#16)
- Update dependencies

## v0.1.5

- First proper release with all parts of the release workflow automated
