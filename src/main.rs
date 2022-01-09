#![feature(bool_to_option)]

use clap::{crate_version, App, Arg};

use temp_converter::{Celsius, Fahrenheit, Kelvin, Temperature};

fn main() {
    let app = App::new("Temperature Converter")
        .version(crate_version!())
        .args(&[
            Arg::new("temperature")
                .required(true)
                .help("The base temperature to convert"),
            Arg::new("base")
                .short('b')
                .takes_value(true)
                .default_value("F")
                .possible_values(["C", "F", "K"])
                .ignore_case(true)
                .help("Base unit to convert"),
            Arg::new("target")
                .short('t')
                .takes_value(true)
                .default_value("C")
                .possible_values(["C", "F", "K"])
                .ignore_case(true)
                .help("Target unit to convert to"),
        ])
        .get_matches();

    let temp = app.value_of_t_or_exit::<f64>("temperature");

    let target = match (app.value_of("target"), app.value_of("base")) {
        (Some("C"), Some("F")) => Temperature::Celsius(Celsius::from(Fahrenheit::from(temp))),
        (Some("C"), Some("K")) => Temperature::Celsius(Celsius::from(Kelvin::from(temp))),
        (Some("C"), Some("C")) => Temperature::Celsius(Celsius::from(temp)),
        (Some("F"), Some("C")) => Temperature::Fahrenheit(Fahrenheit::from(Celsius::from(temp))),
        (Some("F"), Some("F")) => Temperature::Fahrenheit(Fahrenheit::from(temp)),
        (Some("F"), Some("K")) => Temperature::Fahrenheit(Fahrenheit::from(Kelvin::from(temp))),
        (Some("K"), Some("C")) => Temperature::Kelvin(Kelvin::from(Celsius::from(temp))),
        (Some("K"), Some("F")) => Temperature::Kelvin(Kelvin::from(Fahrenheit::from(temp))),
        (Some("K"), Some("K")) => Temperature::Kelvin(Kelvin::from(temp)),
        _ => unreachable!(),
    };

    println!("{}", target);
}
