use std::fmt;
use std::path::Path;

/// Join a path into a String with separator.
pub fn path_to_string(path: &Path, sep: &str) -> String {
    path.iter()
        .filter_map(|s| s.to_str())
        .collect::<Vec<&str>>()
        .join(sep)
}

/// Retrieve value from an environment variable or if unst return the default value.
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
