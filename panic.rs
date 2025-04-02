#![allow(dead_code, clippy::redundant_closure)]

use std::{panic, sync::LazyLock};

fn main() -> Result<(), Error> {
    let _hook_disable = PanicHookDisable::new();
    panic::catch_unwind(|| {
        for text in (1..=3).map(|x| process_text(x)) {
            println!("{text}");
        }
    })
    .map_err(|err| {
        // println!("totally panicked");
        let message = match err.downcast::<String>() {
            Ok(message) => *message,
            Err(err) => format!("{err:?}"),
        };
        Error::Panic(message)
    })
}

fn process_text(id: i32) -> String {
    let text = retrieve_text(id).unwrap_or_else(|err| panic!("{err:?}"));
    let mut codes = str_to_chars(&text);
    rotate_back(&mut codes);
    chars_to_string(&codes)
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

#[derive(Debug)]
enum Error {
    NotFound(String),
    Panic(String),
}

struct PanicHookDisable {
    former: Box<dyn Fn(&panic::PanicHookInfo) + Sync + Send + 'static>,
}

impl PanicHookDisable {
    fn new() -> Self {
        let former = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        Self { former }
    }
}

impl Drop for PanicHookDisable {
    fn drop(&mut self) {
        panic::set_hook(std::mem::replace(&mut self.former, Box::new(|_| {})));
    }
}
