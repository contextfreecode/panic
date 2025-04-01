#![allow(dead_code, clippy::redundant_closure)]

fn main() -> Result<(), Error> {
    for result in (1..=3).map(|x| retrieve_text(x)) {
        match result {
            Ok(text) => {
                let mut codes = str_to_codes(&text);
                rotate_back(&mut codes);
                println!("{}", codes_to_string(&codes));
            }
            Err(_) => println!("{:?}", result),
        }
    }
    Ok(())
}

#[derive(Debug)]
enum Error {
    NotFound(String),
}

const TEXTS: [&str; 2] = ["smile", "tears"];

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

fn codes_to_string(codes: &[u32]) -> String {
    // Panics on bad code points!
    codes.iter().map(|c| char::from_u32(*c).unwrap()).collect()
}

fn str_to_codes(text: &str) -> Vec<u32> {
    text.chars().map(|c| c as u32).collect()
}
