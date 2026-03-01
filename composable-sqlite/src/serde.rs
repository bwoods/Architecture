#![allow(unsafe_code)]

use serde::ser::Error;
use serde::ser::{Impossible, StdError};
use serde::{Serialize, ser};
use sqlite3_sys::{
    SQLITE_ERROR, SQLITE_NOTFOUND, SQLITE_OK, SQLITE_TOOBIG, SQLITE_UTF8, sqlite3_bind_blob64,
    sqlite3_bind_double, sqlite3_bind_int, sqlite3_bind_int64, sqlite3_bind_null,
    sqlite3_bind_parameter_count, sqlite3_bind_parameter_name, sqlite3_bind_text64, sqlite3_errstr,
    sqlite3_stmt,
};
use std::collections::HashMap;
use std::ffi::{CStr, c_uchar};
use std::fmt::{Debug, Display, Formatter};

pub fn bind<T>(stmt: *mut sqlite3_stmt, parameters: T) -> Result<(), SerError>
where
    T: Serialize,
{
    let serializer = Serializer::new(stmt);
    parameters.serialize(serializer)
}

pub struct Serializer<'a> {
    stmt: *mut sqlite3_stmt,
    columns: HashMap<&'a [u8], i32>,
    current: i32,
}

impl Serializer<'_> {
    pub fn new(stmt: *mut sqlite3_stmt) -> Self {
        let mut columns = HashMap::default();

        for col in unsafe { 1..=sqlite3_bind_parameter_count(stmt) } {
            columns.insert(
                unsafe { CStr::from_ptr(sqlite3_bind_parameter_name(stmt, col)).to_bytes() },
                col,
            );
        }

        Self {
            stmt,
            columns,
            current: 1,
        }
    }
}

impl ser::SerializeStruct for Serializer<'_> {
    type Ok = ();
    type Error = SerError;

    /// Serialize a struct field.
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = Serializer::new(self.stmt);
        serializer.current = *self
            .columns
            .get(&key.as_bytes())
            .ok_or(SerError(SQLITE_NOTFOUND))?;
        value.serialize(serializer) // taken by value…
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl ser::SerializeTuple for Serializer<'_> {
    type Ok = ();
    type Error = SerError;

    /// Serialize a tuple element.
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = Serializer::new(self.stmt);
        serializer.current = self.current;

        let res = value.serialize(serializer);
        self.current += 1;
        res
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// Serialize a sequence element.
impl ser::SerializeSeq for Serializer<'_> {
    type Ok = ();
    type Error = SerError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        <Self as ser::SerializeTuple>::serialize_element(self, value)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl ser::Serializer for Serializer<'_> {
    type Ok = ();
    type Error = SerError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    #[inline(always)]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    #[inline(always)]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    #[inline(always)]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    #[inline(always)]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        unsafe { sqlite3_bind_int(self.stmt, self.current, v).result() }
    }

    #[inline(always)]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        unsafe { sqlite3_bind_int64(self.stmt, self.current, v).result() }
    }

    #[inline(always)]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    #[inline(always)]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    #[inline(always)]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        if v > i64::MAX as u64 {
            return Err(SerError(SQLITE_TOOBIG));
        }

        self.serialize_i64(v as i64)
    }

    #[inline(always)]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    #[inline(always)]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        unsafe { sqlite3_bind_double(self.stmt, self.current, v).result() }
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buffer = [0; 4];
        self.serialize_str(v.encode_utf8(&mut buffer))
    }

    #[inline(always)]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        unsafe {
            sqlite3_bind_text64(
                self.stmt,
                self.current,
                v.as_ptr().cast(),
                v.len() as u64,
                None,
                SQLITE_UTF8 as c_uchar,
            )
            .result()
        }
    }

    #[inline(always)]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unsafe {
            sqlite3_bind_blob64(
                self.stmt,
                self.current,
                v.as_ptr().cast(),
                v.len() as u64,
                None,
            )
            .result()
        }
    }

    #[inline(always)]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unsafe { sqlite3_bind_null(self.stmt, self.current).result() }
    }

    #[inline(always)]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline(always)]
    /// Serialize a `()` value.
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    #[inline(always)]
    /// Serialize a unit struct like `struct Unit` or `PhantomData<T>`.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    #[inline(always)]
    /// Serialize a unit variant like `E::A` in `enum E { A, B }`.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unsafe {
            sqlite3_bind_text64(
                self.stmt,
                self.current,
                variant.as_ptr().cast(),
                variant.len() as u64,
                None,
                SQLITE_UTF8 as c_uchar,
            )
            .result()
        }
    }

    /// Serialize a newtype struct like `struct Millimeters(u8)`.
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = Serializer::new(self.stmt);
        serializer.current = self.current;
        value.serialize(serializer)
    }

    /// Serialize a newtype variant like `E::N` in `enum E { N(u8) }`.
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Self::Error::custom("serialize_newtype_variant"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    /// Begin to serialize a tuple struct like `struct Rgb(u8, u8, u8)`.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unreachable!()
    }

    /// Begin to serialize a tuple variant like `E::T` in `enum E { T(u8, u8) }`.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unreachable!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unreachable!()
    }

    /// Begin to serialize a struct like `struct Rgb { r: u8, g: u8, b: u8 }`.
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    /// Begin to serialize a struct variant like `E::S` in `enum E { S { r: u8,
    /// g: u8, b: u8 } }`.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Self::Error::custom("serialize_struct_variant"))
    }
}
pub struct SerError(i32);

impl StdError for SerError {}

impl Debug for SerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for SerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cstr = unsafe { CStr::from_ptr(sqlite3_errstr(self.0)) };
        f.write_str(&cstr.to_owned().into_string().unwrap_or_default())
    }
}

impl ser::Error for SerError {
    fn custom<T>(_msg: T) -> Self
    where
        T: Display,
    {
        SerError(SQLITE_ERROR) // FIXME; useless…
    }
}

trait SerResult {
    fn result(self) -> Result<(), SerError>;
}

impl SerResult for i32 {
    fn result(self) -> Result<(), SerError> {
        match self {
            SQLITE_OK => Ok(()),
            _ => Err(SerError(self)),
        }
    }
}
