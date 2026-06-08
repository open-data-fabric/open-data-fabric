use std::{collections::HashMap, path::Path, sync::Arc};

use jsonschema::{Retrieve, Uri, Validator};
use serde_json::Value;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct Schemas {
    by_id: Arc<HashMap<String, Value>>,
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
        Self {
            by_id: Arc::new(by_id),
        }
    }

    fn validator_for(&self, schema_id: &str) -> Validator {
        let schema = self
            .by_id
            .get(schema_id)
            .unwrap_or_else(|| panic!("Schema not found: {schema_id}"));

        let retriever = ArcRetriever(Arc::clone(&self.by_id));
        jsonschema::options()
            .with_retriever(retriever)
            .build(schema)
            .unwrap_or_else(|e| panic!("Failed to compile schema {schema_id}: {e}"))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct ArcRetriever(Arc<HashMap<String, Value>>);

impl Retrieve for ArcRetriever {
    fn retrieve(
        &self,
        uri: &Uri<&str>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let uri_str = uri.as_str();
        self.0
            .get(uri_str)
            .cloned()
            .ok_or_else(|| format!("Schema not found: {uri_str}").into())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

const SCHEMA_BASE: &str = "http://open-data-fabric.github.com/schemas/";

#[test]
fn test_examples() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let schemas = Schemas::load(&repo_root.join("schemas"));
    let resource_validator = schemas.validator_for(&format!("{SCHEMA_BASE}Resource"));

    let examples_dir = repo_root.join("examples");
    let yaml_files: Vec<_> =
        glob::glob(&format!("{}/**/*.yaml", examples_dir.display()))
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

        // Validate the top-level Resource envelope
        let errors: Vec<_> = resource_validator.iter_errors(&value).collect();
        if !errors.is_empty() {
            failed = true;
            eprintln!("Resource validation failed for {}:", path.display());
            for err in &errors {
                eprintln!("  - {err} (path: {})", err.instance_path);
            }
            continue;
        }

        // Validate spec against the kind-specific schema
        let kind = value["kind"].as_str().unwrap();
        let spec = &value["spec"];
        let spec_schema_id = format!("{SCHEMA_BASE}{kind}");
        let spec_validator = schemas.validator_for(&spec_schema_id);

        let errors: Vec<_> = spec_validator.iter_errors(spec).collect();
        if !errors.is_empty() {
            failed = true;
            eprintln!(
                "Spec validation failed for {} (kind: {kind}):",
                path.display()
            );
            for err in &errors {
                eprintln!("  - {err} (path: {})", err.instance_path);
            }
        }
    }

    assert!(!failed, "Some example files failed validation");
}
