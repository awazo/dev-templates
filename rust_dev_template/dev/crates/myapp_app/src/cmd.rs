use std::env;

pub enum Cmd {
    Migrate,
    None,
}

pub fn parse() -> Cmd {
    match env::args().nth(1).as_deref() {
        Some("migrate") => Cmd::Migrate,
        _ => Cmd::None,
    }
}
