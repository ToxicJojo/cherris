use crate::{Color, Position, Role, Square};

const ZOBRIST_SIZE: usize = Role::COUNT * Color::COUNT * Square::COUNT;
static ZOBRIST_PIECES: [u64; ZOBRIST_SIZE] = generate_zobrist_pieces();
static ZOBRIST_BLACK: u64 = Prng::rand(353284).0;
static ZOBRIST_CASTLING: [u64; 16] = generate_zobrist_castling();
static ZOBRIST_EN_PASSANT: [u64; 8] = generate_zobrist_en_passant();

const fn generate_zobrist_pieces() -> [u64; ZOBRIST_SIZE] {
    let mut zobrist_keys = [0; ZOBRIST_SIZE];
    let mut index = 0;
    let mut seed = 1070372;

    while index < ZOBRIST_SIZE {
        let (key, next_seed) = Prng::rand(seed);
        zobrist_keys[index] = key;
        seed = next_seed;
        index += 1;
    }

    zobrist_keys
}

const fn generate_zobrist_castling() -> [u64; 16] {
    let mut zobrist_keys = [0; 16];
    let mut index = 0;
    let mut seed = 832053;

    while index < 4 {
        let (key, next_seed) = Prng::rand(seed);
        zobrist_keys[index] = key;
        seed = next_seed;
        index += 1;
    }

    zobrist_keys
}

const fn generate_zobrist_en_passant() -> [u64; 8] {
    let mut zobrist_keys = [0; 8];
    let mut index = 0;
    let mut seed = 7234975;

    while index < 8 {
        let (key, next_seed) = Prng::rand(seed);
        zobrist_keys[index] = key;
        seed = next_seed;
        index += 1;
    }

    zobrist_keys
}

pub fn zobrist_hash(position: &Position) -> u64 {
    let mut hash = 0;
    for square in position.board.occupied {
        let piece = position.board.piece_on(square).unwrap();
        let key =
            ZOBRIST_PIECES[piece.color.to_index() * piece.role.to_index() + square.to_index()];

        hash ^= key;
    }

    if position.color_to_move == Color::Black {
        hash ^= ZOBRIST_BLACK;
    }

    hash ^= ZOBRIST_CASTLING
        [position.castling_rights[0].to_index() + position.castling_rights[1].to_index()];

    if let Some(en_passant_sqaure) = position.en_passant_square {
        let file = en_passant_sqaure.to_index() % 8;
        hash ^= ZOBRIST_EN_PASSANT[file];
    }

    hash
}

struct Prng {}

impl Prng {
    pub const fn rand(seed: u64) -> (u64, u64) {
        let mut s = seed;
        s ^= s >> 12;
        s ^= s << 12;
        s ^= s >> 27;

        (s, s.wrapping_mul(2685821657736338717))
    }
}

#[cfg(test)]
mod tests {
    use crate::CastlingRights;

    use super::*;

    #[test]
    fn zobrist_start_pos() {
        let position = Position::default();

        let hash = zobrist_hash(&position);

        assert_eq!(hash, 3821593027773340683);
    }

    #[test]
    fn zobrist_diff_ep() {
        let mut position = Position::default();
        let hash = zobrist_hash(&position);
        position.en_passant_square = Some(Square::E3);
        let hash_ep = zobrist_hash(&position);

        assert_ne!(hash, hash_ep);
    }

    #[test]
    fn zobrist_diff_castling() {
        let mut position = Position::default();
        let hash = zobrist_hash(&position);
        position.castling_rights[0] = CastlingRights::NoSide;
        let hash_castling = zobrist_hash(&position);

        assert_ne!(hash, hash_castling);
    }

    #[test]
    fn zobrist_diff_color() {
        let mut position = Position::default();
        let hash = zobrist_hash(&position);
        position.color_to_move = Color::Black;
        let hash_color = zobrist_hash(&position);

        assert_ne!(hash, hash_color);
    }

    #[test]
    fn zobrist_diff_piece() {
        let mut position = Position::default();
        let hash = zobrist_hash(&position);
        position.make_move(position.legal_moves()[0]);
        let hash_piece = zobrist_hash(&position);

        assert_ne!(hash, hash_piece);
    }
}
