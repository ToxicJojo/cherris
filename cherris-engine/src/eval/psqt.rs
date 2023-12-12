use cherris_core::{Color, Position, Role, Square};

#[rustfmt::skip]
const PAWN_PSQT: [i16; Square::COUNT] =[
     0,   0,   0,   0,   0,   0,   0,   0,
    60,  60,  60,  60,  70,  60,  60,  60,
    40,  40,  40,  50,  60,  40,  40,  40,
    20,  20,  20,  40,  50,  20,  20,  20,
     5,   5,  15,  30,  40,  10,   5,   5,
     5,   5,  10,  20,  30,   5,   5,   5,
     5,   5,   5, -30, -30,   5,   5,   5,
     0,   0,   0,   0,   0,   0,   0,   0
];

#[rustfmt::skip]
const KNIGHT_PSQT: [i16; Square::COUNT] =[
    -20, -10,  -10,  -10,  -10,  -10,  -10,  -20,
    -10,  -5,   -5,   -5,   -5,   -5,   -5,  -10,
    -10,  -5,   15,   15,   15,   15,   -5,  -10,
    -10,  -5,   15,   15,   15,   15,   -5,  -10,
    -10,  -5,   15,   15,   15,   15,   -5,  -10,
    -10,  -5,   10,   15,   15,   15,   -5,  -10,
    -10,  -5,   -5,   -5,   -5,   -5,   -5,  -10,
    -20,   0,  -10,  -10,  -10,  -10,    0,  -20
];

#[rustfmt::skip]
const BISHOP_PSQT: [i16; Square::COUNT] =[
    -20,    0,    0,    0,    0,    0,    0,  -20,
    -15,    0,    0,    0,    0,    0,    0,  -15,
    -10,    0,    0,    5,    5,    0,    0,  -10,
    -10,   10,   10,   30,   30,   10,   10,  -10,
      5,    5,   10,   25,   25,   10,    5,    5,
      5,    5,    5,   10,   10,    5,    5,    5,
    -10,    5,    5,   10,   10,    5,    5,  -10,
    -20,  -10,  -10,  -10,  -10,  -10,  -10,  -20
];

#[rustfmt::skip]
const ROOK_PSQT: [i16; Square::COUNT] = [
    0,   0,   0,   0,   0,   0,   0,   0,
   15,  15,  15,  20,  20,  15,  15,  15,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,  10,  10,  10,   0,   0
];

#[rustfmt::skip]
const QUEEN_PSQT: [i16; Square::COUNT] = [
    -30,  -20,  -10,  -10,  -10,  -10,  -20,  -30,
    -20,  -10,   -5,   -5,   -5,   -5,  -10,  -20,
    -10,   -5,   10,   10,   10,   10,   -5,  -10,
    -10,   -5,   10,   20,   20,   10,   -5,  -10,
    -10,   -5,   10,   20,   20,   10,   -5,  -10,
    -10,   -5,   -5,   -5,   -5,   -5,   -5,  -10,
    -20,  -10,   -5,   -5,   -5,   -5,  -10,  -20,
    -30,  -20,  -10,  -10,  -10,  -10,  -20,  -30 
];

#[rustfmt::skip]
const KING_PSQT: [i16; Square::COUNT] = [
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,    20,   20,    0,    0,    0,
    0,    0,     0,    20,   20,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,   -10,  -10,    0,    0,    0,
    0,    0,    20,   -10,  -10,    0,   20,    0,
];

const PSQT: [[i16; Square::COUNT]; Role::COUNT] = [
    PAWN_PSQT,
    KNIGHT_PSQT,
    BISHOP_PSQT,
    ROOK_PSQT,
    QUEEN_PSQT,
    KING_PSQT,
];

#[rustfmt::skip]
pub const FLIP: [usize; 64] = [
    56, 57, 58, 59, 60, 61, 62, 63,
    48, 49, 50, 51, 52, 53, 54, 55,
    40, 41, 42, 43, 44, 45, 46, 47,
    32, 33, 34, 35, 36, 37, 38, 39,
    24, 25, 26, 27, 28, 29, 30, 31,
    16, 17, 18, 19, 20, 21, 22, 23,
     8,  9, 10, 11, 12, 13, 14, 15,
     0,  1,  2,  3,  4,  5,  6,  7,
];

pub fn eval_psqt(position: &Position) -> i16 {
    let mut eval = 0;

    let white = position.board.color[Color::White];
    let black = position.board.color[Color::Black];

    for role in Role::iter() {
        let white_role = white & position.board.role[role];
        let black_role = black & position.board.role[role];

        for square_white in white_role {
            eval += PSQT[role.to_index()][FLIP[square_white.to_index()]];
        }

        for square_black in black_role {
            eval -= PSQT[role.to_index()][square_black.to_index()];
        }
    }

    eval
}
