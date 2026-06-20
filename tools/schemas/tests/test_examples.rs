use std::{collections::HashMap, path::Path};

use jsonschema::{Resource, Validator};
use serde_json::Value;

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

        // Normalize custom metaschema URIs to the standard draft URI so the
        // jsonschema crate (which only accepts official draft URIs as $schema)
        // can compile schemas that declare a custom ODF metaschema.
        let schema = Self::normalize_meta_schema(schema.clone());

        let resources = self.by_id.iter().filter_map(|(id, value)| {
            let resource = Resource::from_contents(Self::normalize_meta_schema(value.clone())).ok()?;
            Some((id.clone(), resource))
        });

        jsonschema::options()
            .with_resources(resources)
            .build(&schema)
            .unwrap_or_else(|e| panic!("Failed to compile schema {schema_id}: {e}"))
    }

    fn normalize_meta_schema(mut schema: Value) -> Value {
        const STANDARD_DRAFT: &str = "https://json-schema.org/draft/2020-12/schema";
        const ODF_METASCHEMA_PREFIX: &str =
            "https://opendatafabric.org/schemas/metaschemas/";
        if let Some(s) = schema.get("$schema").and_then(Value::as_str) {
            if s.starts_with(ODF_METASCHEMA_PREFIX) {
                schema["$schema"] = Value::String(STANDARD_DRAFT.to_string());
            }
        }
        schema
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_examples() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let schemas = Schemas::load(&repo_root.join("schemas"));

    let examples_dir = repo_root.join("examples");
    let yaml_files: Vec<_> = glob::glob(&format!("{}/**/*.yaml", examples_dir.display()))
        .unwrap()
        .map(|e| e.unwrap())
        .collect();

    assert!(
        !yaml_files.is_empty(),
        "No YAML example files found under {}",
        examples_dir.display()
    );

    let mut failed = false;

    for path in &yaml_files {
        let content = std::fs::read_to_string(path).unwrap();
        let value: Value = serde_yaml::from_str(&content)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {e}", path.display()));

        let schema_id = value["$schema"]
            .as_str()
            .unwrap_or_else(|| panic!("Missing $schema field in {}", path.display()));

        let validator = schemas.validator_for(schema_id);

        let errors: Vec<_> = validator.iter_errors(&value).collect();
        if !errors.is_empty() {
            failed = true;
            eprintln!(
                "Validation failed for {} ($schema: {schema_id}):",
                path.display()
            );
            for err in &errors {
                eprintln!("  - {err} (path: {})", err.instance_path);
            }
        }
    }

    assert!(!failed, "Some example files failed validation");
}
