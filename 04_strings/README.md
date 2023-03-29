### Strings

#### Python 3
  - source code can be stores as UTF-8 (and other encodings)
  - internal representation depends on strings content
    * tries to be as compact as possible going from UCS-1 (u8) to UCS-2 (u16) to UCS-4 (u32)
    * for python >= 3.3
  - by default unicode

#### Rust
  - source code stored in UTF-8
  - by default unicode 
    * encoded in and enforced to be valid UTF-8 
  - direct slicing is dangerous, as it can result in invalid utf-8 which results in a panic
    * use .chars() iterator!
  - interesting crates
    * for grapheme-cluster use unicode-segmentation crate 
    * use crate unicode_normalization (e.g for comparisons)

#### Rust has two kind of strings 

- Strings 
 * heap allocated and not null terminated
 * are stored as a Vec<u8>
 * guranteed to be a valid utf-8 

- &str 
  * slice &[u8] pointing to valid utf-8 sequence
  * can be a view into a String

#### utf-8 
In Rust uses 1 to 4 bytes per character.

 7bits from U+0000 to U+007F = 0xxx_xxxx
11bits from U+0080 to U+07FF = 110x_xxxx 10xx_xxxx 
16bits from U+0800 to U+FFFF = 1110_xxxx 10xx_xxxx 10xx_xxxx
21bits from U+1000 to U+10FFFF = 1111_xxxx 10xx_xxxx 10xx_xxxx 10xx_xxxx


|char | hex           | bin
|-----|---------------|------
|A    | 41            | 0100_001
|Ä    | C3 84         | 1100_0011 1000_0100
|€    | E2 82 AC      | 1110_0010 1000_0001 1010_1100
|     | F0 9D 84 9E   | 1111_0000 1001_1101 1000_0100 1001_1110



