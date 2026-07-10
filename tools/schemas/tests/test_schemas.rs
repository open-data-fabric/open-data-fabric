use std::{collections::HashMap, path::Path};

use jsonschema::{Resource, Validator};
use serde_json::Value;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

const STANDARD_DRAFT: &str = "https://json-schema.org/draft/2020-12/schema";
const ODF_METASCHEMA_PREFIX: &str = "https://opendatafabric.org/schemas/metaschemas/";

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

    // Builds a validator for the given metaschema ID. Metaschemas themselves
    // always use the standard draft URI so no normalization is needed here.
    fn validator_for_metaschema(&self, metaschema_id: &str) -> Validator {
        let metaschema = self
            .by_id
            .get(metaschema_id)
            .unwrap_or_else(|| panic!("Metaschema not found: {metaschema_id}"));

        let resources = self.by_id.iter().filter_map(|(id, value)| {
            // Schemas with a custom ODF $schema fail Resource::from_contents with
            // UnknownSpecification. Strip $schema so the library treats them as
            // standard-draft documents and registers them in the resource registry
            // (needed for $ref resolution during metaschema compilation).
            let mut value = value.clone();
            if let Some(schema) = value.get("$schema").and_then(Value::as_str) {
                if schema.starts_with(ODF_METASCHEMA_PREFIX) {
                    value.as_object_mut()?.remove("$schema");
                }
            }
            let resource = Resource::from_contents(value).ok()?;
            Some((id.clone(), resource))
        });

        jsonschema::options()
            .with_resources(resources)
            .build(metaschema)
            .unwrap_or_else(|e| panic!("Failed to compile metaschema {metaschema_id}: {e}"))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_schemas() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let schemas = Schemas::load(&repo_root.join("schemas"));

    // Build validators keyed by metaschema ID. Standard draft is represented
    // as None (the jsonschema crate validates against it implicitly when
    // compiling schemas, so we skip explicit validation for those).
    let mut validators: HashMap<String, Validator> = HashMap::new();
    for schema in schemas.by_id.values() {
        let meta = schema
            .get("$schema")
            .and_then(Value::as_str)
            .unwrap_or(STANDARD_DRAFT);
        if meta.starts_with(ODF_METASCHEMA_PREFIX) && !validators.contains_key(meta) {
            validators.insert(meta.to_string(), schemas.validator_for_metaschema(meta));
        }
    }

    let mut failed = false;

    for (schema_id, schema) in &schemas.by_id {
        let meta = schema
            .get("$schema")
            .and_then(Value::as_str)
            .unwrap_or(STANDARD_DRAFT);

        // Only validate schemas that declare an ODF metaschema — standard
        // draft conformance is already guaranteed by the jsonschema crate.
        if !meta.starts_with(ODF_METASCHEMA_PREFIX) {
            continue;
        }

        let validator = validators
            .get(meta)
            .unwrap_or_else(|| panic!("No validator for metaschema {meta}"));

        let errors: Vec<_> = validator.iter_errors(schema).collect();
        if !errors.is_empty() {
            failed = true;
            eprintln!("Schema {schema_id} failed validation against metaschema {meta}:");
            for err in &errors {
                eprintln!("  - {err} (path: {})", err.instance_path);
            }
        }
    }

    assert!(!failed, "Some schemas failed metaschema validation");
}
