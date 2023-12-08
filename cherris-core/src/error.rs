#[derive(Debug, PartialEq)]
pub enum Error {
    ParseColor,
    ParseFile,
    ParseRank,
    ParseRole,
    InvalidFen,
    InvalidMove,
}
