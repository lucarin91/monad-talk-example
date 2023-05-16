use std::io;
use std::str::FromStr;
use std::string::FromUtf8Error;
use tokio::fs;

#[tokio::main]
async fn main() {
    const PATH: &str = "person.txt";

    match read_person(PATH).await {
        Ok(p) => println!("person: {:?}", p),
        Err(e) => eprintln!("error: {e}"),
    }
}

#[derive(thiserror::Error, Debug)]
#[allow(unused)]
enum PersonError {
    #[error("cannot read file: {0}")]
    Io(#[from] io::Error),
    #[error("not a valid string: {0}")]
    Utf8(#[from] FromUtf8Error),
    #[error("invalid person format")]
    Parse,
}

async fn read_person(path: &str) -> Result<Person, PersonError> {
    let b: Vec<u8> = fs::read(path).await?;
    let s: String = String::from_utf8(b)?;
    let p: Person = Person::from_str(&s)?;
    Ok(p)
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
