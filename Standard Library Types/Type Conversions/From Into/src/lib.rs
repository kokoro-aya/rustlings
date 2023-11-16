#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: usize,
}

impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}


impl From<&str> for Person {
    fn from(s: &str) -> Person {
        if s.len() == 0 {
            return Person::default()
        }
        let frags: Vec<&str> = s.split(',').collect();
        let name = frags[0];
        let age = frags[1].parse::<usize>();
        match (name, age) {
            (_, Ok(num)) => Person { name: String::from(name), age: num },
            _ => Person::default()
        }
    }
}

