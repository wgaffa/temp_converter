#![feature(bool_to_option)]

use clap::{crate_version, App, Arg};

use temp_converter::{Celsius, Fahrenheit};

fn main() {
    let app = App::new("Temperature Converter")
        .version(crate_version!())
        .args(&[
            Arg::new("temperature")
                .required(true)
                .validator(|x| match x.parse::<f64>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err("must be a floating point number with dot '.' as delimiter"),
                })
                .help("The base temperature to convert"),
            Arg::new("base")
                .short('b')
                .takes_value(true)
                .default_missing_value("F")
                .possible_values(["C", "F", "K"])
                .ignore_case(true)
                .help("Base unit to convert"),
            Arg::new("target")
                .short('t')
                .takes_value(true)
                .default_missing_value("C")
                .possible_values(["C", "F", "K"])
                .ignore_case(true)
                .help("Target unit to convert to"),
        ])
        .get_matches();
}
