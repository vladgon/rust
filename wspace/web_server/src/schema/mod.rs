use std::fmt::{Error, Formatter};

// #[derive(Debug)]
pub struct Customer<'a> {
    name: &'a str,
    last_name: &'a str,
}


impl<'a> Customer<'a> {
    pub const fn new(name: &'a str, last_name: &'a str) -> Self {
        Self {
            name,
            last_name,
        }
    }
}

impl core::fmt::Display for Customer<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "__{}__{}_", self.last_name, self.name)
    }
}

pub fn schema() {
    let name = "John";
    let last_name = "Smith";
    println!("Schema {}", Customer::new(name, last_name));
}