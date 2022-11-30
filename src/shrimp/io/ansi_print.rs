#[macro_export]
macro_rules! add_color {
    () => {
        println!()
    };
    ($($arg:literal)+) => {
        println!("{}", $($arg)*)
    };
}