#![allow(dead_code, clippy::redundant_closure)]

use std::sync::LazyLock;

fn main() -> Result<(), Error> {
    std::panic::catch_unwind(|| {
        for result in (1..=3).map(|x| retrieve_text(x)) {
            match result {
                Ok(text) => {
                    let mut codes = str_to_chars(&text);
                    rotate_back(&mut codes);
                    println!("{}", chars_to_string(&codes));
                }
                Err(_) => println!("{:?}", result),
            }
        }
    })
    .map_err(|err| {
        println!("totally panicked");
        Error::Panic(err)
    })
}

#[derive(Debug)]
enum Error {
    NotFound(String),
    Panic(Box<dyn std::any::Any + Send>),
}

// static TEXTS: [&str; 2] = ["smile", "tears"];
static TEXTS: LazyLock<Vec<&str>> = LazyLock::new(|| vec!["smile", "tears"]);

fn retrieve_text(id: i32) -> Result<String, Error> {
    match TEXTS.get((id - 1) as usize) {
        Some(text) => Ok(text.to_string()),
        _ => Err(Error::NotFound(format!("404 - Not found: {id}"))),
    }
}

fn rotate_back<T: Copy>(vals: &mut [T]) {
    let len = vals.len();
    // if len == 0 {
    //     return;
    // }
    let first = vals[0];
    for index in 1..len {
        vals[index - 1] = vals[index];
    }
    vals[len - 1] = first;
}

fn chars_to_string(codes: &[char]) -> String {
    codes.iter().collect()
}

fn str_to_chars(text: &str) -> Vec<char> {
    text.chars().collect()
}
