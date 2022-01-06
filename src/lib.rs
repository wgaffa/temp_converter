pub struct Celsius(f64);
pub struct Fahrenheit(f64);
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
    }
}

impl_f64_conv!(Celsius, Fahrenheit, Kelvin);

impl_conv!(Celsius, Fahrenheit => |x: f64| x * 9.0/5.0 + 32.0);
impl_conv!(Celsius, Kelvin => |x: f64| x + 273.15);
impl_conv!(Kelvin, Celsius => |x: f64| x - 273.15);
impl_conv!(Kelvin, Fahrenheit => |x: f64| x * 9.0/5.0 - 459.67);
impl_conv!(Fahrenheit, Celsius => |x: f64| (x - 32.0) * 5.0/9.0);
impl_conv!(Fahrenheit, Kelvin => |x: f64| (x + 459.67) * 5.0/9.0);
