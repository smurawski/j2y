use crate::converter::error::ConversionError;
use crate::converter::{convert_json_to_yaml, convert_yaml_to_json};
use clap::{App, Arg};

use std::fs::File;
use std::io;
use std::io::prelude::*;

arg_enum! {
    #[derive(Debug)]
    enum SourceFormat {
        YAML,
        JSON
    }
}

pub fn main_as_cli() {
    let version = format!(
        "{}.{}",
        env!("CARGO_PKG_VERSION"),
        option_env!("BUILD_BUILDID").unwrap_or("0")
    );

    let matches = App::new("J 2 Y")
        .version(&*version)
        .author("Steven Murawski <steven.murawski@microsoft.com>")
        .about("Converts JSON between YAML")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the output file to use")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("SourceFormat")
                .long("source")
                .short("s")
                .help("Input serialization format.")
                .default_value("JSON")
                .possible_values(&SourceFormat::variants()),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .help("Include verbose output"),
        )
        .get_matches();

    let verbose = matches.is_present("verbose");
    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT").unwrap();
    let source_format =
        value_t!(matches.value_of("SourceFormat"), SourceFormat).unwrap_or_else(|e| e.exit());

    let contents = read_source_file(input_file, verbose).expect("Unable to read the file");
    let output_content =
        match source_format {
            SourceFormat::YAML => convert_yaml_to_json(&contents, verbose)
                .expect("Unable to convert the yaml to json."),
            SourceFormat::JSON => convert_json_to_yaml(&contents, verbose)
                .expect("Unable to convert the json to yaml."),
        };
    write_output_file(output_file, output_content, verbose)
        .expect("Failed to write the output file");
}

fn read_source_file(file_path: &str, verbose: bool) -> Result<String, io::Error> {
    if verbose {
        println!("Reading: {} \n", file_path);
    }
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if verbose {
        println!("Read content: \n");
        println!("{}", &contents);
    }
    Ok(contents)
}

fn write_output_file(file_path: &str, output_content: String, verbose: bool) -> io::Result<()> {
    if verbose {
        println!("\nWriting: {} \n", file_path);
    }
    let mut file = File::create(file_path).expect("Failed to create the output file.");
    file.write_all(output_content.into_bytes().as_ref())
}
