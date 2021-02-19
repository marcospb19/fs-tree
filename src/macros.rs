#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        ::std::eprintln!($($arg)*);
        ::std::process::exit(1);
    })
}
