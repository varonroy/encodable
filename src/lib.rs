pub mod deserializer;
pub mod error;
pub mod serializer;

use serde::{Deserialize, Serialize};

use self::{deserializer::Deserializer, error::Result, serializer::Serializer};

/// An encoding of a struct.
///
/// Example:
/// ```rust
/// use encodable::{encode, decode};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, Serialize)]
/// struct Foo {
///     a: f64,
///     b: i64,
///     c: bool,
/// }
///
/// let foo = Foo {
///     a: 1.0,
///     b: 2,
///     c: true,
/// };
///
/// let encoding = encode(&foo).unwrap();
/// let foo: Foo = decode(&encoding).unwrap();
/// ````
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Encoding {
    pub f: Vec<f64>,
    pub i: Vec<i64>,
    pub b: Vec<bool>,
}

/// Encoding a struct
pub fn encode<T>(value: &T) -> Result<Encoding>
where
    T: Serialize,
{
    let mut serializer = Serializer::default();
    value.serialize(&mut serializer)?;
    Ok(serializer.consume())
}

/// Decoding a struct
pub fn decode<'de, T>(encoding: &'de Encoding) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::from_encoding(encoding);
    let res = T::deserialize(&mut deserializer);
    if !deserializer.completed() {
        Err(error::Error::Incomplete)
    } else {
        res
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::{decode, encode};

    /// Testing struct -> encoding -> struct -> encoding
    #[test]
    fn encode_decode() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Bar {
            a: f64,
            b: i64,
            c: bool,
            d: (f64, i64),
        }

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Foo {
            a: i64,
            bar: Bar,
            c: bool,
        }

        let foo = Foo {
            a: 1,
            bar: Bar {
                a: 2.0,
                b: 2,
                c: false,
                d: (8.0, 9),
            },
            c: true,
        };

        let encoding = encode(&foo).unwrap();
        let foo_decoded: Foo = decode(&encoding).unwrap();
        let foo_decoded_encoding = encode(&foo_decoded).unwrap();
        assert_eq!(foo, foo_decoded);
        assert_eq!(encoding, foo_decoded_encoding);
    }
}
