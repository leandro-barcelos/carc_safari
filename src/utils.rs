use std::fs::read_to_string;
use serde_json:: {Value, from_str};

pub fn read_json_file(path: String) -> Value {
    let data = read_to_string(path).expect("error: unable to read file");
    let json: Value = from_str(&*data).expect("error: JSON was not well formatted");
    return json
}