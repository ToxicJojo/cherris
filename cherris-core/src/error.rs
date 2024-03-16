#[derive(Debug, PartialEq)]
pub enum Error {
    ParseColor,
    ParseFile,
    ParseRank,
    ParseRole,
    ParsePiece,
    InvalidFen,
    InvalidMove,
}
