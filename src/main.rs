#[macro_use]
extern crate clap;
mod cli;
mod converter;

use cli::{get_cli_args, read_content, write_content};
use converter::{convert_json_to_yaml, convert_yaml_to_json};

arg_enum! {
    #[derive(Debug)]
    pub enum SourceFormat {
        YAML,
        JSON
    }
}

fn main() {
    let (verbose, input_file, output_file, source_format) = 
        if cfg!(target_family = "wasm"){
            (true, String::new(), String::new(), SourceFormat::YAML)
        }
        else {
            get_cli_args()
        };

    let contents = read_content(&input_file, verbose)
        .expect("Unable to read the file");
    let output_content =
        match source_format {
            SourceFormat::YAML => convert_yaml_to_json(&contents, verbose)
                .expect("Unable to convert the yaml to json."),
            SourceFormat::JSON => convert_json_to_yaml(&contents, verbose)
                .expect("Unable to convert the json to yaml."),
        };
    write_content(&output_file, output_content, verbose)
        .expect("Failed to write the output file");
}
