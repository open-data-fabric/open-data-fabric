PLANTUML = plantuml

MDTPL = tools/template_markdown.py

DIAGRAMS_SRC = $(shell find src/images/ -type f -name '*.puml')
DIAGRAMS = $(subst src/images,images,$(patsubst %.puml, %.svg, $(DIAGRAMS_SRC)))

DIAGRAMS_SRC_RAW = $(shell find src/images/ -type f -name '*.svg')
DIAGRAMS_RAW = $(subst src/images,images,$(DIAGRAMS_SRC_RAW))

SCHEMAS_SRC = $(shell find schemas/ -type f -name '*.json')
SCHEMAS_UTILS_SRC = $(shell find tools/schemas/ -type f -name '*.rs')

CODEGEN_CMD = RUST_BACKTRACE=1 cargo run -q -- codegen
RUSTFMT = rustfmt --edition 2024 --style-edition 2024


all: lint codegen $(SCHEMA_MERMAID_ERD_SVG) $(DIAGRAMS) $(DIAGRAMS_RAW) open-data-fabric.md

.PHONY: nix
nix:
	nix develop ./tools/nix


$(DIAGRAMS): images/%.svg: src/images/%.puml
	$(PLANTUML) -o ../../images -tsvg $^

$(DIAGRAMS_RAW): images/%.svg: src/images/%.svg
	cp $^ $@

schemas-generated/mermaid/erd.svg: schemas-generated/mermaid/erd.mmd
	mmdc -i schemas-generated/mermaid/erd.mmd -o $@


.PHONY: codegen
codegen:
	@mkdir -p build/
	$(CODEGEN_CMD) markdown > build/metadata-reference.md
	$(CODEGEN_CMD) flatbuffers-schema > schemas-generated/flatbuffers/opendatafabric.fbs
	$(CODEGEN_CMD) mermaid-erd > schemas-generated/mermaid/erd.mmd
	$(CODEGEN_CMD) rust-dtos > tools/schemas/output/rust-dtos.rs
	$(CODEGEN_CMD) rust-serde > tools/schemas/output/rust-serde.rs
	$(CODEGEN_CMD) rust-serde-flatbuffers > tools/schemas/output/rust-serde-flatbuffers.rs
	$(CODEGEN_CMD) rust-graphql > tools/schemas/output/rust-graphql.rs
	$(RUSTFMT) tools/schemas/output/rust-dtos.rs
	$(RUSTFMT) tools/schemas/output/rust-serde.rs
	$(RUSTFMT) tools/schemas/output/rust-serde-flatbuffers.rs
	$(RUSTFMT) tools/schemas/output/rust-graphql.rs


open-data-fabric.md: src/open-data-fabric.md $(SCHEMA_MARKDOWN)
	$(MDTPL) src/open-data-fabric.md open-data-fabric.md
	@# Dependency: nodejs-markdown-toc (npm install -g markdown-toc)
	markdown-toc --maxdepth 2 -i open-data-fabric.md


.PHONY: lint
lint:
	RUST_BACKTRACE=1 cargo run -q -- lint
	cargo test


.PHONY: clean
clean:
	rm -rf build/ open-data-fabric.md
