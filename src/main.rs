#[macro_use]
extern crate clap;
mod cli;
mod converter;
mod wasm;

use cli::{get_cli_args, read_content, write_content};
use converter::{convert_json_to_yaml, convert_yaml_to_json};
use wasm::{get_wasm_args, read_wagi_content};

arg_enum! {
    #[derive(Debug)]
    pub enum SourceFormat {
        YAML,
        JSON
    }
}

fn main() {
    let (verbose, input_file, output_file, source_format) = if cfg!(target_family = "wasm") {
        get_wasm_args()
    } else {
        get_cli_args()
    };

    let contents = if cfg!(target_family = "wasm") {
        read_wagi_content()
    } else {
        read_content(&input_file, verbose)
    };

    let output_content =
        match source_format {
            SourceFormat::YAML => convert_yaml_to_json(&contents, verbose)
                .expect("Unable to convert the yaml to json."),
            SourceFormat::JSON => convert_json_to_yaml(&contents, verbose)
                .expect("Unable to convert the json to yaml."),
        };

    write_content(&output_file, output_content, verbose).expect("Failed to write the output file");
}
