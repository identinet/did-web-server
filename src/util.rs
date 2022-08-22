use std::fmt;

pub fn get_env(varname: &str, default: &str) -> String {
    match std::env::var(varname) {
        Ok(value) => value,
        Err(_) => default.to_string(),
    }
}

/// Takes a message and returns a function that takes a variable, prints it with the message and
/// returns the argument.
///
/// # Example:
///
/// ```rust
/// Err("an error")
///   .map_err(log("state of error is"))
///   .map(|x|
///     // do something else
///     x
///   )
/// ```
///
/// Prints: `state of the error is: an error`
pub fn log<T: fmt::Display>(msg: &'static str) -> impl Fn(T) -> T {
    move |o| {
        println!("{}: {}", msg, o);
        o
    }
}
