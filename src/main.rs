#[macro_use]
extern crate clap;
mod converter;

use converter::{read_json_content, convert_json_to_yaml, convert_yaml_to_json, write_yaml_content};

use clap::{Arg, App};

arg_enum!{
    #[derive(Debug)]
    enum SourceFormat {
        YAML,
        JSON
    }
}

fn main() {
    let matches = App::new("J 2 Y")
                        .version("0.0.2")
                        .author("Steven Murawski <steven.murawski@microsoft.com>")
                        .about("Converts JSON between YAML")
                        .arg(Arg::with_name("INPUT")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1))
                        .arg(Arg::with_name("OUTPUT")
                            .help("Sets the output file to use")
                            .required(true)
                            .index(2))
                        .arg(Arg::with_name("SourceFormat")
                            .long("source")
                            .short("s")
                            .help("Input serialization format.")
                            .default_value("JSON")
                            .possible_values(&SourceFormat::variants()))
                        .arg(Arg::with_name("verbose")
                            .short("v")
                            .help("Include verbose output"))
                        .get_matches();

    let verbose = matches.is_present("verbose");
    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT").unwrap();
    let source_format = value_t!(matches.value_of("SourceFormat"), SourceFormat).unwrap_or_else(|e| e.exit());

    let contents = read_json_content(input_file, verbose).expect("Unable to read the file");
    let output_content = match source_format {
        SourceFormat::YAML => convert_yaml_to_json(&contents, verbose).expect("Unable to convert the yaml to json."),
        SourceFormat::JSON => convert_json_to_yaml(&contents, verbose).expect("Unable to convert the json to yaml."),
    };
    write_yaml_content(output_file, output_content, verbose).expect("Failed to write the output file");
}
