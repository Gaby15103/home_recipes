use utoipa::OpenApi;
use serde_json::{Value, json};
use crate::openapi::ApiDoc;
use std::collections::BTreeMap;
use utoipa::openapi::{RefOr, Schema};

pub fn get_cleaned_gemini_schema() -> Value {
    // 1. Generate the full OpenAPI spec from your struct
    let api_doc = ApiDoc::openapi();
    let components = api_doc.components.as_ref()
        .expect("OpenApi components are empty. Ensure DTOs are in the schemas() macro.");

    // 2. Extract the specific schema we want to send to Gemini
    let root_schema = components.schemas.get("CreateRecipeInput")
        .expect("CreateRecipeInput not found. Did you add it to ApiDoc components?");

    let mut schema_json = serde_json::to_value(root_schema).unwrap();

    // 3. Flatten all $ref links into a single object Gemini can read
    resolve_and_clean(&mut schema_json, &components.schemas);

    schema_json
}

fn resolve_and_clean(value: &mut Value, all_schemas: &BTreeMap<String, RefOr<Schema>>) {
    if let Some(obj) = value.as_object_mut() {

        // --- 1. RESOLVE $REF ---
        if let Some(reference) = obj.get("$ref").and_then(|r| r.as_str()) {
            let name = reference.replace("#/components/schemas/", "");
            if let Some(RefOr::T(resolved_schema)) = all_schemas.get(&name) {
                let mut resolved_json = serde_json::to_value(resolved_schema).unwrap();
                // Recurse into the new object before replacing
                resolve_and_clean(&mut resolved_json, all_schemas);
                *value = resolved_json;
                return;
            }
        }

        // --- 2. FIX TYPE ARRAYS (For Nullable fields) ---
        if let Some(t) = obj.get_mut("type") {
            if let Some(arr) = t.as_array() {
                if let Some(first_type) = arr.iter().find(|v| v.as_str() != Some("null")) {
                    *t = first_type.clone();
                }
            }
        }

        // --- 3. STRIP NON-SUPPORTED METADATA ---
        obj.remove("format");   // Gemini can choke on custom formats
        obj.remove("example");
        obj.remove("title");

        // --- 4. RECURSE THROUGH PROPERTIES & ARRAYS ---
        if let Some(props) = obj.get_mut("properties") {
            if let Some(props_obj) = props.as_object_mut() {
                for (_, prop_val) in props_obj.iter_mut() {
                    resolve_and_clean(prop_val, all_schemas);
                }
            }
        }

        if let Some(items) = obj.get_mut("items") {
            resolve_and_clean(items, all_schemas);
        }

        // Handle Enums (oneOf/anyOf)
        for key in ["oneOf", "anyOf", "allOf"] {
            if let Some(list) = obj.get_mut(key).and_then(|l| l.as_array_mut()) {
                for item in list {
                    resolve_and_clean(item, all_schemas);
                }
            }
        }
    }
}