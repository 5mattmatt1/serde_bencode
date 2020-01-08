use std::ops::{AddAssign, MulAssign, Neg};

// use serde::Deserialize;
use serde::de::{
    // EnumAccess, IntoDeserializer, VariantAccess
    self, DeserializeSeed, Visitor, MapAccess, SeqAccess,
};

use crate::error::{Error, Result};

pub use crate::read::{ Read, SliceRead, StrRead}; // IoRead,

pub struct Deserializer<R>
{
    read: R,
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    // input: &'de str,
}

impl<'de, R> Deserializer<R> 
where
    R: Read<'de>,
{
    pub fn new(read: R) -> Self {
        Deserializer {
            read: read,
        }
    }
    // By convention, `Deserializer` constructors are named like `from_xyz`.
    // That way basic use cases are satisfied by something like
    // `serde_json::from_str(...)` while advanced use cases that require a
    // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
    // pub fn from_str(input: &'de str) -> Self {
    //     Deserializer { input }
    // }
}

// impl<'a> Deserializer<'a, read::IoRead<R>>
// where
//     R: io::Read,
// {
//     /// Creates a JSON deserializer from an `io::Read`.
//     ///
//     /// Reader-based deserializers do not support deserializing borrowed types
//     /// like `&str`, since the `std::io::Read` trait has no non-copying methods
//     /// -- everything it does involves copying bytes out of the data source.
//     pub fn from_reader(reader: R) -> Self {
//         Deserializer::new(read::IoRead::new(reader))
//     }
// }

impl<'a> Deserializer<SliceRead<'a>> {
    /// Creates a JSON deserializer from a `&[u8]`.
    pub fn from_slice(bytes: &'a [u8]) -> Self {
        Deserializer::new(SliceRead::new(bytes))
    }
}

impl<'a> Deserializer<StrRead<'a>> {
    /// Creates a JSON deserializer from a `&str`.
    pub fn from_str(s: &'a str) -> Self {
        Deserializer::new(StrRead::new(s))
    }
}

impl<'de, R> Read<'de> for Deserializer<R> 
where
    R: Read<'de>
{
    fn read_bytes(&mut self, len: usize) -> Result<&'de [u8]>
    {
        self.read.read_bytes(len)
    }

    fn read_str(&mut self, len: usize) -> Result<&'de str>
    {
        self.read.read_str(len)
    }

    fn peek(&mut self) -> Result<u8> {
        self.read.peek()
    }

    // Consume the first character in the input.
    fn next(&mut self) -> Result<u8> 
    {
        self.read.next()
    }
}

impl<'de, R> Deserializer<R> 
where
    R: Read<'de>
{
    /// The `Deserializer::end` method should be called after a value has been fully deserialized.
    /// This allows the `Deserializer` to validate that the input stream is at the end or that it
    /// only has trailing whitespace.
    pub fn end(&mut self) -> Result<()> {
        // Need to ensure nothing here
        // match try!(self.parse_whitespace()) {
        //     Some(_) => Err(self.peek_error(ErrorCode::TrailingCharacters)),
        //     None => Ok(()),
        // }
        match self.peek()
        {
            Ok(_) => Err(Error::TrailingCharacters),
            Err(_) => Ok(())
        }
    }

    // Look at the first character in the input without consuming it.

    fn parse_signed<T>(&mut self) -> Result<T>
        where T: Neg<Output = T> + AddAssign<T> + MulAssign<T> + From<i8> + std::fmt::Display,
    {
        // TODO: Invalidate leading 0.
        if self.next()? as char != 'i' {
            return Err(Error::ExpectedI);
        }

        let mut int = match self.next()? as char {
            ch @ '0'..='9' => T::from(ch as i8 - b'0' as i8),
            '-' => match self.next()? as char {
                ch @ '0'..='9' => - T::from(ch as i8 - b'0' as i8),
                _ => { 
                    return Err(Error::ExpectedInteger);
                }
            },
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };

        loop {
            match self.next()? as char {
                ch @ '0'..= '9' => {
                    // self.input = &self.input[1..];
                    // self.input.chars().next();
                    // self.next();
                    int *= T::from(10);
                    int += T::from(ch as i8 - b'0' as i8);
                }
                'e' => {
                    // self.input = &self.input[1..];
                    // self.next();
                    return Ok(int);
                }
                _ => {
                    return Err(Error::UnexpectedChar)
                }
            }
        }
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        let mut len = match self.next()? as char {
            ch @ '0'..='9' => usize::from(ch as u8 - b'0'),
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };

        loop {
            match self.next()? as char {
                ch @ '0'..='9' => {
                    // self.input = &self.input[1..];
                    len *= 10 as usize;
                    len += usize::from(ch as u8 - b'0');
                }
                ':' => {
                    // self.input = &self.input[1..];
                    break;
                }
                _ => {
                    return Err(Error::ExpectedColon)
                }
            }
        }
        // Why is this unreachable?
        self.read_str(len)
    }

    fn parse_bytes(&mut self) -> Result<&'de [u8]> 
    {
        let mut len = match self.next()? as char {
            ch @ '0'..='9' => usize::from(ch as u8 - b'0'),
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };

        loop {
            match self.next()? as char {
                ch @ '0'..='9' => {
                    // self.input = &self.input[1..];
                    len *= 10 as usize;
                    len += usize::from(ch as u8 - b'0');
                }
                ':' => {
                    // self.input = &self.input[1..];
                    break;
                }
                _ => {
                    return Err(Error::ExpectedColon)
                }
            }
        }
        // // Why is this unreachable?
        self.read_bytes(len)
    }
}

// Seem to need to implement Access for these guys instead of the deserializer.
struct ColonSeparated<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
}

impl<'a, R: 'a> ColonSeparated<'a, R>
{

    fn new(de: &'a mut Deserializer<R>) -> Self {
        ColonSeparated {
            de
        }
    }
}

// impl<'de, R: 'de> MapAccess<'de> for ColonSeparated<'de, R> 
impl<'de, 'a, R: Read<'de> + 'a> MapAccess<'de> for ColonSeparated<'a, R> 
// where
//     R: Read<'de>
{
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        // Check if there are no more entries.
        if self.de.peek()? as char == 'e' {
            return Ok(None);
        }

        // Colon is required before every entry except the first.
        // if !self.first && self.de.next()? != ':' {
        //     return Err(Error::ExpectedMapColon);
        // }

        // self.first = false;

        // Deserialize a map key.
        let result = seed.deserialize(&mut *self.de).map(Some);
        return result;
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        // let ch = self.de.next()?; 
        // if ch != ':' {
        //     println!("Expected ':', found '{}'", ch);
        //     return Err(Error::ExpectedMapColon);
        // }
        // Deserialize a map value.
        seed.deserialize(&mut *self.de)
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a, R: Read<'de> + 'a> SeqAccess<'de> for ColonSeparated<'a, R> 
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.peek()? as char == 'e' {
            return Ok(None);
        }

        // Deserialize an array element.
        seed.deserialize(&mut *self.de).map(Some)
    }
}

impl<'de, 'a, R: Read<'de> + 'a> de::Deserializer<'de> for &'a mut Deserializer<R>
{
    type Error = Error;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.peek()? as char {
            'd' => self.deserialize_map(visitor),
            '0'..='9' => self.deserialize_str(visitor),
            _ => Err(Error::Syntax),
        }
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Parse the opening brace of the map.
        if self.next()? as char == 'd' {
            // Visitor
            let value = visitor.visit_map(ColonSeparated::new(& mut self))?;
            // Parse the closing brace of the map.
            if self.next()? as char == 'e' {
                return Ok(value);
            } else {
                return Err(Error::ExpectedMapEnd);
            } 
        } else {
            return Err(Error::ExpectedMap);
        }
        // unimplemented!()
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        return Err(Error::BoolUnsupported);
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_signed()?)
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parse_string()?)
        // unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
        // unimplemented!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        // println!("{}", self.parse_bytes()?)
        visitor.visit_borrowed_bytes(self.parse_bytes()?)
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, 
                                  _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str,
                                     _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        // Parse the opening bracket of the sequence.
        if self.next()? as char == 'l' {
            // Give the visitor access to each element of the sequence.
            let value = visitor.visit_seq(ColonSeparated::new(&mut self))?;
            // Parse the closing bracket of the sequence.
            if self.next()? as char == 'e' {
                Ok(value)
            } else {
                Err(Error::ExpectedListEnd)
            }
        } else {
            Err(Error::ExpectedList)
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str,
                                   _len: usize, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(self, _name: &'static str,
                             _fields: &'static [&'static str],
                             visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(self, _name: &'static str,
                           _variants: &'static [&'static str],
                           _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    // An identifier in Serde is the type that identifies a field of a struct or
    // the variant of an enum. In JSON, struct fields and enum variants are
    // represented as strings. In other formats they may be represented as
    // numeric indices.
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    // Like `deserialize_any` but indicates to the `Deserializer` that it makes
    // no difference which `Visitor` method is called because the data is
    // ignored.
    //
    // Some deserializers are able to implement this more efficiently than
    // `deserialize_any`, for example by rapidly skipping over matched
    // delimiters without paying close attention to the data in between.
    //
    // Some formats are not able to implement this at all. Formats that can
    // implement `deserialize_any` and `deserialize_ignored_any` are known as
    // self-describing.
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

fn from_trait<'de, R: 'de, T>(read: R) -> Result<T>
where
    R: Read<'de>,
    T: de::Deserialize<'de>,
{
    let mut de = Deserializer::new(read);
    let value = de::Deserialize::deserialize(&mut de)?;

    // Make sure the whole stream has been consumed.
    de.end()?;
    Ok(value)
}

// By convention, the public API of a Serde deserializer is one or more
// `from_xyz` methods such as `from_str`, `from_bytes`, or `from_reader`
// depending on what Rust types the deserializer is able to consume as input.
//
// This basic deserializer supports only `from_str`.
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: de::Deserialize<'a>,
{
    from_trait(StrRead::new(s))
}

pub fn from_slice<'a, T>(v: &'a [u8]) -> Result<T>
where
    T: de::Deserialize<'a>,
{
    from_trait(SliceRead::new(v))
}