use crate::Move;

#[derive(Debug, Clone, PartialEq)]
pub enum GameAction {
    /// Make a move on the board.
    Move(Move),
    /// Offer a draw to the opponent.
    OfferDraw,
    /// Resign the game.
    Resign,
    /// Accept a draw offer.
    AcceptDraw,
    /// Decline a draw offer.
    DeclineDraw,
    /// Claim a draw (e.g., by repetition or 50-move rule).
    ClaimDraw,
}
