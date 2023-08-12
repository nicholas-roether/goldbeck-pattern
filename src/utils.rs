#[macro_export]
macro_rules! cls {
	() => { String::new() };
    ($classes: expr) => {
        ToString::to_string($classes)
    };
    ($classes: expr, $($rest: tt)+) => {
        cls!($classes) + " " + &cls!($($rest)*)
    };
}
