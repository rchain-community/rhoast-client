use handlebars::Handlebars;
use serde_json::json;
use std::{error::Error, fs};

pub fn parse_simple_deploy(text: &String) -> Result<String, Box<dyn Error>> {
    let reg = Handlebars::new();
    let rholang_code = get_rholang("simple_rholang.txt");
    let compiled = reg.render_template(&rholang_code, &json!({ "text": &text }))?;
    Ok(compiled)
}

pub fn get_rholang(path: &str) -> String {
    let path = format!("src/rholang/{}", path);
    fs::read_to_string(path).expect("Unable to read file")
}
