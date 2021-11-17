use crate::SourceFormat;
use crate::converter::Error;
use std::env;
use std::io::{stdin, Read};

pub fn get_wasm_args() -> (bool, String, String, SourceFormat) {
    // we'll default to verbose being false as it writes stdout,
    // and stdout is what becomes our response back
    let verbose = false;
    // the input and output files are not used.
    // this is a good point for some future refactoring
    let input_file = String::new();
    let output_file = String::new();
    let input_content_type = env::var("HTTP_CONTENT_TYPE")
        .expect("The Content-Type header was not specified. Unable to convert the source content.");
    let source_format = get_source_format(&input_content_type);
    (verbose, input_file, output_file, source_format)
}

fn get_source_format(input_content_type: &str) -> SourceFormat {
    match input_content_type {
        "application/json" => SourceFormat::JSON,
        "application/yaml" => SourceFormat::YAML,
        "application/yml" => SourceFormat::YAML,
        _ => panic!("Unexpected Content Header."),
    }
}

fn get_output_format(source_format: &SourceFormat) -> String {
    match source_format {
        SourceFormat::JSON => String::from("application/yaml"),
        SourceFormat::YAML => String::from("application/json"),
    }
}

pub fn read_wagi_content() -> String {
    let mut input_content = String::new();
    stdin()
        .read_to_string(&mut input_content)
        .expect("Failed to read from standard in.");
    input_content
}

pub fn write_wagi_output(output_result: Result<String, Error>, source_format: &SourceFormat) {
    let mut content_type = get_output_format(source_format);
    let mut status = 200;
    let output = match output_result {
        Ok(output) => output,
        Err(e) => {
            content_type = "text/plain".to_string();
            status = e.status_code;
            format!("{:?}", e)
        }
    };
    
    println!("Content-Type: {}", content_type );
    println!("Status: {}", status);
    println!();
    println!("{}", output);
}
