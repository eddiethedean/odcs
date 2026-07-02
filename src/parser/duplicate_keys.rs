//! Duplicate key detection for JSON and YAML documents.

use std::collections::HashSet;
use std::fmt;

use serde::de::{self, DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};

/// Returns the first duplicate key found in a JSON document, if any.
pub fn find_json_duplicate_key(content: &[u8]) -> Option<String> {
    let mut de = serde_json::Deserializer::from_slice(content);
    let result = de.deserialize_any(DupeDetectVisitor);
    match result {
        Err(err) => {
            let message = err.to_string();
            if message.starts_with("duplicate key: ") {
                Some(message.trim_start_matches("duplicate key: ").to_string())
            } else {
                None
            }
        }
        Ok(()) => None,
    }
}

struct DupeDetectVisitor;

impl<'de> DeserializeSeed<'de> for DupeDetectVisitor {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DupeDetectVisitor)
    }
}

impl<'de> Visitor<'de> for DupeDetectVisitor {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("JSON value")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut keys = HashSet::new();
        while let Some(key) = access.next_key::<String>()? {
            if !keys.insert(key.clone()) {
                return Err(de::Error::custom(format!("duplicate key: {key}")));
            }
            access.next_value_seed(DupeDetectVisitor)?;
        }
        Ok(())
    }

    fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        while access.next_element_seed(DupeDetectVisitor)?.is_some() {}
        Ok(())
    }

    fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_string<E>(self, _: String) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        DeserializeSeed::deserialize(DupeDetectVisitor, deserializer)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(())
    }
}

/// Returns the first duplicate root-level YAML key found, if any.
pub fn find_yaml_root_duplicate_key(content: &str) -> Option<String> {
    let mut keys = HashSet::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if line.starts_with(' ') || line.starts_with('\t') {
            break;
        }
        let key = trimmed
            .split_once(':')
            .map(|(key, _)| key.trim().trim_matches(['"', '\'']).to_string())?;
        if key.is_empty() {
            continue;
        }
        if !keys.insert(key.clone()) {
            return Some(key);
        }
    }
    None
}
