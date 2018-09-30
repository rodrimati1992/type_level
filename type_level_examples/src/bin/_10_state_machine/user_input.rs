use std::io;
use std::str::FromStr;

/// Union between parse and io errors.
#[derive(Debug)]
pub enum IoParseError<T> {
    Io(io::Error),
    Parse(T),
}

impl<T> From<io::Error> for IoParseError<T> {
    fn from(other: io::Error) -> Self {
        IoParseError::Io(other)
    }
}

/// Reads from stdin and tries to convert it to T.
///
/// Returns either Io or Parse errors
pub fn readln_parse<T: FromStr>() -> Result<T, IoParseError<T::Err>> {
    readln_res()?.parse().map_err(IoParseError::Parse)
}

/// Reads from stdin.
/// Returns an Err on any io::Error or if the line was empty(it must include the newline char).
///
/// On success it trims the trailing space.
pub fn readln_res() -> io::Result<String> {
    let mut buffer = String::new();
    let res = io::stdin().read_line(&mut buffer);
    match res {
        Ok(0) => Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "expected end of line",
        )),
        Ok(_) => {
            let length = buffer.trim_right().len();
            buffer.truncate(length);
            Ok(buffer)
        }
        Err(e) => Err(e),
    }
}
