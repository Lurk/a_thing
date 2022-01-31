use std::fs::read_to_string;
use std::fs::write;
use std::io::Error;

/// # Construct dict from file.
///
pub fn from_file(path: &str) -> Result<Vec<String>, Error> {
    let contents = read_to_string(path)?;

    Ok(contents.split('\n').map(|w| w.to_string()).collect())
}

/// # Write dict to a file
pub fn to_file(dict: &[String], path: &str) -> Result<(), Error> {
    write(path, dict.join("\n"))
}
