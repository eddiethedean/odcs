//! Schema and contract indexes for reference resolution.

use std::collections::{HashMap, HashSet};

use crate::model::{DataContract, SchemaProperty};

/// Index of schema object and property names within a single contract.
#[derive(Debug, Clone, Default)]
pub struct SchemaIndex {
    objects: HashMap<String, HashSet<String>>,
}

impl SchemaIndex {
    /// Build an index from a parsed contract.
    #[must_use]
    pub fn build(contract: &DataContract) -> Self {
        let mut objects = HashMap::new();
        for schema in &contract.schema {
            if let Some(name) = schema.element.name.as_deref() {
                if !name.is_empty() {
                    let mut properties = HashSet::new();
                    collect_property_names(&schema.properties, &mut properties);
                    objects.insert(name.to_string(), properties);
                }
            }
        }
        Self { objects }
    }

    /// Returns `true` when a shorthand reference (`table.column`) resolves.
    #[must_use]
    pub fn resolve_shorthand(&self, reference: &str) -> bool {
        let Some((table, column)) = reference.split_once('.') else {
            return false;
        };
        self.objects
            .get(table)
            .is_some_and(|properties| properties.contains(column))
    }
}

/// Index of contracts by root `id` for cross-file FQN resolution.
#[derive(Debug, Clone, Default)]
pub struct ContractIndex {
    contracts: HashMap<String, SchemaIndex>,
}

impl ContractIndex {
    /// Build an index from multiple contracts keyed by root `id`.
    #[must_use]
    pub fn from_contracts(contracts: &[&DataContract]) -> Self {
        let mut index = Self::default();
        for contract in contracts {
            if !contract.id.is_empty() {
                index
                    .contracts
                    .insert(contract.id.clone(), SchemaIndex::build(contract));
            }
        }
        index
    }

    /// Returns `true` when a fully-qualified reference resolves against the set.
    #[must_use]
    pub fn resolve_fqn(&self, reference: &str) -> bool {
        let Some((contract_id, schema_name, property_name)) = parse_fqn_triple(reference) else {
            return false;
        };
        self.contracts.get(contract_id).is_some_and(|schema_index| {
            schema_index.resolve_shorthand(&format!("{schema_name}.{property_name}"))
        })
    }
}

/// Strip optional URL prefix from a fully-qualified reference.
#[must_use]
pub fn normalize_fqn_path(reference: &str) -> Option<&str> {
    let path = if let Some(idx) = reference.find('#') {
        reference.get(idx + 1..)?
    } else {
        reference
    };
    let path = path.trim_start_matches('/');
    if path.is_empty() {
        None
    } else {
        Some(path)
    }
}

/// Parse `{contractId}/{schemaObject}/{property}` from a fully-qualified reference.
#[must_use]
pub fn parse_fqn_triple(reference: &str) -> Option<(&str, &str, &str)> {
    let path = normalize_fqn_path(reference)?;
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() >= 3 {
        Some((parts[0], parts[1], parts[2]))
    } else {
        None
    }
}

fn collect_property_names(properties: &[SchemaProperty], out: &mut HashSet<String>) {
    for property in properties {
        if let Some(name) = property.element.name.as_deref() {
            if !name.is_empty() {
                out.insert(name.to_string());
            }
        }
        collect_property_names(&property.properties, out);
        if let Some(items) = &property.items {
            collect_property_names(std::slice::from_ref(items), out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fqn_triple_extracts_segments() {
        assert_eq!(
            parse_fqn_triple("provider-contract/customers/customer_id"),
            Some(("provider-contract", "customers", "customer_id"))
        );
        assert_eq!(
            parse_fqn_triple(
                "https://example.com/contracts/foo.yaml#provider/customers/customer_id"
            ),
            Some(("provider", "customers", "customer_id"))
        );
    }
}
