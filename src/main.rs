#![allow(dead_code)]
#![allow(unused_variables)]

pub mod error;
pub mod email;
pub mod instruction;

use clap::{arg, command};
use std::fs;
use std::process;

fn main() {
    let mut log_builder = env_logger::builder();

    if cfg!(debug_assertions) {
        log_builder.filter_level(log::LevelFilter::Debug);
    } else {
        log_builder.filter_level(log::LevelFilter::Off);
    }

    log_builder.init();

    let matches = command!()
        .arg(
            arg!(
                -s --script <FILE> "Sets the script to run."
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(true)
        )
        .get_matches();

    let file_path = matches.value_of("script").unwrap();

    let source = match fs::read_to_string(file_path) {
        Ok(src) => src,
        Err(error) => {
            eprintln!("Failed to read from source file: {:?}", error.to_string());
            process::exit(-1);
        }
    };

    email::get_classes(&source);
}

