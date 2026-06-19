use std::{collections::HashMap, path::Path};

use jsonschema::{Resource, Validator};
use serde_json::{Value, json};

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct Schemas {
    by_id: HashMap<String, Value>,
}

impl Schemas {
    fn load(schemas_dir: &Path) -> Self {
        let mut by_id = HashMap::new();
        for entry in glob::glob(&format!("{}/**/*.json", schemas_dir.display())).unwrap() {
            let path = entry.unwrap();
            let value: Value =
                serde_json::from_reader(std::fs::File::open(&path).unwrap()).unwrap();
            if let Some(id) = value.get("$id").and_then(Value::as_str) {
                by_id.insert(id.to_string(), value);
            }
        }
        Self { by_id }
    }

    fn validator_for(&self, schema_id: &str) -> Validator {
        let schema = self
            .by_id
            .get(schema_id)
            .unwrap_or_else(|| panic!("Schema not found: {schema_id}"));

        let resources = self.by_id.iter().filter_map(|(id, value)| {
            let resource = Resource::from_contents(value.clone()).ok()?;
            Some((id.clone(), resource))
        });

        jsonschema::options()
            .with_resources(resources)
            .build(schema)
            .unwrap_or_else(|e| panic!("Failed to compile schema {schema_id}: {e}"))
    }

    fn assert_valid(&self, schema_id: &str, instance: Value) {
        let validator = self.validator_for(schema_id);
        let errors: Vec<_> = validator.iter_errors(&instance).collect();
        assert!(
            errors.is_empty(),
            "Expected valid but got errors for {schema_id}:\n{}",
            errors
                .iter()
                .map(|e| format!("  - {e} (path: {})", e.instance_path))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    fn assert_invalid(&self, schema_id: &str, instance: Value) {
        let validator = self.validator_for(schema_id);
        let errors: Vec<_> = validator.iter_errors(&instance).collect();
        assert!(
            !errors.is_empty(),
            "Expected invalid but no errors were reported for {schema_id}"
        );
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn schemas() -> Schemas {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    Schemas::load(&repo_root.join("schemas"))
}

const VARIABLE_SET: &str = "https://opendatafabric.org/schemas/config/v1alpha1/VariableSet";
const DATASET: &str = "https://opendatafabric.org/schemas/dataset/v1alpha1/Dataset";

fn valid_variable_set() -> Value {
    json!({
        "$schema": "https://opendatafabric.org/schemas/config/v1alpha1/VariableSet",
        "headers": { "name": "my-vars" },
        "spec": {
            "variables": {
                "var": {
                    "value": "val"
                }
            }
        }
    })
}

fn valid_dataset() -> Value {
    json!({
        "$schema": "https://opendatafabric.org/schemas/dataset/v1alpha1/Dataset",
        "headers": { "name": "my-dataset" },
        "spec": {
            "kind": "Root",
            "metadata": [
                {
                    "kind": "SetDataSchema",
                    "schema": {
                        "fields": [
                            {
                                "name": "a",
                                // Short form
                                "type": "String"
                            },
                            {
                                "name": "b",
                                // Long form
                                "type": {
                                    "kind": "String"
                                }
                            },
                            {
                               "name": "c",
                               "type": {
                                   "kind": "Timestamp",
                                   "unit": "Millisecond",
                                   "timezone": "UTC"
                               }
                            }
                        ]
                    }
                },
                {
                    "kind": "SetPollingSource",
                    "fetch": { "kind": "Url", "url": "https://example.com/data.csv" },
                    "read": { "kind": "Csv" },
                    "merge": { "kind": "Append" }
                }
            ]
        }
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_valid_variable_set() {
    schemas().assert_valid(VARIABLE_SET, valid_variable_set());
}

#[test]
fn test_valid_dataset() {
    schemas().assert_valid(DATASET, valid_dataset());
}

// Unknown property at the root level of a resource
#[test]
fn test_unknown_root_property() {
    let mut instance = valid_variable_set();
    instance["unknown_field"] = json!("bad");
    schemas().assert_invalid(VARIABLE_SET, instance);
}

// Unknown property inside a nested object property (headers -> ResourceHeaders)
#[test]
fn test_unknown_property_in_headers() {
    let mut instance = valid_variable_set();
    instance["headers"]["unknown_field"] = json!("bad");
    schemas().assert_invalid(VARIABLE_SET, instance);
}

// Unknown property inside a nested spec object
#[test]
fn test_unknown_property_in_spec() {
    let mut instance = valid_variable_set();
    instance["spec"]["unknown_field"] = json!("bad");
    schemas().assert_invalid(VARIABLE_SET, instance);
}

// Unknown property inside patternProperties
#[test]
fn test_unknown_property_in_pattern_properties() {
    let mut instance = valid_variable_set();
    instance["spec"]["variables"]["var"]["unknown_field"] = json!("bad");
    schemas().assert_invalid(VARIABLE_SET, instance);
}

// Unknown property inside a metadata event variant (enum variant in a oneOf)
#[test]
fn test_unknown_property_in_metadata_event() {
    let mut instance = valid_dataset();
    assert_eq!(instance["spec"]["metadata"][1]["kind"], "SetPollingSource");
    instance["spec"]["metadata"][1]["unknown_field"] = json!("bad");
    schemas().assert_invalid(DATASET, instance);
}

// Unknown property inside a deeply nested enum variant property (fetch inside SetDataSchema...Timestamp)
#[test]
fn test_unknown_property_in_nested_enum_variant() {
    let mut instance = valid_dataset();
    assert_eq!(instance["spec"]["metadata"][0]["kind"], "SetDataSchema");
    assert_eq!(
        instance["spec"]["metadata"][0]["schema"]["fields"][2]["type"]["kind"],
        "Timestamp"
    );
    instance["spec"]["metadata"][0]["schema"]["fields"][2]["type"]["unknown_field"] = json!("bad");
    schemas().assert_invalid(DATASET, instance);
}
