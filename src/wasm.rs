use std::env;
use std::io::{stdin, Read};

use converter::error::ConversionError;
use converter::{convert_json_to_yaml, convert_yaml_to_json};

pub fn main_as_wasm() {
    let input_content_type = env::var("HTTP_CONTENT_TYPE")
        .expect("The Content-Type header was not specified. Unable to convert the source content.");
    let input_content = read_source_content_from_stdin();

    match convert_source_to_target(&input_content_type, &input_content) {
        Ok((return_content_type, return_content)) => {
            println!("Content-Type: {}", return_content_type);
            println!();
            println!("{}", return_content);
        }
        Err(_) => {
            let return_content_type = "text/plain";
            let status = "406";
            println!("Content-Type: {}", return_content_type);
            println!("Status: {}", status);
            println!();
            println!("Failed to process:");
            println!("\tIncoming Content-Type: {}", input_content_type);
            println!("\tIncoming Content:\n{}", input_content);
        }
    }
}

fn convert_source_to_target(
    source_content_type: &str,
    source_content: &str,
) -> Result<(String, String), ConversionError> {
    match source_content_type {
        "application/json" => Ok((
            String::from("application/yaml"),
            convert_json_to_yaml(source_content, false)?,
        )),
        "application/yaml" => Ok((
            String::from("application/json"),
            convert_yaml_to_json(source_content, false)?,
        )),
        _ => {
            //String::from("Input Content-Type needs to be set to either application/json or application/yaml based on the source content to convert.")
            Err(ConversionError)
        }
    }
}

fn read_source_content_from_stdin() -> String {
    let mut input_content = String::from("");
    stdin()
        .read_to_string(&mut input_content)
        .expect("Failed to read from standard in.");
    input_content
}
