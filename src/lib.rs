#[derive(Debug, Clone, PartialEq)]
pub struct Celsius(f64);

#[derive(Debug, Clone, PartialEq)]
pub struct Fahrenheit(f64);

#[derive(Debug, Clone, PartialEq)]
pub struct Kelvin(f64);

macro_rules! impl_f64_conv {
    ($($t:ident),*) => {
        $(
            impl From<f64> for $t {
                fn from(val: f64) -> Self {
                    $t(val)
                }
            }
        )*
    }
}

macro_rules! impl_conv {
    ($b:ident, $t:ident => $f:expr) => {
        impl From<$b> for $t {
            fn from(val: $b) -> Self {
                let converted = $f;
                $t(converted(val.0))
            }
        }
    };

    ($b:ident, $t:ident, $val:ident => $f:expr) => {
        impl From<$b> for $t {
            fn from($val: $b) -> Self {
                $t($f)
            }
        }
    }
}

impl_f64_conv!(Celsius, Fahrenheit, Kelvin);

impl_conv!(Celsius, Fahrenheit, val => val.0 * 9.0/5.0 + 32.0);
impl_conv!(Celsius, Kelvin, val => val.0 + 273.15);
impl_conv!(Kelvin, Celsius, val => val.0 - 273.15);
impl_conv!(Kelvin, Fahrenheit, val => val.0 * 9.0/5.0 - 459.67);
impl_conv!(Fahrenheit, Celsius, val => (val.0 - 32.0) * 5.0/9.0);
impl_conv!(Fahrenheit, Kelvin, val => (val.0 + 459.67) * 5.0/9.0);

#[cfg(test)]
mod test {
    use super::*;

    use quickcheck::{quickcheck, Arbitrary, Gen};
    use float_cmp::approx_eq;

    macro_rules! arbitrary_two_point_precision {
        ($($i:ident),*) => {
            $(
                impl Arbitrary for $i {
                    fn arbitrary(gen: &mut Gen) -> Self {
                        let num = (f64::arbitrary(gen) * 100.0).round() / 100.0;
                        $i(num)
                    }
                }
            )*
        }
    }

    macro_rules! quick_temp {
        ($name:ident , $base:ident, $target:ident) => {
            quickcheck! {
                fn $name(input: $base) -> bool {
                    let actual = input.clone();
                    let target: $target = input.into();
                    let expected: $base = target.into();

                    approx_eq!(f64, actual.0, expected.0, ulps = 2, epsilon = 0.01)
                }
            }
        }
    }

    arbitrary_two_point_precision!(Celsius, Fahrenheit, Kelvin);

    quick_temp!(celsius_fahrenheit, Celsius, Fahrenheit);
    quick_temp!(celsius_kelvin, Celsius, Kelvin);
    quick_temp!(fahrenheit_celsius, Fahrenheit, Celsius);
    quick_temp!(fahrenheit_kelvin, Fahrenheit, Kelvin);
    quick_temp!(kelvin_celsius, Kelvin, Celsius);
    quick_temp!(kelvin_fahrenheit, Kelvin, Fahrenheit);
}
