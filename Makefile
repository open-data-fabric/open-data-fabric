ifeq (, $(shell which podman 2>/dev/null))
RUNTIME = docker
else
RUNTIME = podman
endif

PLANTUML_IMG = "docker.io/plantuml/plantuml:1.2022.14"
PLANTUML = $(RUNTIME) run --rm -v $(PWD):/opt/workdir --workdir /opt/workdir $(PLANTUML_IMG)

MDTPL = tools/template_markdown.py

DIAGRAMS_SRC = $(shell find src/images/ -type f -name '*.puml')
DIAGRAMS = $(subst src/images,images,$(patsubst %.puml, %.svg, $(DIAGRAMS_SRC)))

DIAGRAMS_SRC_RAW = $(shell find src/images/ -type f -name '*.svg')
DIAGRAMS_RAW = $(subst src/images,images,$(DIAGRAMS_SRC_RAW))

SCHEMAS_SRC = $(shell find schemas/ -type f -name '*.json')
SCHEMAS_UTILS_SRC = $(shell find tools/schemas/ -type f -name '*.rs')

SCHEMA_MARKDOWN = build/metadata-reference.md
SCHEMA_FLATBUFFERS = schemas-generated/flatbuffers/opendatafabric.fbs

CODEGEN_CMD = RUST_BACKTRACE=1 cargo run -q -- codegen
RUSTFMT = rustfmt --edition 2024 --style-edition 2024

all: build/ $(DIAGRAMS) $(DIAGRAMS_RAW) schema-lint codegen $(SCHEMA_MARKDOWN) open-data-fabric.md

build/:
	mkdir -p build/

$(DIAGRAMS): images/%.svg: src/images/%.puml
	$(PLANTUML) -o ../../images -tsvg $^

$(DIAGRAMS_RAW): images/%.svg: src/images/%.svg
	cp $^ $@

schema-lint: $(SCHEMAS_SRC) $(SCHEMAS_UTILS_SRC)
	RUST_BACKTRACE=1 cargo run -q -- lint

$(SCHEMA_MARKDOWN): $(SCHEMAS_SRC) $(SCHEMAS_UTILS_SRC)
	$(CODEGEN_CMD) markdown > $@

$(SCHEMA_FLATBUFFERS): $(SCHEMAS_SRC) $(SCHEMAS_UTILS_SRC)
	$(CODEGEN_CMD) flatbuffers-schema > $@

.PHONY: codegen
codegen:
	$(CODEGEN_CMD) markdown > $(SCHEMA_MARKDOWN)
	$(CODEGEN_CMD) flatbuffers-schema > $(SCHEMA_FLATBUFFERS)
	$(CODEGEN_CMD) rust-dtos > tools/schemas/output/rust-dtos.rs
	$(CODEGEN_CMD) rust-serde > tools/schemas/output/rust-serde.rs
	$(CODEGEN_CMD) rust-serde-flatbuffers > tools/schemas/output/rust-serde-flatbuffers.rs
	$(CODEGEN_CMD) rust-graphql > tools/schemas/output/rust-graphql.rs
	$(RUSTFMT) tools/schemas/output/rust-dtos.rs
	$(RUSTFMT) tools/schemas/output/rust-serde.rs
	$(RUSTFMT) tools/schemas/output/rust-serde-flatbuffers.rs
	$(RUSTFMT) tools/schemas/output/rust-graphql.rs


.PHONY: test-examples
test-examples:
	cargo test


open-data-fabric.md: src/open-data-fabric.md $(SCHEMA_MARKDOWN)
	$(MDTPL) src/open-data-fabric.md open-data-fabric.md
	@# Dependency: nodejs-markdown-toc (npm install -g markdown-toc)
	markdown-toc --maxdepth 2 -i open-data-fabric.md


.PHONY: lint
lint:
	RUST_BACKTRACE=1 cargo run -q -- lint


.PHONY: clean
clean:
	rm -rf build/ open-data-fabric.md


# Image with all tools pre-installed
.PHONY: image
image:
	cd tools/image && $(RUNTIME) build -t odf-dev .

.PHONY: all-in-container
all-in-container:
	$(RUNTIME) run --rm -v "$(PWD)":/workspace odf-dev make