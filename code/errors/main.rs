/*
Rust defines 2 types of errors:

- recoverable: Result<T, E>
- unrecoverable: panic!
*/

use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::fs;
use std::fs::File;

fn main() {
    println!("Hello, Errors!");
    let do_panic = false;
    if do_panic {
        panic!("panic here!");
    }
    let v = vec![5, 7, 11];
    println!("{}", v[1]);

    let file_name = "hw.txt";
    let f = File::open(file_name);

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    match read_user_from_file() {
        Ok(user) => println!("user: {}", user),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}

fn read_user_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hw.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hw.txt")
}
