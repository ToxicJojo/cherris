use crate::{Bitboard, Square};

pub static RAYS: [[Bitboard; Square::COUNT]; 8] = generate_rays();
pub const DIRECTIONS: [i8; 8] = [8, 9, 1, -7, -8, -9, -1, 7];

pub static ROOK_RAYS: [Bitboard; Square::COUNT] = generate_rook_rays();
pub static BISHOP_RAYS: [Bitboard; Square::COUNT] = generate_bishop_rays();

pub static RAY_BETWEEN: [[Bitboard; Square::COUNT]; Square::COUNT] = generate_rays_between();

const fn generate_rays_between() -> [[Bitboard; Square::COUNT]; Square::COUNT] {
    let mut rays_between = [[Bitboard::EMPTY; 64]; 64];

    let mut from = 0;
    let mut to = 0;

    while from < 64 {
        while to < 64 {
            let ray_between = ray_between(from, to);
            rays_between[from as usize][to as usize] = ray_between;

            to += 1;
        }
        to = 0;
        from += 1;
    }

    rays_between
}

pub const fn generate_rook_rays() -> [Bitboard; Square::COUNT] {
    let all_rays = generate_rays();

    let mut rays = [Bitboard::EMPTY; Square::COUNT];
    let mut sqaure = 0;

    while sqaure < 64 {
        let ray_north = all_rays[0][sqaure];
        let ray_east = all_rays[2][sqaure];
        let ray_south = all_rays[4][sqaure];
        let ray_west = all_rays[6][sqaure];

        rays[sqaure] = Bitboard::new(ray_north.0 | ray_east.0 | ray_south.0 | ray_west.0);

        sqaure += 1;
    }

    rays
}

pub const fn generate_bishop_rays() -> [Bitboard; Square::COUNT] {
    let all_rays = generate_rays();

    let mut rays = [Bitboard::EMPTY; Square::COUNT];
    let mut sqaure = 0;

    while sqaure < 64 {
        let ray_north_east = all_rays[1][sqaure];
        let ray_south_east = all_rays[3][sqaure];
        let ray_south_west = all_rays[5][sqaure];
        let ray_north_west = all_rays[7][sqaure];

        rays[sqaure] = Bitboard::new(
            ray_north_east.0 | ray_south_east.0 | ray_south_west.0 | ray_north_west.0,
        );

        sqaure += 1;
    }

    rays
}

const fn generate_rays() -> [[Bitboard; Square::COUNT]; 8] {
    let mut rays = [[Bitboard::EMPTY; 64]; 8];
    let mut sqaure = 0;
    let mut direction = 0;

    while sqaure < 64 {
        while direction < DIRECTIONS.len() {
            let ray = ray(sqaure, DIRECTIONS[direction], 0);
            rays[direction][sqaure as usize] = ray;

            direction += 1;
        }
        sqaure += 1;
        direction = 0;
    }

    rays
}

pub const fn ray_between(from: i8, to: i8) -> Bitboard {
    if from == to {
        return Bitboard::EMPTY;
    }

    let from_file = from % 8;
    let from_rank = from / 8;
    let to_file = to % 8;
    let to_rank = to / 8;
    let file_diff = to_file - from_file;
    let rank_diff = to_rank - from_rank;

    if file_diff.abs() == rank_diff.abs() {
        if file_diff > 0 && rank_diff > 0 {
            return ray(from, DIRECTIONS[1], 1 << to);
        } else if file_diff > 0 && rank_diff < 0 {
            return ray(from, DIRECTIONS[3], 1 << to);
        } else if file_diff < 0 && rank_diff < 0 {
            return ray(from, DIRECTIONS[5], 1 << to);
        } else if file_diff < 0 && rank_diff > 0 {
            return ray(from, DIRECTIONS[7], 1 << to);
        }
    } else if file_diff == 0 || rank_diff == 0 {
        if file_diff > 0 {
            return ray(from, DIRECTIONS[2], 1 << to);
        } else if file_diff < 0 {
            return ray(from, DIRECTIONS[6], 1 << to);
        }

        if rank_diff > 0 {
            return ray(from, DIRECTIONS[0], 1 << to);
        } else if rank_diff < 0 {
            return ray(from, DIRECTIONS[4], 1 << to);
        }
    }

    Bitboard::EMPTY
}

pub const fn ray(sqaure: i8, direction: i8, occupants: u64) -> Bitboard {
    let mut ray = 0;
    let mut i = 0;
    let mut file_from = sqaure % 8;

    loop {
        i += 1;
        let next_index = sqaure + direction * i;

        let file_target = next_index % 8;

        let file_diff = i8::abs(file_from - file_target);

        if next_index < 0 || next_index > 63 || file_diff > 1 {
            break;
        }

        file_from = file_target;

        let square_bb = 1 << next_index;

        ray |= square_bb;

        if square_bb & occupants != 0 {
            break;
        }
    }

    Bitboard::new(ray)
}

pub const fn xray(sqaure: i8, direction: i8, occupants: u64) -> Bitboard {
    let mut ray = 0;
    let mut i = 0;
    let mut file_from = sqaure % 8;
    let mut hit_count = 0;

    loop {
        i += 1;
        let next_index = sqaure + direction * i;

        let file_target = next_index % 8;

        let file_diff = i8::abs(file_from - file_target);

        if next_index < 0 || next_index > 63 || file_diff > 1 {
            break;
        }

        file_from = file_target;

        let square_bb = 1 << next_index;

        ray |= square_bb;

        if square_bb & occupants != 0 {
            if hit_count == 1 {
                break;
            }
            hit_count += 1;
        }
    }

    Bitboard::new(ray)
}
