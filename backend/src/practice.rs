#![allow(unused)]
use rand::{distr::StandardUniform, Rng};













fn main() -> Result<(),anyhow::Error> {


    let mut all_keys = Vec::new();

    let file_read = std::fs::read_to_string("datafolder/all_cols_fitted_data.json")?;

    let json_parsed = serde_json::from_str(&file_read)?;

    if let serde_json::Value::Object(map) = &json_parsed {
        for key in map.keys() {
            all_keys.push(key.to_owned());
        }
    }
    
    if let Some(serde_json::Value::Object(map)) = json_parsed.get(&all_keys[0]) {
        for key in map.keys() {
            println!("key: {}",key);
        }
    }

    Ok(())
}
