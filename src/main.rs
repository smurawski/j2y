extern crate serde_json;
extern crate serde_yaml;

#[macro_use]
extern crate clap;

pub mod converter;

mod cli;
use cli::main_as_cli;

mod wasm;
use wasm::main_as_wasm;

fn main() {
    if cfg!(target_family = "wasm") {
        main_as_wasm()
    } else {
        main_as_cli()
    }
}
