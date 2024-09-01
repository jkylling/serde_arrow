use chrono::{Duration, NaiveDate, NaiveDateTime};
use serde::de::Visitor;

use crate::internal::{
    arrow::BitsWithOffset,
    error::{Context, Result},
    utils::{btree_map, Mut},
};

use super::{simple_deserializer::SimpleDeserializer, utils::ArrayBufferIterator};

pub struct Date32Deserializer<'a> {
    path: String,
    array: ArrayBufferIterator<'a, i32>,
}

impl<'a> Date32Deserializer<'a> {
    pub fn new(path: String, buffer: &'a [i32], validity: Option<BitsWithOffset<'a>>) -> Self {
        Self {
            path,
            array: ArrayBufferIterator::new(buffer, validity),
        }
    }

    pub fn get_string_repr(&self, ts: i32) -> Result<String> {
        const UNIX_EPOCH: NaiveDate = NaiveDateTime::UNIX_EPOCH.date();
        #[allow(deprecated)]
        let delta = Duration::days(ts as i64);
        let date = UNIX_EPOCH + delta;
        Ok(date.to_string())
    }
}

impl<'de> Context for Date32Deserializer<'de> {
    fn annotations(&self) -> std::collections::BTreeMap<String, String> {
        btree_map!("path" => self.path.clone(), "data_type" => "Date32")
    }
}

impl<'de> SimpleDeserializer<'de> for Date32Deserializer<'de> {
    fn name() -> &'static str {
        "Date32Deserializer"
    }

    fn deserialize_any<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value> {
        if self.array.peek_next()? {
            self.deserialize_i32(visitor)
        } else {
            self.array.consume_next();
            visitor.visit_none()
        }
    }

    fn deserialize_option<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value> {
        if self.array.peek_next()? {
            visitor.visit_some(Mut(self))
        } else {
            self.array.consume_next();
            visitor.visit_none()
        }
    }

    fn deserialize_i32<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value> {
        visitor.visit_i32(self.array.next_required()?)
    }

    fn deserialize_str<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value> {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value> {
        let ts = self.array.next_required()?;
        visitor.visit_string(self.get_string_repr(ts)?)
    }
}
