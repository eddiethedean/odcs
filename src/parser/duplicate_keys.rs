//! Duplicate key detection for JSON and YAML documents.

use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt;
use std::mem::MaybeUninit;
use std::rc::Rc;
use std::slice;

use serde::de::{self, DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};
#[allow(clippy::unsafe_removed_from_name)]
use unsafe_libyaml as sys;

/// A duplicate mapping key and its location in the document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicateKeyFinding {
    /// The duplicated key name.
    pub key: String,
    /// JSON-path-style reference (e.g. `schema[0].name`).
    pub object_ref: String,
}

/// Returns the first duplicate key found in a JSON document, if any.
pub fn find_json_duplicate_key(content: &[u8]) -> Option<DuplicateKeyFinding> {
    let finding = Rc::new(RefCell::new(None));
    let mut de = serde_json::Deserializer::from_slice(content);
    let visitor = DupeDetectVisitor {
        finding: Rc::clone(&finding),
        path: Vec::new(),
    };
    if de.deserialize_any(visitor).is_err() {
        return finding.borrow().clone();
    }
    None
}

/// Returns the first duplicate key found in a YAML document, if any.
///
/// Uses an `unsafe-libyaml` event walk so duplicates are detected before
/// `serde_yaml` deserialization (which silently overwrites duplicate keys).
/// Flow-style mappings and anchor/alias resolution are not fully validated.
pub fn find_yaml_duplicate_key(content: &str) -> Option<DuplicateKeyFinding> {
    let bytes = content.as_bytes();
    let mut parser = MaybeUninit::<sys::yaml_parser_t>::uninit();

    unsafe {
        let parser = parser.as_mut_ptr();
        if sys::yaml_parser_initialize(parser).fail {
            return None;
        }

        struct ParserGuard(*mut sys::yaml_parser_t);
        impl Drop for ParserGuard {
            fn drop(&mut self) {
                unsafe {
                    sys::yaml_parser_delete(self.0);
                }
            }
        }
        let _guard = ParserGuard(parser);

        sys::yaml_parser_set_encoding(parser, sys::YAML_UTF8_ENCODING);
        sys::yaml_parser_set_input_string(parser, bytes.as_ptr(), bytes.len() as u64);

        let mut state = YamlDupeState::default();
        let mut event = MaybeUninit::<sys::yaml_event_t>::uninit();

        loop {
            let event_ptr = event.as_mut_ptr();
            if sys::yaml_parser_parse(parser, event_ptr).fail {
                return None;
            }

            let event_type = (*event_ptr).type_;
            if let Some(finding) = state.handle_event(event_ptr, event_type) {
                sys::yaml_event_delete(event_ptr);
                return Some(finding);
            }

            sys::yaml_event_delete(event_ptr);
            if event_type == sys::YAML_STREAM_END_EVENT {
                break;
            }
        }
    }

    None
}

fn format_path(segments: &[String]) -> String {
    let mut result = String::new();
    for segment in segments {
        if !segment.starts_with('[') && !result.is_empty() {
            result.push('.');
        }
        result.push_str(segment);
    }
    result
}

fn object_ref(path: &[String], key: &str) -> String {
    let base = format_path(path);
    if base.is_empty() {
        key.to_string()
    } else {
        format!("{base}.{key}")
    }
}

struct DupeDetectVisitor {
    finding: Rc<RefCell<Option<DuplicateKeyFinding>>>,
    path: Vec<String>,
}

impl DupeDetectVisitor {
    fn record_duplicate(&self, key: String) {
        *self.finding.borrow_mut() = Some(DuplicateKeyFinding {
            object_ref: object_ref(&self.path, &key),
            key,
        });
    }
}

impl<'de> Visitor<'de> for DupeDetectVisitor {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("JSON value")
    }

    fn visit_map<M>(mut self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut keys = HashSet::new();
        while let Some(key) = access.next_key::<String>()? {
            if !keys.insert(key.clone()) {
                self.record_duplicate(key);
                return Err(de::Error::custom("duplicate key detected"));
            }

            self.path.push(key);
            access.next_value_seed(DupeDetectSeed {
                finding: Rc::clone(&self.finding),
                path: self.path.clone(),
            })?;
            self.path.pop();
        }
        Ok(())
    }

    fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let mut index = 0usize;
        while access
            .next_element_seed(DupeDetectSeed {
                finding: Rc::clone(&self.finding),
                path: {
                    let mut path = self.path.clone();
                    path.push(format!("[{index}]"));
                    path
                },
            })?
            .is_some()
        {
            index += 1;
        }
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
        DeserializeSeed::deserialize(
            DupeDetectSeed {
                finding: Rc::clone(&self.finding),
                path: self.path,
            },
            deserializer,
        )
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(())
    }
}

struct DupeDetectSeed {
    finding: Rc<RefCell<Option<DuplicateKeyFinding>>>,
    path: Vec<String>,
}

impl<'de> DeserializeSeed<'de> for DupeDetectSeed {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DupeDetectVisitor {
            finding: self.finding,
            path: self.path,
        })
    }
}

#[derive(Default)]
struct YamlDupeState {
    frames: Vec<YamlFrame>,
    path: Vec<String>,
    pending_key: Option<String>,
}

enum YamlFrame {
    Mapping {
        keys_seen: HashSet<String>,
        expect_key: bool,
    },
    Sequence {
        next_index: usize,
    },
}

impl YamlDupeState {
    fn handle_event(
        &mut self,
        event: *const sys::yaml_event_t,
        event_type: sys::yaml_event_type_t,
    ) -> Option<DuplicateKeyFinding> {
        match event_type {
            sys::YAML_MAPPING_START_EVENT => self.on_mapping_start(),
            sys::YAML_MAPPING_END_EVENT => self.on_mapping_end(),
            sys::YAML_SEQUENCE_START_EVENT => self.on_sequence_start(),
            sys::YAML_SEQUENCE_END_EVENT => self.on_sequence_end(),
            sys::YAML_SCALAR_EVENT => self.on_scalar(event),
            sys::YAML_ALIAS_EVENT => self.on_alias(),
            _ => None,
        }
    }

    fn on_mapping_start(&mut self) -> Option<DuplicateKeyFinding> {
        if let Some(key) = self.pending_key.take() {
            self.path.push(key);
        }

        if matches!(self.frames.last(), Some(YamlFrame::Sequence { .. })) {
            if let Some(YamlFrame::Sequence { next_index }) = self.frames.last_mut() {
                self.path.push(format!("[{next_index}]"));
                *next_index += 1;
            }
        }

        self.frames.push(YamlFrame::Mapping {
            keys_seen: HashSet::new(),
            expect_key: true,
        });
        None
    }

    fn on_mapping_end(&mut self) -> Option<DuplicateKeyFinding> {
        self.frames.pop()?;

        if matches!(self.frames.last(), Some(YamlFrame::Mapping { .. })) {
            self.set_parent_mapping_expect_key(true);
        }

        if matches!(self.frames.last(), Some(YamlFrame::Sequence { .. })) {
            if self
                .path
                .last()
                .is_some_and(|segment| segment.starts_with('['))
            {
                self.path.pop();
            }
        } else if self.pending_key.is_none() {
            self.path.pop();
        }

        None
    }

    fn on_sequence_start(&mut self) -> Option<DuplicateKeyFinding> {
        if let Some(key) = self.pending_key.take() {
            self.path.push(key);
        }

        self.set_parent_mapping_expect_key(false);
        self.frames.push(YamlFrame::Sequence { next_index: 0 });
        None
    }

    fn on_sequence_end(&mut self) -> Option<DuplicateKeyFinding> {
        self.frames.pop()?;

        if self
            .path
            .last()
            .is_some_and(|segment| !segment.starts_with('['))
        {
            self.path.pop();
        }

        self.set_parent_mapping_expect_key(true);
        None
    }

    fn on_scalar(&mut self, event: *const sys::yaml_event_t) -> Option<DuplicateKeyFinding> {
        let value = unsafe { scalar_value(event) }?;

        let Some(YamlFrame::Mapping {
            keys_seen,
            expect_key,
        }) = self.frames.last_mut()
        else {
            return None;
        };

        if *expect_key {
            if !keys_seen.insert(value.clone()) {
                return Some(DuplicateKeyFinding {
                    key: value.clone(),
                    object_ref: object_ref(&self.path, &value),
                });
            }
            self.pending_key = Some(value);
            *expect_key = false;
            return None;
        }

        *expect_key = true;
        None
    }

    fn on_alias(&mut self) -> Option<DuplicateKeyFinding> {
        if matches!(self.frames.last(), Some(YamlFrame::Mapping { expect_key, .. }) if !*expect_key)
        {
            self.set_parent_mapping_expect_key(true);
        }
        None
    }

    fn set_parent_mapping_expect_key(&mut self, expect_key: bool) {
        if let Some(YamlFrame::Mapping {
            expect_key: parent_expect_key,
            ..
        }) = self.frames.last_mut()
        {
            *parent_expect_key = expect_key;
        }
    }
}

unsafe fn scalar_value(event: *const sys::yaml_event_t) -> Option<String> {
    let event = &*event;
    let ptr = event.data.scalar.value;
    if ptr.is_null() {
        return Some(String::new());
    }
    let len = event.data.scalar.length as usize;
    let bytes = slice::from_raw_parts(ptr, len);
    std::str::from_utf8(bytes).ok().map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_YAML: &str = r#"
version: "1.0.0"
apiVersion: "v3.1.0"
kind: "DataContract"
id: "example"
status: "draft"
"#;

    #[test]
    fn yaml_valid_document_has_no_duplicate_keys() {
        assert!(find_yaml_duplicate_key(MINIMAL_YAML).is_none());
    }

    #[test]
    fn yaml_root_duplicate_key() {
        let yaml = r#"
id: first
id: second
"#;
        let finding = find_yaml_duplicate_key(yaml).expect("duplicate");
        assert_eq!(finding.key, "id");
        assert_eq!(finding.object_ref, "id");
    }

    #[test]
    fn yaml_nested_duplicate_key() {
        let yaml = r#"
schema:
  - name: customers
    name: duplicate
"#;
        let finding = find_yaml_duplicate_key(yaml).expect("duplicate");
        assert_eq!(finding.key, "name");
        assert_eq!(finding.object_ref, "schema[0].name");
    }

    #[test]
    fn json_nested_duplicate_key() {
        let json = br#"{"schema":[{"name":"customers","name":"duplicate"}]}"#;
        let finding = find_json_duplicate_key(json).expect("duplicate");
        assert_eq!(finding.key, "name");
        assert_eq!(finding.object_ref, "schema[0].name");
    }
}
