use derive_more::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fahrenheit(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kelvin(pub f64);

impl std::fmt::Display for Celsius {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}°C", self.0)
    }
}

impl std::fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}°F", self.0)
    }
}

impl std::fmt::Display for Kelvin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}°K", self.0)
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Display)]
pub enum Temperature {
    Celsius(Celsius),
    Fahrenheit(Fahrenheit),
    Kelvin(Kelvin),
}

#[cfg(test)]
mod test {
    use super::*;

    use quickcheck::{quickcheck, Arbitrary, Gen};
    use float_cmp::approx_eq;
    use paste::paste;

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

                    approx_eq!(f64, actual.0, expected.0, ulps = 2, epsilon = 0.000001)
                }
            }

            quick_temp!($($tail)*);
        };

        ($base:ident, $target:ident; $($tail:tt)* ) => {
            paste! {
                quickcheck! {
                    fn [<$base:lower _to_ $target:lower>](input: $base) -> bool {
                        let actual = input.clone();
                        let target: $target = input.into();
                        let expected: $base = target.into();

                        approx_eq!(f64, actual.0, expected.0, ulps = 2, epsilon = 0.000001)
                    }
                }
            }

            quick_temp!($($tail)*);
        }
    }

    arbitrary_two_point_precision!(Celsius, Fahrenheit, Kelvin);

    quick_temp!{
        Celsius, Fahrenheit;
        Celsius, Kelvin;
        Fahrenheit, Celsius;
        Fahrenheit, Kelvin;
        Kelvin, Celsius;
        Kelvin, Fahrenheit;
    }
}
