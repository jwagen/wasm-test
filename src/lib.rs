use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn preprocess(code: &str) -> String {
    let pre_defines = HashMap::new();
    let include_paths: Vec<String>  = Vec::new();
    let parsed = sv_parser_pp::preprocess::preprocess_str(code, "/", &pre_defines, &include_paths, true, false, 10);
    match parsed {
        Ok(p) => p.0.text().to_owned(),
        Err(_) => "Error parsing".to_owned(),
    }
}

