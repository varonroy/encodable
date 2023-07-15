use serde::Serialize;

use super::error::Error;
use super::Encoding;

#[derive(Debug, Default)]
pub struct Serializer {
    encoding: Encoding,
}

impl Serializer {
    pub fn consume(self) -> Encoding {
        self.encoding
    }
}

impl<'a> serde::ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    /* Core types */
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.encoding.f.push(v);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.encoding.i.push(v);
        Ok(())
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.encoding.b.push(v);
        Ok(())
    }

    /* int */
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i128(self, _v: i128) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    /* uint */
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u128(self, _v: u128) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    /* float */
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    /* strings */
    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    /* bytes */
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    /* misc */
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    /* list / map */
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unimplemented!()
    }

    /* struct */
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }
}

impl<'a> serde::ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> serde::ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> serde::ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> serde::ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> serde::ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn skip_field(&mut self, _key: &'static str) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use itertools::izip;
    use serde::Serialize;

    use crate::{encode, Encoding};

    fn compare_encodings(a: &Encoding, b: &Encoding) {
        assert_eq!(a.f.len(), b.f.len());
        izip!(&a.f, &b.f).for_each(|(a, b)| assert_relative_eq!(a, b));
        izip!(&a.i, &b.i).for_each(|(a, b)| assert_eq!(a, b));
        izip!(&a.b, &b.b).for_each(|(a, b)| assert_eq!(a, b));
    }

    #[test]
    fn primitives() {
        #[derive(Serialize)]
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

        let encoding = Encoding {
            f: vec![1.0],
            i: vec![2],
            b: vec![true],
        };

        let encoded = encode(&foo).unwrap();

        compare_encodings(&encoding, &encoded);
    }

    #[test]
    fn nested_structs() {
        #[derive(Debug, Serialize)]
        struct Bar {
            a: f64,
            b: i64,
            c: bool,
            d: (f64, i64),
        }

        #[derive(Debug, Serialize)]
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

        let encoding = Encoding {
            f: vec![2.0, 8.0],
            i: vec![1, 2, 9],
            b: vec![false, true],
        };

        let encoded = encode(&foo).unwrap();

        compare_encodings(&encoding, &encoded);
    }
}
