use serde::de::{DeserializeSeed, MapAccess, Visitor};

use crate::internal::{
    arrow::BitsWithOffset,
    error::{fail, Context, Error, Result},
    utils::{btree_map, Mut},
};

use super::{
    array_deserializer::ArrayDeserializer,
    simple_deserializer::SimpleDeserializer,
    utils::{bitset_is_set, check_supported_list_layout},
};

pub struct MapDeserializer<'a> {
    path: String,
    key: Box<ArrayDeserializer<'a>>,
    value: Box<ArrayDeserializer<'a>>,
    offsets: &'a [i32],
    validity: Option<BitsWithOffset<'a>>,
    next: (usize, usize),
}

impl<'a> MapDeserializer<'a> {
    pub fn new(
        path: String,
        key: ArrayDeserializer<'a>,
        value: ArrayDeserializer<'a>,
        offsets: &'a [i32],
        validity: Option<BitsWithOffset<'a>>,
    ) -> Result<Self> {
        check_supported_list_layout(validity, offsets)?;

        Ok(Self {
            path,
            key: Box::new(key),
            value: Box::new(value),
            offsets,
            validity,
            next: (0, 0),
        })
    }

    pub fn peek_next(&self) -> Result<bool> {
        if self.next.0 + 1 >= self.offsets.len() {
            fail!("Exhausted ListDeserializer")
        }
        if let Some(validity) = &self.validity {
            Ok(bitset_is_set(validity, self.next.0)?)
        } else {
            Ok(true)
        }
    }

    pub fn consume_next(&mut self) {
        self.next = (self.next.0 + 1, 0);
    }
}

impl<'de> Context for MapDeserializer<'de> {
    fn annotations(&self) -> std::collections::BTreeMap<String, String> {
        btree_map!("path" => self.path.clone(), "data_type" => "Map(..)")
    }
}

impl<'de> SimpleDeserializer<'de> for MapDeserializer<'de> {
    fn name() -> &'static str {
        "MapDeserializer"
    }

    fn deserialize_any<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value> {
        if self.peek_next()? {
            self.deserialize_map(visitor)
        } else {
            self.consume_next();
            visitor.visit_none()
        }
    }

    fn deserialize_option<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value> {
        if self.peek_next()? {
            visitor.visit_some(Mut(self))
        } else {
            self.consume_next();
            visitor.visit_none()
        }
    }

    fn deserialize_map<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value> {
        visitor.visit_map(self)
    }
}

impl<'de> MapAccess<'de> for MapDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        let (item, entry) = self.next;
        if item + 1 >= self.offsets.len() {
            fail!("Exhausted MapDeserializer");
        }
        let start: usize = self.offsets[item].try_into()?;
        let end: usize = self.offsets[item + 1].try_into()?;

        if entry >= (end - start) {
            self.next = (item + 1, 0);
            return Ok(None);
        }
        let res = seed.deserialize(Mut(self.key.as_mut()))?;
        Ok(Some(res))
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        let (item, entry) = self.next;
        self.next = (item, entry + 1);
        seed.deserialize(Mut(self.value.as_mut()))
    }
}
