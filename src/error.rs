#[derive(Debug, PartialEq)]
pub enum Error {
    ParsingFailed(String),
    PayloadEmpty,
}
