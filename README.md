# auto_enum_type

`auto_enum_type` is a Rust procedural macro library that simplifies the creation of type enums from existing enums. It automatically generates a corresponding type enum and a conversion method, making it easier to work with enum variants as types.

## Installation

Add `auto_enum_type` to your `Cargo.toml`:
```toml
[dependencies]
auto_enum_type = { git = "https://github.com/Th0rgal/auto_enum_type" }
```


## Usage

Use the `#[derive(TypeEnum)]` attribute on your enum:
```rs
use auto_enum_type::TypeEnum;

#[derive(TypeEnum)]
pub enum MyEnum {
    Variant1,
    Variant2(String),
    Variant3 { field: i32 },
}
```

This generates:

1. A `MyEnumType` enum with variants matching `MyEnum`.
2. An `event_type()` method for `MyEnum` to get the corresponding `MyEnumType`.

The macro automatically chooses the most efficient representation (u8, u16, u32, or u64) based on the number of variants.

## Features

- Supports enums with up to 2^64 variants
- Works with enums that have fields in their variants
- Maintains `repr(C)` for the original enum
- Efficient conversion using transmutation of the discriminant

Note: This macro assumes standard `repr(C)` layout. For critical applications, additional safety checks may be necessary.