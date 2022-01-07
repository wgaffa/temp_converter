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
    () => {};

    ($b:ident, $t:ident => $f:expr, $($tt:tt)*) => {
        impl From<$b> for $t {
            fn from(val: $b) -> Self {
                let converted = $f;
                $t(converted(val.0))
            }
        }

        impl_conv!($($tt)*);
    };

    ($b:ident, $t:ident, $val:ident => $f:expr, $($tt:tt)* ) => {
        impl From<$b> for $t {
            fn from($val: $b) -> Self {
                $t($f)
            }
        }

        impl_conv!($($tt)*);
    };
}

impl_f64_conv!(Celsius, Fahrenheit, Kelvin);

impl_conv!{
    Celsius, Fahrenheit, val => val.0 * 9.0/5.0 + 32.0,
    Celsius, Kelvin, val => val.0 + 273.15,
    Kelvin, Celsius, val => val.0 - 273.15,
    Kelvin, Fahrenheit, val => val.0 * 9.0/5.0 - 459.67,
    Fahrenheit, Celsius, val => (val.0 - 32.0) * 5.0/9.0,
    Fahrenheit, Kelvin, val => (val.0 + 459.67) * 5.0/9.0,
}

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
        () => {};

        ($name:ident , $base:ident, $target:ident; $($tail:tt)* ) => {
            quickcheck! {
                fn $name(input: $base) -> bool {
                    let actual = input.clone();
                    let target: $target = input.into();
                    let expected: $base = target.into();

                    approx_eq!(f64, actual.0, expected.0, ulps = 2, epsilon = 0.01)
                }
            }

            quick_temp!($($tail)*);
        }
    }

    arbitrary_two_point_precision!(Celsius, Fahrenheit, Kelvin);

    quick_temp!{
        celsius_fahrenheit, Celsius, Fahrenheit;
        celsius_kelvin, Celsius, Kelvin;
        fahrenheit_celsius, Fahrenheit, Celsius;
        fahrenheit_kelvin, Fahrenheit, Kelvin;
        kelvin_celsius, Kelvin, Celsius;
        kelvin_fahrenheit, Kelvin, Fahrenheit;
    }
}
