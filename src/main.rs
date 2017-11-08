extern crate serde_json;
extern crate serde_yaml;
extern crate clap;

use std::io::prelude::*;
use std::fs::File;
use clap::{Arg, App};
use serde_json::{Error};

fn main() {
    let matches = App::new("J 2 Y")
                          .version("0.0.1")
                          .author("Steven Murawski <steven.murawski@microsoft.com>")
                          .about("Converts JSON to YAML")
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("OUTPUT")
                               .help("Sets the output file to use")
                               .required(true)
                               .index(2))
                          .arg(Arg::with_name("verbose")
                               .short("v")
                               .help("Include verbose output"))
                          .get_matches();

    let verbose = matches.is_present("verbose");
    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT").unwrap();

    let contents = read_json_content(input_file, verbose);
    let output_content = convert_json_to_yaml(&contents, verbose).expect("Unable to convert the json to yaml.");
    write_yaml_content(output_file, output_content, verbose);
}

fn read_json_content(file_path: &str, verbose: bool) -> String {
    if verbose {
        println!("Reading: {} \n", file_path);
    }
    let mut file = File::open(file_path).expect("Cannot open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    if verbose {
        println!("Read content: \n");
        println!("{}", &contents);
    }
    contents
}

fn convert_json_to_yaml(json_str: &str, verbose: bool) -> Result<String, Error> {

    // Parse the string of json data into serde_yaml::Value.
    let v: serde_yaml::Value = serde_json::from_str(json_str)?;
    let yaml_string = serde_yaml::to_string(&v).expect("Failed to convert the YAML to a string.");

    if verbose {
        println!("\nAfter YAML conversion: \n");
        println!("{}", &yaml_string);
    }
    
    Ok(yaml_string)
}

fn write_yaml_content(file_path: &str, output_content: String, verbose: bool) {
    if verbose {
        println!("\nWriting: {} \n", file_path);
    }
    let mut file = File::create(file_path).expect("Failed to create the output file.");
    file.write_all(output_content.into_bytes().as_ref()).expect("Failed to write the yaml document.");
}
