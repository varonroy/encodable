# Encodable
Encoding structs into vectors of primitives.

## Why?
This library serves the purpose of converting Rust structs into arrays of primitives.
While structs are convenient to work with, machine learning models typically require
tensor inputs. This library facilitates the conversion process, bridging the gap between 
the struct format and the array format, making it easier to feed structured data into
ML models. By utilizing this library, you can seamlessly convert your struct instances 
into arrays, ensuring compatibility with ML models without the need for manual conversion 
efforts.

## How?
This library is based on the `serde` library and provides a custom serializer and a deserializer.
Any struct that implements `serde::Deserialize` can be decoded, and any struct that implements
`serde::Serialize` can be encoded.

For example:
```rust
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Foo {
    a: f64,
    b: i64,
    c: bool,
}

let foo = Foo {
    a: 1.0,
    b: 2,
    c: true,
};

let encoding = encode(&foo).unwrap();
let foo: Foo = decode(&encoding).unwrap();
```

