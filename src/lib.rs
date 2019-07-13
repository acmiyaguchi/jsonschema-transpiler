#![recursion_limit = "128"]
#[macro_use]
extern crate log;
extern crate regex;
#[macro_use]
extern crate serde;
extern crate serde_json;

use wasm_bindgen::prelude::*;

mod ast;
mod avro;
mod bigquery;
mod jsonschema;
mod traits;

use serde_json::{json, Value};
use traits::TranslateFrom;

/// The error resoluton method in the [`TranslateFrom`] and [`TranslateInto`]
/// interfaces when converting between schema formats.
///
/// The `Cast` method will represent under-specified (e.g. empty objects) and
/// incompatible (e.g. variant-types or conflicting oneOf definitions) as
/// strings. This behavior is useful for compacting complex types into a single
/// column. In Spark and BigQuery, a casted column can be processed via a user
/// defined function that works on JSON. However, this method may cause issues
/// with schema evolution, for example when adding properties to empty objects.
///
/// The `Drop` method will drop fields if they do not fall neatly into one of
/// the supported types. This method ensures forward compatibility with schemas,
/// but it can lose large portions of nested data. Support from the data
/// processing side can recover dropped data from the structured section of the
/// schema.
///
/// The `Panic` method will panic if the JSON Schema is inconsistent or uses
/// unsupported features. This method is a useful way to test for incompatible
/// schemas.
#[derive(Copy, Clone)]
pub enum ResolveMethod {
    Cast,
    Drop,
    Panic,
}

impl Default for ResolveMethod {
    fn default() -> Self {
        ResolveMethod::Cast
    }
}

/// Options for modifying the behavior of translating between two schema
/// formats.
///
/// This structure passes context from the command-line interface into the
/// translation logic between the various schema types in the project. In
/// particular, the context is useful for resolving edge-cases in ambiguous
/// situations. This can includes situations like casting or dropping an empty
/// object.
#[derive(Copy, Clone, Default)]
pub struct Context {
    pub resolve_method: ResolveMethod,
}

fn into_ast(input: &Value, context: Context) -> ast::Tag {
    let jsonschema: jsonschema::Tag = match serde_json::from_value(json!(input)) {
        Ok(tag) => tag,
        Err(e) => panic!(format!("{:#?}", e)),
    };
    ast::Tag::translate_from(jsonschema, context).unwrap()
}

/// Convert JSON Schema into an Avro compatible schema
pub fn convert_avro(input: &Value, context: Context) -> Value {
    let avro = avro::Type::translate_from(into_ast(input, context), context).unwrap();
    json!(avro)
}

/// Convert JSON Schema into a BigQuery compatible schema
pub fn convert_bigquery(input: &Value, context: Context) -> Value {
    let bq = bigquery::Schema::translate_from(into_ast(input, context), context).unwrap();
    json!(bq)
}

/// Convert JSONSchema into a BigQuery compatible schema
#[wasm_bindgen]
pub fn convert_bigquery_js(input: &JsValue) -> JsValue {
    let context = Context {
        ..Default::default()
    };
    let value: Value = input.into_serde().unwrap();
    let ast_tag = into_ast(&value, context);
    let bq_tag = bigquery::Schema::translate_from(ast_tag, context).unwrap();
    JsValue::from_serde(&bq_tag).unwrap()
}
