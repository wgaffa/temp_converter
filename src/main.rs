#![feature(bool_to_option)]

use std::cell::Cell;

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

    let base = match app.value_of("base") {
        Some("C") => Temperature::Celsius(Celsius::from(temp)),
        Some("F") => Temperature::Fahrenheit(Fahrenheit::from(temp)),
        Some("K") => Temperature::Kelvin(Kelvin::from(temp)),
        Some(_) => unreachable!(),
        None => unreachable!(),
    };

    let target = match app.value_of("target") {
        Some("C") => match base {
            Temperature::Celsius(base) => Temperature::Celsius(base),
            Temperature::Fahrenheit(base) => Temperature::Celsius(Celsius::from(base)),
            Temperature::Kelvin(base) => Temperature::Celsius(Celsius::from(base)),
        },
        Some("F") => match base {
            Temperature::Celsius(base) => Temperature::Fahrenheit(Fahrenheit::from(base)),
            Temperature::Fahrenheit(base) => Temperature::Fahrenheit(base),
            Temperature::Kelvin(base) => Temperature::Fahrenheit(Fahrenheit::from(base)),
        },
        Some("K") => match base {
            Temperature::Celsius(base) => Temperature::Kelvin(Kelvin::from(base)),
            Temperature::Fahrenheit(base) => Temperature::Kelvin(Kelvin::from(base)),
            Temperature::Kelvin(base) => Temperature::Kelvin(base),
        },
        Some(_) => unreachable!(),
        None => unreachable!(),
    };

    println!("{:?}", target);
}
