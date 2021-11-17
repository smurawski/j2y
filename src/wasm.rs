use crate::SourceFormat;
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

pub fn read_wagi_content() -> String {
    let mut input_content = String::new();
    stdin()
        .read_to_string(&mut input_content)
        .expect("Failed to read from standard in.");
    input_content
}
