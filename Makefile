MDTPL = tools/template_markdown.py

DIAGRAMS_SRC = $(wildcard src/images/*.puml)
DIAGRAMS = $(subst src/images,images,$(patsubst %.puml, %.svg, $(DIAGRAMS_SRC)))

DIAGRAMS_SRC_RAW = $(wildcard src/images/*.svg)
DIAGRAMS_RAW = $(subst src/images,images,$(DIAGRAMS_SRC_RAW))

SCHEMAS_SRC = $(wildcard schemas/**/*.json)

SCHEMA_MARKDOWN = build/metadata-reference.md
SCHEMA_MARKDOWNC = python tools/jsonschema_to_markdown.py schemas/

SCHEMA_FLATBUFFERS = schemas-generated/flatbuffers/opendatafabric.fbs
SCHEMA_FLATBUFFERSC = python tools/jsonschema_to_flatbuffers.py schemas/

all: build/ $(DIAGRAMS) $(DIAGRAMS_RAW) $(SCHEMA_MARKDOWN) $(SCHEMA_FLATBUFFERS) open-data-fabric.md

build/:
	mkdir -p build/

$(DIAGRAMS): images/%.svg: src/images/%.puml
	plantuml -o ../../images -tsvg $^

$(DIAGRAMS_RAW): images/%.svg: src/images/%.svg
	cp $^ $@

$(SCHEMA_MARKDOWN): $(SCHEMAS_SRC) tools/jsonschema_to_markdown.py
	$(SCHEMA_MARKDOWNC) > $@

$(SCHEMA_FLATBUFFERS): $(SCHEMAS_SRC) tools/jsonschema_to_flatbuffers.py
	$(SCHEMA_FLATBUFFERSC) > $@

open-data-fabric.md: src/open-data-fabric.md $(SCHEMA_MARKDOWN)
	$(MDTPL) src/open-data-fabric.md open-data-fabric.md
	@# Dependency: nodejs-markdown-toc
	markdown-toc --maxdepth 2 -i open-data-fabric.md

clean:
	rm -rf build/ open-data-fabric.md