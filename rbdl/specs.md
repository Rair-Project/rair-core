### Attributes

#### `endianness`

Used to set endianness for numeric values that acquire more than one byte.
`endianness` attribute is compatible with `u16`, `s16`, `u32`, `u32`, `u64`, `s64`, `u128`, `s128`.

Possible values:
- `le`: little endian (Default value).
- `be`: big endian.

Example:
```rust
MyStruct: struct {
 #[endianness=be]
 a: u64
}
```

#### `alignment`

Align the start address.

Example:
```rust
MyStruct: struct {
 a: u8
 #[alignment=16]
 b: u8
}
```

#### `padding`

Align the end address.

Example:

```rust
#[padding=512]
TarHeader: struct {
 //stuff here
}
```

#### `discard`
Do not include getter function for members marked with this attribute.

Example:

```rust
Mystruct : struct{
 #[discard]
 my_size: u64,
 #[count=my_size]
 my_vec: Vec<u8>,
}
```

#### `unreliable`

Used on members that may fail to parse correctly. In which case these members will be ignored and parsing will continue past them. Any type referencing an unreliable member must be unreliable as well.

Example
```rust
Mystruct: struct {
 #[count=5, unreliable]
 x: oct
}
```

#### `static`

Used on members with already known values.

Example:

```rust
MyStruct : struct {
 #[static=0b101]
 x: u8,
 #[static=-5, endianness=be]
 y: i64,
 #[static = "Hello", delimiter=0, discard]
 signature: String,
 #[static=[0x00, 0x80, 0xff]]
 data: Vec<u8>,
 #[static='x']
 my_char: char,
 #[count=2, static=0o17]
 my_oct: oct
}
```
#### `encoding`
Allows setting encoding for `String` type. Available Encodings:
- ascii
- utf8
- utf16

#### `delimiter`

Sets the ending delimiter for both vectors and strings.
A delimiter can be a `u8` or `Vec<u8>`.
Note: In case of existence of both `delimiter` and `count`, both are valid whichever comes first.

#### `count`

Sets the number of elements in either `String` or vector.

### Example

#### Tar file parser

``` rust
FileType: enum {
 #[static='0']
 Regular: char,
 #[static='1']
 HardLink: char,
 #[static='2']
 SymbolicLink: char,
 #[static='3']
 CharacterDevice: char,
 #[static='4']
 BlockDevice: char,
 #[static='5']
 Directory: char,
 #[static='6']
 FIFONode: char,
}
#[padding=512]
TarHeader: struct {
 #[size=100, encoding="ascii"]
 file_name: String,
 #[size=8, encoding="ascii"]
 mode: oct,
 #[size=8, encoding="ascii"]
 uid: oct,
 #[size=8, encoding="ascii"]
 gid: oct,
 #[size=12, encoding="ascii"]
 size: oct,
 #[size=12, encoding="ascii"]
 mtime: oct,
 #[size=6, encoding="ascii"]
 checksum: oct,
 #[static=[0, 32], hidden]
 checksum_deimiter: Vec<u8>,
 typeflag: FileType,
 #[size=100, encoding="ascii", delimiter=0]
 linkname: String,
 #[static="ustar", delimiter=0, encoding="ascii", hidden]
 magic: String,
 #[size=2]
 version: String,
 #[size=32, encoding="ascii", delimiter=0]
 user: String,
 #[size=32, encoding="ascii", delimiter=0]
 group: String,
 #[size=8, encoding="ascii"]
 devmajor: oct,
 #[size=8, encoding="ascii"]
 devminor: oct,
 #[size=155, encoding="ascii", delimiter=0]
 prefix: String,
}
```



## Unresolved questions

[unresolved-questions]: #unresolved-questions

* Are these primitives enough to parse all binary file formats?

As pointed to me by @thestr4ng3r, No there is at least a case where using RBDL at its current state will not practical. Consider the following scenario: We have two independent version numbers followed by two pieces of data each depends on its respective version number as shown below:

```rust
FileA0B0: struct {
 #[static=0]
 version_a: u8,
 #[static=0]
 version_b: u8,
 data1: u8,
 data2: u8
}

FileA0B1: struct {
 #[static=0]
 version_a: u8,
 #[static=1]
 version_b: u8,
 data1: u8,
 data2: u16
}

FileA1B0: struct {
 #[static=1]
 version_a: u8,
 #[static=0]
 version_b: u8,
 data1: u16,
 data2: u8
}

FileA1B0: struct {
 #[static=1]
 version_a: u8,
 #[static=0]
 version_b: u8,
 data1: u16,
 data2: u8
}

FullFile: Enum {
 f00: FileA0B0,
 f01: FileA0B1,
 f10: FileA1B0,
 f11: FileA1B1,
}
```

We would need a way to convert `data1` and `data2` into independent enums each with a
separate selector. Ideally, that selector should be flexible enough to describe arbitrary
logic.

Kaitai approach is providing a set of predefined logic selectors such as `switch-on` and `FourCC`. While
these logic selectors cover mosts of the use cases found in the real world. They are not flexible, and they
can't model arbitrary logic without facing the same exponential explosion.

A suggested Approach is using the target language as expression language to provide arbitrary logic support.
Revisiting our original `enum` definition:

    Enums are only valid when all members pairs inside the enum are separable.

Since enums will be able to use that would decide which variant of the enum will be selected. It may not be necessary in that case for every 2 member pairs of the enum to be separable.

Example:

grammar.rbdl

```rust
Version: enum {
 #[static=0]
 V0: u8
 #[static=1]
 V1: u8
}
#[shadow=DataShadow]
Data : enum {
 D8: u8,
 D16: u16,
}

FullFile: struct {
 version_a: Version,
 version_b: Version,
 #[selector=version_sel, selector_args=[version_a]]
 data_a: Data,
 #[selector=version_sel, selector_args=[version_b]]
 data_b: Data,
}
```

grammar.rs
```rust
rbdl_include!("grammar.rdbl")

fn version_sel(version: &Version) -> DataShadow {
 match version {
 Version::V0 => DataShadow::D8,
 Version::V1 => DataShadow::D16
 }
}
```
The code snippet above introduces 3 attributes, the first of which is `shadow`. `shadow` attribute will only be available for rbdl enums and it will translate in the target language to `enum` or its nearest equivalent with the same fields as the attribute users, but no data. It will be used with selector attribute to signal which variation of the rbdl enum to be used. The other 2 attributes are `selector` and `selector_args`. These are used to identify the name of the selector function and which members of the parent struct to send to them.
