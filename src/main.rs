#![feature(bool_to_option)]

use clap::{crate_name, crate_version, App, Arg};

use temp_converter::{Celsius, Fahrenheit};

fn unit_validator(input: &str) -> Result<(), &'static str> {
    let uppercase = input.to_uppercase();
    matches!(uppercase.chars().nth(0), Some('C') | Some('F') | Some('K'))
        .then_some(())
        .ok_or("Valid unit types are (C)elsius, (F)ahrenheit and (K)elvin")
}

fn main() {
    let app = App::new("Temperature Converter")
        .version(crate_version!())
        .arg(
            Arg::new("base")
                .short('b')
                .takes_value(true)
                .default_missing_value("F")
                .possible_values(["C", "F", "K"])
                .ignore_case(true)
                .help("Base unit to convert"),
        )
        .arg(
            Arg::new("target")
                .short('t')
                .takes_value(true)
                .default_missing_value("C")
                .possible_values(["C", "F", "K"])
                .ignore_case(true)
                .help("Target unit to convert to"),
        )
        .get_matches();
}
