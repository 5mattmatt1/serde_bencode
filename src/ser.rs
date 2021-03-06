use serde::{ser, Serialize};

use crate::error::{Error, Result};

pub struct Serializer {
    // This string starts empty and JSON is appended as values are serialized.
    output: String,
}

// By convention, the public API of a Serde serializer is one or more `to_abc`
// functions such as `to_string`, `to_bytes`, or `to_writer` depending on what
// Rust types the serializer is able to produce as output.
//
// This basic serializer supports only `to_string`.
pub fn to_str<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = Error;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<()>
	{
		unimplemented!()
	}

	fn serialize_i8(self, v: i8) -> Result<()>
	{
		self.serialize_i64(i64::from(v))
	}

	fn serialize_i16(self, v: i16) -> Result<()>
	{
		self.serialize_i64(i64::from(v))
	}

	fn serialize_i32(self, v: i32) -> Result<()>
	{
		self.serialize_i64(i64::from(v))
	}

	fn serialize_i64(self, v: i64) -> Result<()>
	{
        self.output += "i";
		self.output += &v.to_string();
        self.output += "e";
        Ok(())
	}

	fn serialize_u8(self, _v: u8) -> Result<()>
	{
		unimplemented!()
	}

	fn serialize_u16(self, _v: u16) -> Result<()>
	{
		unimplemented!()
	}

	fn serialize_u32(self, _v: u32) -> Result<()>
	{
		unimplemented!()
	}

	fn serialize_u64(self, _v: u64) -> Result<()>
	{
		unimplemented!()
	}

	fn serialize_f32(self, _v: f32) -> Result<()>
	{
		unimplemented!()
	}

	fn serialize_f64(self, _v: f64) -> Result<()>
	{
		unimplemented!()
	}

	fn serialize_char(self, _v: char) -> Result<()>
	{
		unimplemented!()
	}

	fn serialize_str(self, v: &str) -> Result<()>
	{
        self.output += &v.len().to_string();
        self.output += ":";
        self.output += v;
		Ok(())
	}

	fn serialize_bytes(self, _v: &[u8]) -> Result<()>
	{
		unimplemented!()
	}

	fn serialize_none(self) -> Result<()>
	{
		unimplemented!()
	}

	fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
	{
		unimplemented!()
	}

	fn serialize_unit(self) -> Result<()>
	{
		unimplemented!()
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<()>
	{
		unimplemented!()
	}

	fn serialize_unit_variant(self, _name: &'static str,
                              _variant_index: u32,
                              _variant: &'static str,) -> Result<()>
	{
		unimplemented!()
	}

	fn serialize_newtype_struct<T>(self, _name: &'static str,
                                    _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
	{
		unimplemented!()
	}

	fn serialize_newtype_variant<T>(self, _name: &'static str,
                                _variant_index: u32,
                                _variant: &'static str,
                                _value: &T,) -> Result<()>
    where
        T: ?Sized + Serialize,
	{
		unimplemented!()
	}

	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq>
	{
		self.output += "l";
        Ok(self)
	}

	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple>
	{
		unimplemented!()
	}

	fn serialize_tuple_struct(self, _name: &'static str,
                              _len: usize,) -> Result<Self::SerializeTupleStruct>
	{
		unimplemented!()
	}

	fn serialize_tuple_variant(self, _name: &'static str,
                                _variant_index: u32,
                                _variant: &'static str,
                                _len: usize,) -> Result<Self::SerializeTupleVariant>
	{
		unimplemented!()
	}

	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap>
	{
		self.output += "d";
        Ok(self)
	}

	fn serialize_struct(self, _name: &'static str, 
                        len: usize,) -> Result<Self::SerializeStruct>
	{
		self.serialize_map(Some(len))
	}

	fn serialize_struct_variant(self, _name: &'static str,
                                _variant_index: u32,
                                _variant: &'static str,
                                _len: usize,) -> Result<Self::SerializeStructVariant>
	{
		unimplemented!()
	}
}

// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        self.output += "e";
        Ok(())
    }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // if !self.output.ends_with('[') {
        //     self.output += ",";
        // }
        // value.serialize(&mut **self)
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        // self.output += "]";
        // Ok(())
        unimplemented!()
    }
}

// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // if !self.output.ends_with('[') {
        //     self.output += ",";
        // }
        // value.serialize(&mut **self)
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        // self.output += "]";
        // Ok(())
        unimplemented!()
    }
}

// Tuple variants are a little different. Refer back to the
// `serialize_tuple_variant` method above:
//
//    self.output += "{";
//    variant.serialize(&mut *self)?;
//    self.output += ":[";
//
// So the `end` method in this impl is responsible for closing both the `]` and
// the `}`.
impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // if !self.output.ends_with('[') {
        //     self.output += ",";
        // }
        // value.serialize(&mut **self)
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        // self.output += "]}";
        // Ok(())
        unimplemented!()
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // if !self.output.ends_with('{') {
        //     self.output += ",";
        // }
        // key.serialize(&mut **self)
        unimplemented!()
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // self.output += ":";
        // value.serialize(&mut **self)
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        // self.output += "}";
        // Ok(())
        unimplemented!()
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "e";
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // if !self.output.ends_with('{') {
        //     self.output += ",";
        // }
        // key.serialize(&mut **self)?;
        // self.output += ":";
        // value.serialize(&mut **self)
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        // self.output += "}}";
        // Ok(())
        unimplemented!()
    }
}
