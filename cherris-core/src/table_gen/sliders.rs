use crate::{Bitboard, DIRECTIONS, Square, ray, xray};
use core::arch::x86_64::_pdep_u64;
use std::sync::LazyLock;

const A_FILE: u64 = 0x0101010101010101;
const H_FILE: u64 = 0x0101010101010101 << 7;
const FIRST_RANK: u64 = 0x00000000000000FF;
const EIGHT_RANK: u64 = 0xFF00000000000000;

pub static ROOK_MAKS: [u64; Square::COUNT] = generate_rook_masks();
pub static ROOK_OFFSETS: [u64; Square::COUNT] = generate_rook_offsets();
pub static ROOK_ATTACKS: LazyLock<[Bitboard; 102400]> = LazyLock::new(generate_rook_attacks);
pub static ROOK_XRAY_ATTACKS: LazyLock<[Bitboard; 102400]> =
    LazyLock::new(generate_rook_xray_attacks);

pub static BISHOP_MASKS: [u64; Square::COUNT] = generate_bishop_masks();
pub static BISHOP_OFFSETS: [u64; Square::COUNT] = generate_bishop_offsets();
pub static BISHOP_ATTACKS: LazyLock<[Bitboard; 5248]> = LazyLock::new(generate_bishop_attacks);
pub static BISHOP_XRAY_ATTACKS: LazyLock<[Bitboard; 5248]> =
    LazyLock::new(generate_bishop_xray_attacks);

pub fn generate_lookup_tables() {
    let _ = &*ROOK_ATTACKS;
    let _ = &*ROOK_XRAY_ATTACKS;
    let _ = &*BISHOP_ATTACKS;
    let _ = &*BISHOP_XRAY_ATTACKS;
}

pub const fn generate_bishop_masks() -> [u64; Square::COUNT] {
    let mut masks = [0; Square::COUNT];
    let mut square = 0;

    while square < 64 {
        let mask = generate_bishop_mask(square);
        masks[square as usize] = mask;

        square += 1;
    }

    masks
}

const fn generate_bishop_mask(square: i8) -> u64 {
    let mut mask = 0;
    let mut direction = 1;

    while direction < DIRECTIONS.len() {
        let ray = match direction {
            1 => ray(square, DIRECTIONS[direction], 0).0 & !(EIGHT_RANK | H_FILE),
            3 => ray(square, DIRECTIONS[direction], 0).0 & !(FIRST_RANK | H_FILE),
            5 => ray(square, DIRECTIONS[direction], 0).0 & !(FIRST_RANK | A_FILE),
            7 => ray(square, DIRECTIONS[direction], 0).0 & !(EIGHT_RANK | A_FILE),
            _ => unreachable!(),
        };

        mask |= ray;
        direction += 2;
    }

    mask
}

const fn generate_offsets(masks: [u64; Square::COUNT]) -> [u64; Square::COUNT] {
    let mut offsets = [0; Square::COUNT];

    let mut square = 0;
    let mut offset = 0;
    while square < 64 {
        offsets[square] = offset;

        let mask = masks[square];
        offset += 1 << mask.count_ones();

        square += 1;
    }

    offsets
}

pub const fn generate_bishop_offsets() -> [u64; Square::COUNT] {
    let bishop_masks = generate_bishop_masks();

    generate_offsets(bishop_masks)
}

pub fn generate_bishop_xray_attacks() -> [Bitboard; 5248] {
    let mut attacks = [Bitboard::EMPTY; 5248];
    let bishop_masks = generate_bishop_masks();

    let mut square = 0;
    let mut base = 0;
    while square < 64 {
        let mut index = 0;
        let mask = bishop_masks[square];
        let max = 1 << mask.count_ones();
        while index < max {
            let occupants = unsafe { _pdep_u64(index, mask) };

            attacks[base] = generate_bishop_xray_attack(square as i8, occupants);

            index += 1;
            base += 1;
        }

        square += 1;
    }

    attacks
}

pub fn generate_bishop_attacks() -> [Bitboard; 5248] {
    let mut attacks = [Bitboard::EMPTY; 5248];
    let bishop_masks = generate_bishop_masks();

    let mut square = 0;
    let mut base = 0;
    while square < 64 {
        let mut index = 0;
        let mask = bishop_masks[square];
        let max = 1 << mask.count_ones();
        while index < max {
            let occupants = unsafe { _pdep_u64(index, mask) };

            attacks[base] = generate_bishop_attack(square as i8, occupants);

            index += 1;
            base += 1;
        }

        square += 1;
    }

    attacks
}

pub const fn generate_bishop_attack(square: i8, occupants: u64) -> Bitboard {
    let mut attacks = 0;
    let mut direction = 1;

    while direction < DIRECTIONS.len() {
        let ray = ray(square, DIRECTIONS[direction], occupants);

        attacks |= ray.0;
        direction += 2;
    }

    Bitboard(attacks)
}

pub const fn generate_bishop_xray_attack(square: i8, occupants: u64) -> Bitboard {
    let mut attacks = 0;
    let mut direction = 1;

    while direction < DIRECTIONS.len() {
        let xray = xray(square, DIRECTIONS[direction], occupants);

        attacks |= xray.0;
        direction += 2;
    }

    Bitboard(attacks)
}

pub const fn generate_rook_offsets() -> [u64; Square::COUNT] {
    let rook_masks = generate_rook_masks();

    generate_offsets(rook_masks)
}

pub fn generate_rook_xray_attacks() -> [Bitboard; 102400] {
    let mut attacks = [Bitboard::EMPTY; 102400];
    let rook_masks = generate_rook_masks();

    let mut square = 0;
    let mut base = 0;
    while square < 64 {
        let mut index = 0;
        let mask = rook_masks[square];
        let max = 1 << mask.count_ones();
        while index < max {
            let occupants = unsafe { _pdep_u64(index, mask) };

            attacks[base] = generate_rook_xray_attack(square as i8, occupants);

            index += 1;
            base += 1;
        }

        square += 1;
    }

    attacks
}

pub fn generate_rook_attacks() -> [Bitboard; 102400] {
    let mut attacks = [Bitboard::EMPTY; 102400];
    let rook_masks = generate_rook_masks();

    let mut square = 0;
    let mut base = 0;
    while square < 64 {
        let mut index = 0;
        let mask = rook_masks[square];
        let max = 1 << mask.count_ones();
        while index < max {
            let occupants = unsafe { _pdep_u64(index, mask) };

            attacks[base] = generate_rook_attack(square as i8, occupants);

            index += 1;
            base += 1;
        }

        square += 1;
    }

    attacks
}

pub const fn generate_rook_masks() -> [u64; Square::COUNT] {
    let mut masks = [0; Square::COUNT];
    let mut square = 0;

    while square < 64 {
        let mask = generate_rook_mask(square);
        masks[square as usize] = mask;

        square += 1;
    }

    masks
}

pub const fn generate_rook_mask(square: i8) -> u64 {
    let mut attacks = 0;
    let mut direction = 0;

    while direction < DIRECTIONS.len() {
        let ray = match direction {
            0 => ray(square, DIRECTIONS[direction], 0).0 & !EIGHT_RANK,
            2 => ray(square, DIRECTIONS[direction], 0).0 & !H_FILE,
            4 => ray(square, DIRECTIONS[direction], 0).0 & !FIRST_RANK,
            6 => ray(square, DIRECTIONS[direction], 0).0 & !A_FILE,
            _ => unreachable!(),
        };

        attacks |= ray;
        direction += 2;
    }

    attacks
}

pub const fn generate_rook_attack(square: i8, occupants: u64) -> Bitboard {
    let mut attacks = 0;
    let mut direction = 0;

    while direction < DIRECTIONS.len() {
        let ray = ray(square, DIRECTIONS[direction], occupants);

        attacks |= ray.0;
        direction += 2;
    }

    Bitboard(attacks)
}

pub const fn generate_rook_xray_attack(square: i8, occupants: u64) -> Bitboard {
    let mut attacks = 0;
    let mut direction = 0;

    while direction < DIRECTIONS.len() {
        let xray = xray(square, DIRECTIONS[direction], occupants);

        attacks |= xray.0;
        direction += 2;
    }

    Bitboard(attacks)
}
