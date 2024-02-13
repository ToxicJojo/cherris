use arrayvec::ArrayVec;

use crate::Move;

/// The maximum number of moves in a position.
const MAX_MOVES: usize = 256;

pub type MoveList = ArrayVec<Move, MAX_MOVES>;
