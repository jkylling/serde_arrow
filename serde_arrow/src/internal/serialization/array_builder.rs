use std::collections::BTreeMap;

use half::f16;
use serde::Serialize;

use crate::internal::{
    arrow::Array,
    error::{Context, Error, Result},
};

use super::{
    binary_builder::BinaryBuilder, bool_builder::BoolBuilder, date32_builder::Date32Builder,
    date64_builder::Date64Builder, decimal_builder::DecimalBuilder,
    dictionary_utf8_builder::DictionaryUtf8Builder, duration_builder::DurationBuilder,
    fixed_size_binary_builder::FixedSizeBinaryBuilder,
    fixed_size_list_builder::FixedSizeListBuilder, float_builder::FloatBuilder,
    int_builder::IntBuilder, list_builder::ListBuilder, map_builder::MapBuilder,
    null_builder::NullBuilder, simple_serializer::merge_annotations,
    simple_serializer::SimpleSerializer, struct_builder::StructBuilder, time_builder::TimeBuilder,
    union_builder::UnionBuilder, unknown_variant_builder::UnknownVariantBuilder,
    utf8_builder::Utf8Builder,
};

#[derive(Debug, Clone)]
pub enum ArrayBuilder {
    Null(NullBuilder),
    Bool(BoolBuilder),
    I8(IntBuilder<i8>),
    I16(IntBuilder<i16>),
    I32(IntBuilder<i32>),
    I64(IntBuilder<i64>),
    U8(IntBuilder<u8>),
    U16(IntBuilder<u16>),
    U32(IntBuilder<u32>),
    U64(IntBuilder<u64>),
    F16(FloatBuilder<f16>),
    F32(FloatBuilder<f32>),
    F64(FloatBuilder<f64>),
    Date32(Date32Builder),
    Date64(Date64Builder),
    Time32(TimeBuilder<i32>),
    Time64(TimeBuilder<i64>),
    Duration(DurationBuilder),
    Decimal128(DecimalBuilder),
    List(ListBuilder<i32>),
    LargeList(ListBuilder<i64>),
    FixedSizedList(FixedSizeListBuilder),
    Binary(BinaryBuilder<i32>),
    LargeBinary(BinaryBuilder<i64>),
    FixedSizeBinary(FixedSizeBinaryBuilder),
    Map(MapBuilder),
    Struct(StructBuilder),
    Utf8(Utf8Builder<i32>),
    LargeUtf8(Utf8Builder<i64>),
    DictionaryUtf8(DictionaryUtf8Builder),
    Union(UnionBuilder),
    UnknownVariant(UnknownVariantBuilder),
}

macro_rules! dispatch {
    ($obj:expr, $wrapper:ident($name:ident) => $expr:expr) => {
        match $obj {
            $wrapper::Bool($name) => $expr,
            $wrapper::Null($name) => $expr,
            $wrapper::I8($name) => $expr,
            $wrapper::I16($name) => $expr,
            $wrapper::I32($name) => $expr,
            $wrapper::I64($name) => $expr,
            $wrapper::U8($name) => $expr,
            $wrapper::U16($name) => $expr,
            $wrapper::U32($name) => $expr,
            $wrapper::U64($name) => $expr,
            $wrapper::F16($name) => $expr,
            $wrapper::F32($name) => $expr,
            $wrapper::F64($name) => $expr,
            $wrapper::Date32($name) => $expr,
            $wrapper::Date64($name) => $expr,
            $wrapper::Time32($name) => $expr,
            $wrapper::Time64($name) => $expr,
            $wrapper::Duration($name) => $expr,
            $wrapper::Decimal128($name) => $expr,
            $wrapper::Utf8($name) => $expr,
            $wrapper::LargeUtf8($name) => $expr,
            $wrapper::List($name) => $expr,
            $wrapper::LargeList($name) => $expr,
            $wrapper::FixedSizedList($name) => $expr,
            $wrapper::Binary($name) => $expr,
            $wrapper::LargeBinary($name) => $expr,
            $wrapper::FixedSizeBinary($name) => $expr,
            $wrapper::Map($name) => $expr,
            $wrapper::Struct($name) => $expr,
            $wrapper::DictionaryUtf8($name) => $expr,
            $wrapper::Union($name) => $expr,
            $wrapper::UnknownVariant($name) => $expr,
        }
    };
}

impl ArrayBuilder {
    pub fn is_nullable(&self) -> bool {
        dispatch!(self, Self(builder) => builder.is_nullable())
    }

    pub fn into_array(self) -> Result<Array> {
        dispatch!(self, Self(builder) => builder.into_array())
    }
}

impl ArrayBuilder {
    /// Take the contained array builder, while leaving structure intact
    // TODO: use ArrayBuilder as return type for the impls and use dispatch here
    pub fn take(&mut self) -> ArrayBuilder {
        match self {
            Self::Null(builder) => Self::Null(builder.take()),
            Self::Bool(builder) => Self::Bool(builder.take()),
            Self::I8(builder) => Self::I8(builder.take()),
            Self::I16(builder) => Self::I16(builder.take()),
            Self::I32(builder) => Self::I32(builder.take()),
            Self::I64(builder) => Self::I64(builder.take()),
            Self::U8(builder) => Self::U8(builder.take()),
            Self::U16(builder) => Self::U16(builder.take()),
            Self::U32(builder) => Self::U32(builder.take()),
            Self::U64(builder) => Self::U64(builder.take()),
            Self::F16(builder) => Self::F16(builder.take()),
            Self::F32(builder) => Self::F32(builder.take()),
            Self::F64(builder) => Self::F64(builder.take()),
            Self::Date32(builder) => Self::Date32(builder.take()),
            Self::Date64(builder) => Self::Date64(builder.take()),
            Self::Time32(builder) => Self::Time32(builder.take()),
            Self::Time64(builder) => Self::Time64(builder.take()),
            Self::Duration(builder) => Self::Duration(builder.take()),
            Self::Decimal128(builder) => Self::Decimal128(builder.take()),
            Self::Utf8(builder) => Self::Utf8(builder.take()),
            Self::LargeUtf8(builder) => Self::LargeUtf8(builder.take()),
            Self::List(builder) => Self::List(builder.take()),
            Self::LargeList(builder) => Self::LargeList(builder.take()),
            Self::FixedSizedList(builder) => Self::FixedSizedList(builder.take()),
            Self::Binary(builder) => Self::Binary(builder.take()),
            Self::LargeBinary(builder) => Self::LargeBinary(builder.take()),
            Self::FixedSizeBinary(builder) => Self::FixedSizeBinary(builder.take()),
            Self::Struct(builder) => Self::Struct(builder.take()),
            Self::Map(builder) => Self::Map(builder.take()),
            Self::DictionaryUtf8(builder) => Self::DictionaryUtf8(builder.take()),
            Self::Union(builder) => Self::Union(builder.take()),
            Self::UnknownVariant(builder) => Self::UnknownVariant(builder.take()),
        }
    }
}

impl Context for ArrayBuilder {
    fn annotations(&self) -> BTreeMap<String, String> {
        dispatch!(self, Self(builder) => builder.annotations())
    }
}

#[rustfmt::skip]
impl SimpleSerializer for ArrayBuilder {
    fn name(&self) -> &str {
        "ArrayBuilder"
    }

    fn annotate_error(&self, err: Error) -> Error {
        dispatch!(self, Self(builder) => builder.annotate_error(err))
    }

    fn serialize_default(&mut self) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_default().map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_unit_struct(&mut self, name: &'static str) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_unit_struct(name).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_none(&mut self) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_none().map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_some<V: Serialize + ?Sized>(&mut self, value: &V) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_some(value).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_unit(&mut self) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_unit().map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_bool(&mut self, v: bool) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_bool(v).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_i8(&mut self, v: i8) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_i8(v).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_i16(&mut self, v: i16) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_i16(v).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_i32(&mut self, v: i32) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_i32(v).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_i64(&mut self, v: i64) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_i64(v).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_u8(&mut self, v: u8) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_u8(v).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_u16(&mut self, v: u16) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_u16(v).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_u32(&mut self, v: u32) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_u32(v).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_u64(&mut self, v: u64) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_u64(v).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_f32(&mut self, v: f32) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_f32(v).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_f64(&mut self, v: f64) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_f64(v).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_char(&mut self, v: char) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_char(v).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_str(&mut self, v: &str) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_str(v).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_bytes(&mut self, v: &[u8]) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_bytes(v).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_seq_start(&mut self, len: Option<usize>) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_seq_start(len).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_seq_element<V: Serialize + ?Sized>(&mut self, value: &V) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_seq_element(value).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_seq_end(&mut self) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_seq_end().map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_struct_start(&mut self, name: &'static str, len: usize) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_struct_start(name, len).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_struct_field<V: Serialize + ?Sized>(&mut self, key: &'static str, value: &V) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_struct_field(key, value).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_struct_end(&mut self) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_struct_end().map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_map_start(&mut self, len: Option<usize>) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_map_start(len).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_map_key<V: Serialize + ?Sized>(&mut self, key: &V) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_map_key(key).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_map_value<V: Serialize + ?Sized>(&mut self, value: &V) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_map_value(value).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_map_end(&mut self) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_map_end().map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_tuple_start(&mut self, len: usize) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_tuple_start(len).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_tuple_element<V: Serialize + ?Sized>(&mut self, value: &V) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_tuple_element(value).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_tuple_end(&mut self) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_tuple_end().map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_tuple_struct_start(&mut self, name: &'static str, len: usize) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_tuple_struct_start(name, len).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_tuple_struct_field<V: Serialize + ?Sized>(&mut self, value: &V) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_tuple_struct_field(value).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_tuple_struct_end(&mut self) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_tuple_struct_end().map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_newtype_struct<V: Serialize + ?Sized>(&mut self, name: &'static str, value: &V) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_newtype_struct(name, value).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_newtype_variant<V: Serialize + ?Sized>(&mut self, name: &'static str, variant_index: u32, variant: &'static str, value: &V) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_newtype_variant(name, variant_index, variant, value).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_unit_variant(&mut self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<()> {
        dispatch!(self, Self(builder) => builder.serialize_unit_variant(name, variant_index, variant).map_err(|err| builder.annotate_error(err)))
    }

    fn serialize_struct_variant_start<'this>(&'this mut self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<&'this mut ArrayBuilder> {
        let annotations_err = dispatch!(self, Self(builder) => builder.annotate_error(Error::empty()));
        dispatch!(self, Self(builder) => builder.serialize_struct_variant_start(name, variant_index, variant, len).map_err(|err| merge_annotations(err, annotations_err)))
    }

    fn serialize_tuple_variant_start<'this> (&'this mut self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<&'this mut ArrayBuilder> {
        let annotations_err = dispatch!(self, Self(builder) => builder.annotate_error(Error::empty()));
        dispatch!(self, Self(builder) => builder.serialize_tuple_variant_start(name, variant_index, variant, len).map_err(|err| merge_annotations(err, annotations_err)))
    }
}
