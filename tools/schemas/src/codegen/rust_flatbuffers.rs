use crate::model;
use crate::utils::indent_writer::IndentWriter;
use std::io::Write;

const SPEC_URL: &str =
    "https://github.com/kamu-data/open-data-fabric/blob/master/open-data-fabric.md";

const PREAMBLE: &str = indoc::indoc!(
    r#"
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // WARNING: This file is auto-generated from Open Data Fabric Schemas
    // See: http://opendatafabric.org/
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(clippy::all)]
    #![allow(clippy::pedantic)]

    use super::proxies_generated as fb;
    mod odf {
        pub use crate::dtos::*;
        pub use crate::formats::*;
        pub use crate::identity::*;
    }
    use std::convert::TryFrom;
    use std::path::PathBuf;

    use ::flatbuffers::{FlatBufferBuilder, Table, UnionWIPOffset, WIPOffset};
    use chrono::prelude::*;

    pub trait FlatbuffersSerializable<'fb> {
        type OffsetT;
        fn serialize(&self, fb: &mut FlatBufferBuilder<'fb>) -> Self::OffsetT;
    }

    pub trait FlatbuffersDeserializable<T> {
        fn deserialize(fb: T) -> Self;
    }

    pub trait FlatbuffersEnumSerializable<'fb, E> {
        fn serialize(&self, fb: &mut FlatBufferBuilder<'fb>) -> (E, WIPOffset<UnionWIPOffset>);
    }

    pub trait FlatbuffersEnumDeserializable<'fb, E> {
        fn deserialize(table: Table<'fb>, t: E) -> Self
        where
            Self: Sized;
    }
    "#
);

const FOOTER: &str = indoc::indoc!(
    r#"
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Helpers
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    fn datetime_to_fb(dt: &DateTime<Utc>) -> fb::Timestamp {
        fb::Timestamp::new(
            dt.year(),
            dt.ordinal() as u16,
            dt.naive_utc().num_seconds_from_midnight(),
            dt.naive_utc().nanosecond(),
        )
    }

    fn fb_to_datetime(dt: &fb::Timestamp) -> DateTime<Utc> {
        let naive_date_time = NaiveDate::from_yo_opt(dt.year(), dt.ordinal() as u32)
            .unwrap()
            .and_time(
                NaiveTime::from_num_seconds_from_midnight_opt(
                    dt.seconds_from_midnight(),
                    dt.nanoseconds(),
                )
                .unwrap(),
            );
        Utc.from_local_datetime(&naive_date_time).unwrap()
    }
    "#
);

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct Helpers {
    model: model::Model,
}

impl Helpers {
    fn is_union(&self, typ: &model::Type) -> bool {
        match typ {
            model::Type::Custom(id) => {
                let typ = self.model.types.get(id).unwrap();
                match typ {
                    model::TypeDefinition::Union(_) => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn is_enum(&self, typ: &model::Type) -> bool {
        match typ {
            model::Type::Custom(id) => {
                let typ = self.model.types.get(id).unwrap();
                match typ {
                    model::TypeDefinition::Enum(_) => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn is_enum_id(&self, id: &model::TypeId) -> bool {
        match self.model.types.get(id).unwrap() {
            model::TypeDefinition::Enum(_) => true,
            _ => false,
        }
    }

    fn is_struct_id(&self, id: &model::TypeId) -> bool {
        match self.model.types.get(id).unwrap() {
            model::TypeDefinition::Object(_) => true,
            _ => false,
        }
    }

    fn is_integer(&self, typ: &model::Type) -> bool {
        match typ {
            model::Type::Int16
            | model::Type::Int32
            | model::Type::Int64
            | model::Type::UInt16
            | model::Type::UInt32
            | model::Type::UInt64 => true,
            model::Type::Boolean
            | model::Type::String
            | model::Type::DatasetAlias
            | model::Type::DatasetId
            | model::Type::DatasetRef
            | model::Type::DateTime
            | model::Type::Flatbuffers
            | model::Type::Multicodec
            | model::Type::Multihash
            | model::Type::Path
            | model::Type::Regex
            | model::Type::Url
            | model::Type::Array(_)
            | model::Type::Custom(_) => false,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn render(model: model::Model, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let mut w = IndentWriter::new(w, "  ");
    render_impl(model, &mut w)
}

fn render_impl(
    model: model::Model,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<(), std::io::Error> {
    writeln!(w, "{}", PREAMBLE)?;

    let helpers = Helpers {
        model: model.clone(),
    };

    for typ in model.types.values() {
        if typ.id().name == "Manifest" || typ.id().name == "DatasetSnapshot" {
            continue;
        }

        writeln!(w, "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////")?;
        writeln!(w, "// {}", typ.id().join(""))?;
        writeln!(
            w,
            "// {SPEC_URL}#{}-schema",
            typ.id().join("").to_lowercase()
        )?;
        writeln!(w, "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////")?;
        writeln!(w, "")?;

        match &typ {
            model::TypeDefinition::Object(t) => render_object(t, &helpers, w)?,
            model::TypeDefinition::Union(t) => render_union(t, w)?,
            model::TypeDefinition::Enum(t) => render_enum(t, w)?,
        }
        writeln!(w)?;
    }

    writeln!(w, "{}", FOOTER)?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_object(
    typ: &model::Object,
    helpers: &Helpers,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<(), std::io::Error> {
    let name = typ.id.join("");

    writeln!(
        w,
        "impl<'fb> FlatbuffersSerializable<'fb> for odf::{name} {{"
    )?;
    writeln!(w, "type OffsetT = WIPOffset<fb::{name}<'fb>>;")?;
    writeln!(w)?;
    writeln!(
        w,
        "fn serialize(&self, fb: &mut FlatBufferBuilder <'fb>) -> Self::OffsetT {{"
    )?;

    let mut preserialized = Vec::new();
    for field in typ.fields.values() {
        let preser = render_field_pre_ser(field, helpers, w)?;
        preserialized.push(preser);
    }

    writeln!(w, "let mut builder = fb::{name}Builder::new(fb);")?;

    for (field, preserialized) in typ.fields.values().zip(preserialized) {
        render_field_ser(field, preserialized, helpers, w)?;
    }

    writeln!(w, "builder.finish()")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w)?;

    writeln!(
        w,
        "impl<'fb> FlatbuffersDeserializable<fb::{name}<'fb>> for odf::{name} {{"
    )?;
    writeln!(w, "fn deserialize(proxy: fb::{name}<'fb>) -> Self {{")?;
    writeln!(w, "odf::{name} {{")?;

    for field in typ.fields.values() {
        render_field_de(field, helpers, w)?;
    }

    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Pre serialization
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_field_pre_ser(
    field: &model::Field,
    helpers: &Helpers,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<bool, std::io::Error> {
    let name = &field.name;

    if field.optional {
        let mut buf = Vec::<u8>::new();
        format_pre_ser_type(format!("v"), &field.typ, helpers, &mut buf)?;
        if !buf.is_empty() {
            writeln!(w, "let {name}_offset = self.{name}.as_ref().map(|v| {{")?;
            w.write_all(&buf)?;
            writeln!(w, "}});")?;
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        let mut buf = Vec::<u8>::new();
        format_pre_ser_type(
            format!("self.{}", field.name),
            &field.typ,
            helpers,
            &mut buf,
        )?;
        if !buf.is_empty() {
            writeln!(w, "let {name}_offset = {{")?;
            w.write_all(&buf)?;
            writeln!(w, "}};")?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

fn format_pre_ser_type(
    name: String,
    typ: &model::Type,
    helpers: &Helpers,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    match typ {
        model::Type::Boolean
        | model::Type::Int16
        | model::Type::Int32
        | model::Type::Int64
        | model::Type::UInt16
        | model::Type::UInt32
        | model::Type::UInt64
        | model::Type::DateTime => (),
        model::Type::String => writeln!(w, "fb.create_string(&{name})")?,
        model::Type::DatasetAlias | model::Type::DatasetRef => {
            writeln!(w, "fb.create_string(&{name}.to_string())")?
        }
        model::Type::DatasetId => writeln!(w, "fb.create_vector(&{name}.as_bytes().as_slice())")?,
        model::Type::Flatbuffers => writeln!(w, "fb.create_vector(&{name}[..])")?,
        model::Type::Multicodec => todo!(),
        model::Type::Multihash => writeln!(w, "fb.create_vector(&{name}.as_bytes().as_slice())")?,
        model::Type::Path => writeln!(w, "fb.create_string({name}.to_str().unwrap())")?,
        model::Type::Regex | model::Type::Url => writeln!(w, "fb.create_string(&{name})")?,
        model::Type::Array(array) => {
            writeln!(w, "let offsets: Vec<_> = {name}.iter().map(|i| {{")?;
            if helpers.is_union(&array.item_type) {
                // TODO: This is a dirty hack
                writeln!(w, "let (value_type, value_offset) = i.serialize(fb);")?;
                writeln!(w, "let mut builder = fb::PrepStepWrapperBuilder::new(fb);")?;
                writeln!(w, "builder.add_value_type(value_type);")?;
                writeln!(w, "builder.add_value(value_offset);")?;
                writeln!(w, "builder.finish()")?;
            } else {
                let mut buf = Vec::<u8>::new();
                format_pre_ser_type(format!("i"), &array.item_type, helpers, &mut buf)?;

                if !buf.is_empty() {
                    w.write_all(&buf)?;
                } else {
                    render_type_ser(format!("i"), &array.item_type, helpers, w)?;
                }
            }
            writeln!(w, "}}).collect();")?;
            writeln!(w, "fb.create_vector(&offsets)")?;
        }
        model::Type::Custom(id) if helpers.is_enum_id(id) => (),
        model::Type::Custom(_) => writeln!(w, "{name}.serialize(fb)")?,
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Serialization
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_field_ser(
    field: &model::Field,
    preserialized: bool,
    helpers: &Helpers,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<(), std::io::Error> {
    let name = &field.name;

    if preserialized {
        if !field.optional {
            if helpers.is_union(&field.typ) {
                writeln!(w, "builder.add_{name}_type({name}_offset.0);")?;
                writeln!(w, "builder.add_{name}({name}_offset.1);")?;
            } else {
                writeln!(w, "builder.add_{name}({name}_offset);")?;
            }
        } else {
            if helpers.is_union(&field.typ) {
                writeln!(w, "{name}_offset.map(|(e, off)| {{ builder.add_{name}_type(e); builder.add_{name}(off) }});")?;
            } else {
                writeln!(w, "{name}_offset.map(|off| builder.add_{name}(off));")?;
            }
        }
    } else {
        if !field.optional {
            writeln!(w, "builder.add_{name}(")?;
            render_type_ser(format!("self.{name}"), &field.typ, helpers, w)?;
            writeln!(w, ");")?;
        } else {
            writeln!(w, "self.{name}.map(|v| builder.add_{name}(")?;
            render_type_ser(format!("v"), &field.typ, helpers, w)?;
            writeln!(w, "));")?;
        }
    }
    Ok(())
}

fn render_type_ser(
    name: String,
    typ: &model::Type,
    helpers: &Helpers,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    match typ {
        model::Type::Boolean
        | model::Type::Int16
        | model::Type::Int32
        | model::Type::Int64
        | model::Type::UInt16
        | model::Type::UInt32
        | model::Type::UInt64 => writeln!(w, "{name}")?,
        model::Type::String
        | model::Type::DatasetAlias
        | model::Type::DatasetId
        | model::Type::DatasetRef
        | model::Type::Flatbuffers
        | model::Type::Multicodec
        | model::Type::Multihash
        | model::Type::Path
        | model::Type::Regex
        | model::Type::Url
        | model::Type::Array(_) => (),
        model::Type::DateTime => writeln!(w, "&datetime_to_fb(&{name})")?,
        model::Type::Custom(type_id) if helpers.is_enum_id(type_id) => {
            writeln!(w, "{name}.into()")?
        }
        model::Type::Custom(_) => (),
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_field_de(
    field: &model::Field,
    helpers: &Helpers,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<(), std::io::Error> {
    let name = &field.name;

    writeln!(w, "{name}:")?;
    if !field.optional && (helpers.is_enum(&field.typ) || helpers.is_integer(&field.typ)) {
        render_type_de(
            format!("proxy.{name}()"),
            &field.typ,
            format!("proxy.{name}_type()"),
            helpers,
            w,
        )?;
    } else {
        writeln!(w, "proxy.{name}().map(|v| {{")?;
        render_type_de(
            format!("v"),
            &field.typ,
            format!("proxy.{name}_type()"),
            helpers,
            w,
        )?;
        writeln!(w, "}})")?;
        if !field.optional {
            writeln!(w, ".unwrap()")?;
        }
    }
    writeln!(w, ",")?;
    Ok(())
}

fn render_type_de(
    name: String,
    typ: &model::Type,
    enum_t_accessor: String,
    helpers: &Helpers,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<(), std::io::Error> {
    match typ {
        model::Type::Boolean
        | model::Type::Int16
        | model::Type::Int32
        | model::Type::Int64
        | model::Type::UInt16
        | model::Type::UInt32
        | model::Type::UInt64 => writeln!(w, "{name}")?,
        model::Type::String => writeln!(w, "{name}.to_owned()")?,
        model::Type::DatasetAlias => writeln!(w, "odf::DatasetAlias::try_from({name}).unwrap()")?,
        model::Type::DatasetId => {
            writeln!(w, "odf::DatasetID::from_bytes({name}.bytes()).unwrap()")?
        }
        model::Type::DatasetRef => writeln!(w, "odf::DatasetRef::try_from({name}).unwrap()")?,
        model::Type::DateTime => writeln!(w, "fb_to_datetime({name})")?,
        model::Type::Flatbuffers => writeln!(w, "{name}.bytes().to_vec()")?,
        model::Type::Multicodec => todo!(),
        model::Type::Multihash => {
            writeln!(w, "odf::Multihash::from_bytes({name}.bytes()).unwrap()")?
        }
        model::Type::Path => writeln!(w, "PathBuf::from({name})")?,
        model::Type::Regex => writeln!(w, "{name}.to_owned()")?,
        model::Type::Url => writeln!(w, "{name}.to_owned()")?,
        model::Type::Array(array) => {
            writeln!(w, "{name}.iter().map(|i| ")?;
            if helpers.is_union(&array.item_type) {
                render_type_de(
                    format!("i.value().unwrap()"),
                    &array.item_type,
                    format!("i.value_type()"),
                    helpers,
                    w,
                )?;
            } else {
                render_type_de(format!("i"), &array.item_type, "".to_string(), helpers, w)?;
            }
            writeln!(w, ").collect()")?;
        }
        model::Type::Custom(type_id) if helpers.is_enum_id(type_id) => {
            writeln!(w, "{name}.into()")?
        }
        model::Type::Custom(type_id) if helpers.is_struct_id(type_id) => {
            writeln!(w, "odf::{}::deserialize({name})", type_id.join(""))?
        }
        model::Type::Custom(type_id) => writeln!(
            w,
            "odf::{}::deserialize({name}, {enum_t_accessor})",
            type_id.join("")
        )?,
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_union(
    typ: &model::Union,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<(), std::io::Error> {
    let name = typ.id.join("");

    writeln!(
        w,
        "impl<'fb> FlatbuffersEnumSerializable<'fb, fb::{name}> for odf::{name} {{"
    )?;
    writeln!(w, "fn serialize(&self, fb: &mut FlatBufferBuilder<'fb>) -> (fb::{name}, WIPOffset<UnionWIPOffset>) {{")?;
    writeln!(w, "match self {{")?;

    for variant in &typ.variants {
        let typ = variant.join("");
        let var = &variant.name;
        writeln!(
            w,
            "odf::{name}::{var}(v) => (fb::{name}::{typ}, v.serialize(fb).as_union_value()),"
        )?;
    }

    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w)?;
    writeln!(
        w,
        "impl<'fb> FlatbuffersEnumDeserializable<'fb, fb::{name}> for odf::{name} {{"
    )?;
    writeln!(
        w,
        "fn deserialize(table: flatbuffers::Table<'fb>, t: fb::{name}) -> Self {{"
    )?;
    writeln!(w, "match t {{")?;

    for variant in &typ.variants {
        let typ = variant.join("");
        let var = &variant.name;
        writeln!(w, "fb::{name}::{typ} => odf::{name}::{var}(")?;
        writeln!(w, "    odf::{typ}::deserialize(")?;
        writeln!(w, "        unsafe {{ fb::{typ}::init_from_table(table) }}")?;
        writeln!(w, "    )")?;
        writeln!(w, "),")?;
    }
    writeln!(w, "_ => panic!(\"Invalid enum value: {{}}\", t.0),")?;

    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_enum(
    typ: &model::Enum,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<(), std::io::Error> {
    let name = &typ.id.join("");

    writeln!(w, "impl From<odf::{name}> for fb::{name} {{")?;
    writeln!(w, "fn from(v: odf::{name}) -> Self {{")?;
    writeln!(w, "match v {{")?;
    for variant in &typ.variants {
        writeln!(w, "odf::{name}::{variant} => fb::{name}::{variant},")?;
    }
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "")?;
    writeln!(w, "impl Into<odf::{name}> for fb::{name} {{")?;
    writeln!(w, "fn into(self) -> odf::{name} {{")?;
    writeln!(w, "match self {{")?;
    for variant in &typ.variants {
        writeln!(w, "fb::{name}::{variant} => odf::{name}::{variant},")?;
    }
    writeln!(w, "_ => panic!(\"Invalid enum value: {{}}\", self.0),")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
