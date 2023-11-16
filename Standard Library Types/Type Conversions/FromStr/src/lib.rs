use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
pub enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for ParsePersonError {
    fn from(err: ParseIntError) -> Self {
        ParsePersonError::ParseInt(err)
    }
}

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.len() == 0 {
            return Err(ParsePersonError::Empty)
        }
        let parts: Vec<&str> = s.split(',').collect();
        /* Add a condition for the cases when there is a wrong number of fields and return an appropriate error */
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen)
        }
        /* Extract the first element from the split operation and use it as the name. Return an appropriate error if the field is empty. */
        let name = parts[0];
        if name.len() == 0 {
            return Err(ParsePersonError::NoName)
        }
        /* Extract the other element from the split and parse it as age. Return an appropriate error if something goes wrong. */
        let age = parts[1].parse::<usize>();
        if age.is_err() {
            return Err(ParsePersonError::ParseInt(age.err().unwrap()))
        }

        /* Return a Result of a Person object if everything goes well. */
        Result::Ok(Person {
            name: String::from(name),
            age: age.unwrap()
        })
    }
}
