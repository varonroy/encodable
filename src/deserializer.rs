use serde::de::{DeserializeSeed, SeqAccess};

use super::{
    error::{Error, Result},
    Encoding,
};

pub struct Deserializer<'de> {
    encoding: &'de Encoding,
    f_i: usize,
    i_i: usize,
    b_i: usize,
}

impl<'de> Deserializer<'de> {
    pub fn from_encoding(encoding: &'de Encoding) -> Self {
        Self {
            encoding,
            f_i: 0,
            i_i: 0,
            b_i: 0,
        }
    }

    pub fn completed(&self) -> bool {
        self.f_i == self.encoding.f.len()
            && self.i_i == self.encoding.i.len()
            && self.b_i == self.encoding.b.len()
    }

    fn next_float(&mut self) -> Result<f64> {
        let f = self
            .encoding
            .f
            .get(self.f_i)
            .copied()
            .ok_or(Error::FloatIndexOutOfBounds)?;
        self.f_i += 1;
        Ok(f)
    }

    fn next_int(&mut self) -> Result<i64> {
        let i = self
            .encoding
            .i
            .get(self.i_i)
            .copied()
            .ok_or(Error::IntIndexOutOfBounds)?;
        self.i_i += 1;
        Ok(i)
    }

    fn next_bool(&mut self) -> Result<bool> {
        let b = self
            .encoding
            .b
            .get(self.b_i)
            .copied()
            .ok_or(Error::BoolIndexOutOfBounds)?;
        self.b_i += 1;
        Ok(b)
    }
}

impl<'de, 'a> serde::de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    /* Core types */
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f64(self.next_float()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i64(self.next_int()?)
    }

    fn deserialize_bool<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bool(self.next_bool()?)
    }

    /* int */
    fn deserialize_i8<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_i32<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i128<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    /* uint */
    fn deserialize_u8<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u128<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    /* float */
    fn deserialize_f32<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    /* strings */
    fn deserialize_str<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    /* bytes */
    fn deserialize_byte_buf<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    /* option */
    fn deserialize_option<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    /* () */
    fn deserialize_unit<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    /* list / sequence */
    fn deserialize_seq<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    /* tuple */
    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let fields = Fields::new(self, len);
        visitor.visit_seq(fields)
    }

    /* map */
    fn deserialize_map<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    /* struct */
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let fields = Fields::new(self, fields.len());
        visitor.visit_seq(fields)
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!("unit struct")
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!("newtupe struct")
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!("tuple struct")
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    /* identifier */
    fn deserialize_identifier<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    /* any */
    fn deserialize_any<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }
}

struct Fields<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    num_fields: usize,
    i: usize,
}

impl<'a, 'de> Fields<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, num_fields: usize) -> Self {
        Fields {
            de,
            num_fields,
            i: 0,
        }
    }
}

impl<'de, 'a> SeqAccess<'de> for Fields<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.i >= self.num_fields {
            return Ok(None);
        }

        seed.deserialize(&mut *self.de).map(Some)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use serde::Deserialize;

    use crate::{decode, Encoding};

    #[test]
    fn primitives() {
        let encoding = Encoding {
            f: vec![1.0],
            i: vec![],
            b: vec![],
        };

        let f: f64 = decode(&encoding).unwrap();
        assert_relative_eq!(f, 1.0);

        let encoding = Encoding {
            f: vec![],
            i: vec![1],
            b: vec![],
        };

        let i: i64 = decode(&encoding).unwrap();
        assert_eq!(i, 1);

        let encoding = Encoding {
            f: vec![],
            i: vec![],
            b: vec![true],
        };

        let b: bool = decode(&encoding).unwrap();
        assert!(b);
    }

    #[test]
    fn flat_struct() {
        #[derive(Deserialize)]
        struct Foo {
            a: f64,
            b: i64,
            c: bool,
        }

        let encoding = Encoding {
            f: vec![1.0],
            i: vec![2],
            b: vec![true],
        };

        let foo: Foo = decode(&encoding).unwrap();
        assert_relative_eq!(foo.a, 1.0);
        assert_eq!(foo.b, 2);
        assert!(foo.c);
    }

    #[test]
    fn nested_struct() {
        #[derive(Debug, Deserialize)]
        struct Bar {
            a: f64,
            b: i64,
            c: bool,
            d: (f64, i64),
        }

        #[derive(Debug, Deserialize)]
        struct Foo {
            a: i64,
            bar: Bar,
            c: bool,
        }

        let encoding = Encoding {
            f: vec![1.0, 8.0],
            i: vec![1, 2, 9],
            b: vec![false, true],
        };

        let foo: Foo = decode(&encoding).unwrap();
        assert_eq!(foo.a, 1);
        assert_relative_eq!(foo.bar.a, 1.0);
        assert_eq!(foo.bar.b, 2);
        assert!(!foo.bar.c);
        assert_relative_eq!(foo.bar.d.0, 8.0);
        assert_eq!(foo.bar.d.1, 9);
        assert!(foo.c);
    }
}
