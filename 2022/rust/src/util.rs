use std::error;
use std::fmt;
use std::io;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Error {
    description: Option<String>
}

impl Error {
    pub fn empty() -> Self {
        Self { description: None }
    }

    pub fn new(s: &str) -> Self {
        Self { description: Some(s.into()) }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(description) = &self.description {
            write!(f, "{description}")
        } else {
            write!(f, "error")
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub fn parse_many<'a, F, I, S>(lines: I) -> Result<Vec<F>, <F as FromStr>::Err>
where
    F: FromStr,
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    lines
        .into_iter()
        .map(|s| s.as_ref().parse::<F>())
        .collect::<Result<_, _>>()
}

pub fn to_ioerror<E>(error: E) -> io::Error
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, error)
}
