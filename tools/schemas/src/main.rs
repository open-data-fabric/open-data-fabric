use std::path::{Path, PathBuf};

use clap::Parser;
use odf_schemas::{cli, codegen, json_schema, model};

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let args = odf_schemas::cli::Cli::parse();
    let schemas_dir = args.schemas_dir.unwrap_or(PathBuf::from("schemas/"));
    match args.command {
        cli::Command::Lint(cmd) => lint(cmd, &schemas_dir),
        cli::Command::Codegen(cmd) => codegen(cmd, &schemas_dir),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn lint(_cmd: cli::Lint, schemas_dir: &Path) {
    let schemas = json_schema::load_schemas(schemas_dir);

    // TODO: Replace concrete names with a directory structure
    json_schema::check_referential_integrity(
        &schemas,
        &[
            "Manifest".to_string(),
            "DatasetSnapshot".to_string(),
            "MetadataBlock".to_string(),
            "RawQueryRequest".to_string(),
            "RawQueryResponse".to_string(),
            "TransformRequest".to_string(),
            "TransformResponse".to_string(),
        ],
    );

    let model = model::parse_jsonschema(schemas);

    for t in model.types.values() {
        eprintln!("{}", t.id().join("::"));
    }
    eprintln!("Successfully linted {} types", model.types.len());
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn codegen(cmd: cli::Codegen, schemas_dir: &Path) {
    let schemas = json_schema::load_schemas(schemas_dir);
    let model = model::parse_jsonschema(schemas);
    let mut w = std::io::BufWriter::new(std::io::stdout());

    match cmd.language {
        cli::CodegenLang::FlatbuffersSchema => {
            codegen::flatbuffers_schema::render(model, &mut w).unwrap();
        }
        cli::CodegenLang::Markdown => {
            codegen::markdown::render(model, &mut w).unwrap();
        }
        cli::CodegenLang::RustDtos => {
            codegen::rust_dtos::render(model, &mut w).unwrap();
        }
        cli::CodegenLang::RustGraphql => {
            codegen::rust_graphql::render(model, &mut w).unwrap();
        }
        cli::CodegenLang::RustSerde => {
            codegen::rust_serde::render(model, &mut w).unwrap();
        }
        cli::CodegenLang::RustSerdeFlatbuffers => {
            codegen::rust_serde_flatbuffers::render(model, &mut w).unwrap();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
