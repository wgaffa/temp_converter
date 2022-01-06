use temp_converter::{Celsius, Fahrenheit};

fn main() {
    println!("Hello, world!");

    let celsius = Celsius::from(1.2357468219272696e308);
    println!("{:?}", celsius);

    let fahr = Fahrenheit::from(celsius);
    println!("{:?}", fahr);

    let celsius = Celsius::from(fahr);
    println!("{:?}", celsius);
}
