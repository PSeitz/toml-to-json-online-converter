mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use serde_json::Deserializer;
// use serde_json::{Serializer, Deserializer};
use toml::ser::Serializer;
// use toml::de::Deserializer;


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(&format!("Hello, {}", s));
}

#[wasm_bindgen]
pub fn convert_json_to_toml(input:&str) -> Result<String, JsValue>{
    // let val: serde_json::Value = serde_json::from_str(input).map_err(|err| JsValue::from_str(&err.to_string()))?;


    // let reader = BufReader::new(File::open("input.json").unwrap());
    // let writer = BufWriter::new(File::create("output.json").unwrap());
    let mut toml = String::new();

    let mut deserializer = Deserializer::from_str(input);
    let mut serializer = Serializer::pretty(&mut toml);
    serde_transcode::transcode(&mut deserializer, &mut serializer).map_err(|err| JsValue::from_str(&err.to_string()))?;
    // serializer.into_inner().flush().map_err(|err| JsValue::from_str(&err.to_string()))?;

    // let val: toml::Value = toml::from_str(input).map_err(|err| JsValue::from_str(&err.to_string()))?;
    // let toml = toml::to_string_pretty(&val).map_err(|err| JsValue::from_str(&err.to_string()))?;

    // let toml = toml::to_string_pretty(&val).map_err(|err| JsValue::from_str(&err.to_string()))?;
    Ok(toml)
}

#[wasm_bindgen]
pub fn convert_toml_to_json(input:&str) -> Result<String, JsValue>{
    let val: toml::Value = toml::from_str(input).map_err(|err| JsValue::from_str(&err.to_string()))?;
    let serde_json = serde_json::to_string_pretty(&val).map_err(|err| JsValue::from_str(&err.to_string()))?;
    Ok(serde_json)
}


#[test]
fn test_json_to_toml() {
    let input = r#"{
        "Data":{"is_cool":true}
    }"#;

    let output = r#"[Data]
is_cool = true
"#;
    let conv = convert_json_to_toml(input).unwrap();

    assert_eq!(conv, output);
}


#[test]
fn test_json_complex_to_toml() {
    let input = r#"{
  "title": "TOML Example",
  "owner": {
    "organization": "GitHub"
  }
}"#;

    let output = r#"title = 'TOML Example'

[owner]
organization = 'GitHub'
"#;
    let conv = convert_json_to_toml(input).unwrap();

    assert_eq!(conv, output);
}


#[test]
fn test_toml_to_json() {
    let input = r#"[Data]
is_cool = true
"#;

    let output = r#"{
  "Data": {
    "is_cool": true
  }
}"#;


    let conv = convert_toml_to_json(input).unwrap();
    assert_eq!(conv, output);
}


// #[test]
// fn test_toml_to_json_error() {
//     let input = r#"[Data
// is_cool = true
// "#;
//     assert_eq!(convert_toml_to_json(input).unwrap_err(), "expected a right bracket, found a newline at line 1 column 6");
// }