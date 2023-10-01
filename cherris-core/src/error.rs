#[derive(Debug, PartialEq)]
pub enum Error {
    ParseColor,
    ParseFile,
    ParseRank,
    InvalidFen,
}
