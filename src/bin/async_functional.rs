use futures::{Future, FutureExt};
use std::error::Error;
use std::fmt;
use std::io;
use std::str::FromStr;
use std::string::FromUtf8Error;

#[tokio::main]
async fn main() {
    const PATH: &str = "person.txt";

    match read_person(PATH).await {
        Ok(p) => println!("person: {:?}", p),
        Err(e) => eprintln!("error: {e}"),
    }
}

fn read_person(path: &str) -> impl Future<Output = Result<Person, PersonError>> + '_ {
    tokio::fs::read(path).map(|r: Result<Vec<u8>, io::Error>| {
        r.map_err(PersonError::from)
            .and_then(|b: Vec<u8>| String::from_utf8(b).map_err(PersonError::from))
            .and_then(|s: String| Person::from_str(&s))
    })
}

#[derive(Debug)]
#[allow(unused)]
struct Person {
    name: String,
    surname: String,
}

impl FromStr for Person {
    type Err = PersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',').map(str::to_owned);

        let name = it.next().ok_or(PersonError::Parse)?;
        let surname = it.next().ok_or(PersonError::Parse)?;

        if it.next().is_some() {
            return Err(PersonError::Parse);
        }

        Ok(Person { name, surname })
    }
}

#[derive(Debug)]
enum PersonError {
    Io(io::Error),
    Utf8(FromUtf8Error),
    Parse,
}

impl fmt::Display for PersonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PersonError::Io(e) => write!(f, "cannot read file: {e}"),
            PersonError::Utf8(e) => write!(f, "not a valid string: {e}"),
            PersonError::Parse => write!(f, "invalid person format"),
        }
    }
}

impl Error for PersonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PersonError::Io(e) => Some(e),
            PersonError::Utf8(e) => Some(e),
            PersonError::Parse => None,
        }
    }
}

impl From<io::Error> for PersonError {
    fn from(err: io::Error) -> PersonError {
        PersonError::Io(err)
    }
}
impl From<FromUtf8Error> for PersonError {
    fn from(err: FromUtf8Error) -> PersonError {
        PersonError::Utf8(err)
    }
}
