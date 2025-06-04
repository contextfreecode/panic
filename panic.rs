#![allow(dead_code, clippy::redundant_closure, clippy::type_complexity)]

use std::panic;

fn main() {
    run();
    println!("Still alive.");
}

fn run() {
    let _hook_disable = PanicHookDisable::new();
    if let Err(err) = panic::catch_unwind(|| {
        for text in (1..=3).map(|x| process_text(x)) {
            println!("{}", text);
        }
    }) {
        let message = match err.downcast::<String>() {
            Ok(message) => *message,
            Err(err) => format!("{err:?}"),
        };
        println!("panic: {message}");
    }
}

fn process_text(id: i32) -> String {
    let text = retrieve_text(id).unwrap_or_else(|err| panic!("{err:?}"));
    text.chars().rev().collect()
}

static TEXTS: &[&str] = &["tar", "flow"];
// static TEXTS: &[&str] = &["tar", "flow", "🐻‍❄️❤️🦭"];

fn retrieve_text(id: i32) -> Result<String, Error> {
    match TEXTS.get((id - 1) as usize) {
        Some(text) => Ok(text.to_string()),
        _ => Err(Error::NotFound(format!("404 - Not found: {id}"))),
    }
}

#[derive(Debug)]
enum Error {
    NotFound(String),
}

struct PanicHookDisable {
    former: Option<Box<dyn Fn(&panic::PanicHookInfo) + Sync + Send + 'static>>,
}

impl PanicHookDisable {
    fn new() -> Self {
        let former = Some(panic::take_hook());
        panic::set_hook(Box::new(|_| {
            // std::process::exit(1);
        }));
        Self { former }
    }
}

impl Drop for PanicHookDisable {
    fn drop(&mut self) {
        panic::set_hook(self.former.take().unwrap());
    }
}
