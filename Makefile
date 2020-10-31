BUILD_DIR = /docs

SCHEMAC = tools/jsonschema_to_markdown.py
MDTPL = tools/template_markdown.py

DIAGRAMS_SRC = $(wildcard src/images/*.puml)
DIAGRAMS = $(subst src/images,images,$(patsubst %.puml, %.svg, $(DIAGRAMS_SRC)))

DIAGRAMS_SRC_RAW = $(wildcard src/images/*.svg)
DIAGRAMS_RAW = $(subst src/images,images,$(DIAGRAMS_SRC_RAW))

SCHEMAS_SRC = $(wildcard schemas/*.json)
SCHEMAS = $(subst schemas,build/schemas,$(patsubst %.json, %.md, $(SCHEMAS_SRC)))

SCHEMA_FLATBUFFERS = schemas/flatbuffers/opendatafabric.fbs
SCHEMA_FLATBUFFERSC = python tools/jsonschema_to_flatbuffers.py schemas/

all: build/ $(DIAGRAMS) $(DIAGRAMS_RAW) $(SCHEMAS) $(SCHEMA_FLATBUFFERS) open-data-fabric.md

build/:
	mkdir -p build/schemas

$(DIAGRAMS): images/%.svg: src/images/%.puml
	plantuml -o ../../images -tsvg $^

$(DIAGRAMS_RAW): images/%.svg: src/images/%.svg
	cp $^ $@

$(SCHEMAS): build/schemas/%.md: schemas/%.json
	$(SCHEMAC) $^ $@

$(SCHEMA_FLATBUFFERS): $(SCHEMAS_SRC)
	$(SCHEMA_FLATBUFFERSC) > $@

open-data-fabric.md: src/open-data-fabric.md $(SCHEMAS)
	$(MDTPL) src/open-data-fabric.md open-data-fabric.md
	markdown-toc --maxdepth 2 -i open-data-fabric.md

clean:
	rm -rf build/ open-data-fabric.md