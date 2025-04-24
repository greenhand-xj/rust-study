use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::num::ParseIntError;

fn main() {
    // let file = File::open("1.txt");
    // match file {
    //     Ok(file) => {},
    //     Err(err) => {
    //         println!("Error opening file, {:?}", err);
    //     }
    // }
    // let file = File::open("1.txt");
    // let fc = match file {
    //     Ok(file) => file,
    //     Err(e) => {
    //          match e.kind() {
    //             ErrorKind::NotFound => match File::create("1.txt") {
    //                 Ok(f) => f,
    //                 Err(_) => panic!("Cannot create file!"),
    //             },
    //             other => panic!("{}", other)
    //         }
    //     }
    // };
    //  fs::read_to_string("1.txt");
}

fn read_username_from_file() -> Option<String> {
    let mut username_file = File::open("1.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Some(username)
}

fn read_to_string() -> Result<String,MyError> {
    let mut name = String::new();
    let file = File::open("1.txt")?.read_to_string(&mut name)?;
    let num = "55".parse()?;
    return Ok(name);
}

pub enum MyError {
    Io(Error),
    ParseInt(ParseIntError),
    Other(String),
}
impl From<Error> for MyError {
    fn from(value: Error) -> Self {
        return MyError::Io(value);
    }
}
impl From<ParseIntError> for MyError {
    fn from(value: ParseIntError) -> Self {
        return MyError::ParseInt(value);
    }
}