use chrono::Timelike;


#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        // let time = $crate::logging::get_time();
        // ANSI: \x1b[90m = bright black (gray), \x1b[0m = reset
        println!(
            // "[\x1b[36m{}\x1b[0m] [\x1b[36m{}:{}\x1b[0m] {}",
            "[\x1b[36m{}:{}\x1b[0m] {}",
            // time,
            file!(),
            line!(),
            format!($($arg)*)
        );
    }};
}


#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        // let time = $crate::logging::get_time();
        // ANSI: \x1b[90m = bright black (gray), \x1b[0m = reset
        println!(
            // "[\x1b[36m{}\x1b[0m] [\x1b[36m{}:{}\x1b[0m] {}",
            "[\x1b[33m{}:{}\x1b[0m] {}",
            // time,
            file!(),
            line!(),
            format!($($arg)*)
        );
    }};
}


#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        // let time = $crate::logging::get_time();
        // ANSI: \x1b[90m = bright black (gray), \x1b[0m = reset
        println!(
            // "[\x1b[36m{}\x1b[0m] [\x1b[36m{}:{}\x1b[0m] {}",
            "[\x1b[31m{}:{}\x1b[0m] {}",
            // time,
            file!(),
            line!(),
            format!($($arg)*)
        );
    }};
}


pub fn get_time() -> String {
    use chrono::Local;
    let now = Local::now();
    format!("{:02}:{:02}:{:02}.{:03}", now.hour(), now.minute(), now.second(), now.timestamp_subsec_millis())
}