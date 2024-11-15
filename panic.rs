#![allow(dead_code)]

#[derive(Debug)]
enum Error {
    NotFound(String),
}

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
}

fn get_user(id: i32) -> Result<User, Error> {
    if id == 2 {
        return Err(Error::NotFound("404 - User does not exist".into()));
    }
    Ok(User {
        id,
        name: "Kyle".into(),
    })
}

fn main() -> Result<(), Error> {
    let user = get_user(1)?;
    dbg!(user);
    let user2 = get_user(2)?;
    dbg!(user2);
    Ok(())
}
