use handlebars::Handlebars;
use serde_json::json;
use std::{collections::BTreeMap, error::Error, fs};

pub fn parse_simple_deploy(text: &String) -> Result<String, Box<dyn Error>> {
    let reg = Handlebars::new();
    let rholang_code = get_rholang("simple_rholang.txt");
    let compiled = reg.render_template(&rholang_code, &json!({ "text": &text }))?;
    Ok(compiled)
}

fn parse_simple(rholang: &String, arg: &String, text: &String) -> Result<String, Box<dyn Error>> {
    let reg = Handlebars::new();
    let compiled = reg.render_template(&rholang, &json!({ arg: &text }))?;
    Ok(compiled)
}

pub fn parse_multiple_args(
    rholang: &String,
    args: &BTreeMap<String, String>,
) -> Result<String, Box<dyn Error>> {
    let mut rtc: String = rholang.to_string();
    for a in args.keys() {
        let val = args.get(a).unwrap();
        rtc = parse_simple(&rtc, a, val).unwrap().to_string();
    }
    Ok(rtc.to_string())
}

pub fn get_rholang(path: &str) -> String {
    let path = format!("src/rholang/{}", path);
    fs::read_to_string(path).expect("Unable to read file")
}
