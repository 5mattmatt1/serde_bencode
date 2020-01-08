
use crate::error::{Error, Result};

// Where is Private::Sealed implemented
pub trait Read<'de> {
    #[doc(hidden)]
    fn read_str(&mut self, len: usize) -> Result<&'de str>;

    #[doc(hidden)]
    fn next(&mut self) -> Result<u8>;

    #[doc(hidden)]
    fn peek(&mut self) -> Result<u8>;
}

/// JSON input source that reads from a slice of bytes.
//
// This is more efficient than other iterators because peek() can be read-only
// and we can compute line/col position only if an error happens.
pub struct SliceRead<'a> {
    slice: &'a [u8],
    /// Index of the *next* byte that will be returned by next() or peek().
    index: usize,
}

pub struct StrRead<'a> {
    delegate: SliceRead<'a>,
}

impl<'a> SliceRead<'a> {
    /// Create a JSON input source to read from a slice of bytes.
    pub fn new(slice: &'a [u8]) -> Self {
        SliceRead {
            slice: slice,
            index: 0,
        }
    }
}

impl<'a> StrRead<'a> {
    /// Create a JSON input source to read from a UTF-8 string.
    pub fn new(s: &'a str) -> Self {
        StrRead {
            delegate: SliceRead::new(s.as_bytes()),
        }
    }
}

impl<'a> Read<'a> for SliceRead<'a> {
    #[inline]
    fn read_str(&mut self, len: usize) -> Result<&'a str>
    {
        if self.index + len - 1 < self.slice.len() {
            let string = std::str::from_utf8(&self.slice[self.index..self.index+len]);
            self.index += len;
            match string
            {
                Ok(string) => Ok(string),
                Err(e) => Err(Error::UTF8Error(e)) 
            }
        } else {
            Err(Error::IndexError)   
        }
    }

    #[inline]
    fn next(&mut self) -> Result<u8> {
        // `Ok(self.slice.get(self.index).map(|ch| { self.index += 1; *ch }))`
        // is about 10% slower.
        if self.index < self.slice.len() {
            let ch = self.slice[self.index];
            self.index += 1;
            Ok(ch)
        } else {
            Err(Error::IndexError)
        }
    }

    #[inline]
    fn peek(&mut self) -> Result<u8> {
        // `Ok(self.slice.get(self.index).map(|ch| *ch))` is about 10% slower
        // for some reason.
        if self.index < self.slice.len() {
            Ok(self.slice[self.index])
        } else
        {
            Err(Error::IndexError)
        }
    }
}

// Seems unsafe...
// Shouldn't I ensure that it is valid UTF-8 for every next and peek?
impl<'a> Read<'a> for StrRead<'a> {
    #[inline]
    fn read_str(&mut self, len: usize) -> Result<&'a str>
    {
        self.delegate.read_str(len)
    }

    #[inline]
    fn next(&mut self) -> Result<u8> {
        self.delegate.next()
    }

    #[inline]
    fn peek(&mut self) -> Result<u8> {
        self.delegate.peek()
    }
}