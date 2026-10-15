PLANTUML = plantuml

# Diagrams are built in place, with output placed side-by-side with the source
DIAGRAMS_SRC = $(shell find specs rfcs -type f -name '*.puml')
DIAGRAMS = $(patsubst %.puml, %.svg, $(DIAGRAMS_SRC))

SCHEMAS_SRC = $(shell find schemas/ -type f -name '*.json')
SCHEMAS_UTILS_SRC = $(shell find tools/schemas/ -type f -name '*.rs')

CODEGEN_CMD = RUST_BACKTRACE=1 cargo run -q -- codegen
RUSTFMT = rustfmt --edition 2024 --style-edition 2024


all: lint codegen contracts/erd.svg $(DIAGRAMS)

.PHONY: nix
nix:
	nix develop ./tools/nix


$(DIAGRAMS): %.svg: %.puml
	$(PLANTUML) -tsvg $<

contracts/erd.svg: contracts/erd.mmd
	mmdc -i contracts/erd.mmd -o $@


.PHONY: codegen
codegen:
	$(CODEGEN_CMD) markdown > specs/reference.md
	$(CODEGEN_CMD) flatbuffers-schema > contracts/metadata/opendatafabric.fbs
	$(CODEGEN_CMD) mermaid-erd > contracts/erd.mmd
	$(CODEGEN_CMD) rust-dtos > tools/schemas/output/rust-dtos.rs
	$(CODEGEN_CMD) rust-serde > tools/schemas/output/rust-serde.rs
	$(CODEGEN_CMD) rust-serde-flatbuffers > tools/schemas/output/rust-serde-flatbuffers.rs
	$(CODEGEN_CMD) rust-graphql > tools/schemas/output/rust-graphql.rs
	$(RUSTFMT) tools/schemas/output/rust-dtos.rs
	$(RUSTFMT) tools/schemas/output/rust-serde.rs
	$(RUSTFMT) tools/schemas/output/rust-serde-flatbuffers.rs
	$(RUSTFMT) tools/schemas/output/rust-graphql.rs


.PHONY: lint
lint:
	RUST_BACKTRACE=1 cargo run -q -- lint
	cargo test

