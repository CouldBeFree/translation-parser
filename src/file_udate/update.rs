extern crate serde;
extern crate serde_json;

use serde_json::{Value, json};

pub fn update(file_path: String, key: String, translation: String) {
    let mut data = load_json_file(&file_path).unwrap();
    let updated_value = json!(translation);
    update_json_value(&mut data, &key, updated_value.clone()).expect("Key not found");
    save_to_json(&data, &file_path).unwrap();
}

fn load_json_file(file_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let json_str = std::fs::read_to_string(file_path)?;
    let data: Value = serde_json::from_str(&json_str)?;
    Ok(data)
}

fn update_json_value(json_data: &mut Value, key: &str, new_value: Value) -> Result<(), ()> {
    if let Value::Object(map) = json_data {
        if let Some(existing_value) = map.get_mut(key) {
            *existing_value = new_value;
            return Ok(())
        }
    }
    Err(())
}

fn save_to_json(json_data: &Value, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_str = serde_json::to_string_pretty(json_data)?;
    std::fs::write(file_path, json_str)?;
    Ok(())
}
