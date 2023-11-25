use sdl2::{
    image::{InitFlag, LoadTexture},
    mouse::MouseButton,
    rect::{Point, Rect},
    render::Texture,
    video::Window,
};
use std::time::Duration;

use cherris_core::{generate_lookup_tables, File, Game, Move, Piece, Position, Rank, Role, Square};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const SQAURE_SIZE: u32 = WINDOW_WIDTH / 8;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let _sld_image = sdl2::image::init(InitFlag::PNG).unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Cherris", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture("cherris-game/assets/Pieces.png")
        .unwrap();

    generate_lookup_tables();
    let mut game = Game::new();
    let mut hovered_square: Option<Square> = None;
    let mut dragged_piece: Option<Piece> = None;
    let mut dragged_starting_sqaure: Option<Square> = None;
    let mut mouse_position = Point::new(0, 0);

    let mut moves = game.legal_moves();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        draw_board(&mut canvas);
        if let Some(piece) = dragged_piece {
            let mut piece_moves = moves.to_vec();
            piece_moves.retain(|m| match m {
                Move::Standard { from, .. } => Some(from) == dragged_starting_sqaure.as_ref(),
                Move::EnPassant { from, .. } => Some(*from) == dragged_starting_sqaure,
                Move::CastleLong => piece.role == Role::King && piece.color == game.color_to_move(),
                Move::CastleShort => {
                    piece.role == Role::King && piece.color == game.color_to_move()
                }
            });
            draw_possible_moves(&mut canvas, piece_moves, game.color_to_move());
        }
        draw_position(
            &mut canvas,
            &texture,
            game.position(),
            dragged_starting_sqaure,
        );

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    game = Game::new();
                    moves = game.legal_moves();
                }
                Event::MouseMotion { x, y, .. } => {
                    hovered_square = None;
                    mouse_position = Point::new(x, y);
                    for rank in Rank::iter() {
                        for file in File::iter() {
                            let rank_index = rank.to_index();
                            let file_index = file.to_index();

                            let sqaure_rect = Rect::new(
                                (SQAURE_SIZE * file_index as u32).try_into().unwrap(),
                                (SQAURE_SIZE * (7 - rank_index) as u32).try_into().unwrap(),
                                SQAURE_SIZE,
                                SQAURE_SIZE,
                            );

                            if sqaure_rect.contains_point(mouse_position) {
                                let square = Square::from((*file, *rank));
                                hovered_square = Some(square);
                            }
                        }
                    }
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        if let Some(square) = hovered_square {
                            if let Some(piece) = game.position().board.piece_on(square) {
                                if piece.color == game.color_to_move() {
                                    dragged_starting_sqaure = hovered_square;
                                    dragged_piece = Some(piece);
                                }
                            }
                        }
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        if dragged_piece.is_some() {
                            if let Some(sqaure) = dragged_starting_sqaure {
                                if let Some(hoverd_sqaure) = hovered_square {
                                    let mut piece_moves = moves.to_vec();
                                    piece_moves.retain(|m| match m {
                                        Move::Standard { from, to, .. } => {
                                            *from == sqaure && hoverd_sqaure == *to
                                        }
                                        Move::EnPassant { from, to, .. } => {
                                            *from == sqaure && hoverd_sqaure == *to
                                        }
                                        Move::CastleShort => {
                                            let to = match game.color_to_move() {
                                                cherris_core::Color::White => Square::G1,
                                                cherris_core::Color::Black => Square::G8,
                                            };
                                            hoverd_sqaure == to
                                        }
                                        Move::CastleLong => {
                                            let to = match game.color_to_move() {
                                                cherris_core::Color::White => Square::C1,
                                                cherris_core::Color::Black => Square::C8,
                                            };
                                            hoverd_sqaure == to
                                        }
                                    });

                                    if let Some(choosen_move) = piece_moves.get(0) {
                                        game.make_move(*choosen_move);
                                        moves = game.legal_moves();
                                        if moves.is_empty() {
                                            match game.color_to_move() {
                                                cherris_core::Color::White => {
                                                    println!("Checkmate! Black wins!")
                                                }
                                                cherris_core::Color::Black => {
                                                    println!("Checkmate! White wins!")
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        dragged_starting_sqaure = None;
                        dragged_piece = None;
                    }
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        if let Some(piece) = dragged_piece {
            let piece_position = Point::new(
                mouse_position.x - (SQAURE_SIZE / 2) as i32,
                mouse_position.y - (SQAURE_SIZE / 2) as i32,
            );
            draw_piece(&mut canvas, &texture, piece, piece_position);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_possible_moves(
    canvas: &mut Canvas<Window>,
    moves: Vec<Move>,
    color_to_move: cherris_core::Color,
) {
    for chess_move in moves {
        match chess_move {
            Move::Standard { to, .. } => {
                let file_index = to.to_index() % 8;
                let rank_index = to.to_index() / 8;
                canvas.set_draw_color(Color::RED);
                canvas
                    .fill_rect(Rect::new(
                        (SQAURE_SIZE * file_index as u32).try_into().unwrap(),
                        (SQAURE_SIZE * (7 - rank_index) as u32).try_into().unwrap(),
                        SQAURE_SIZE,
                        SQAURE_SIZE,
                    ))
                    .unwrap();
            }
            Move::EnPassant { to, .. } => {
                let file_index = to.to_index() % 8;
                let rank_index = to.to_index() / 8;
                canvas.set_draw_color(Color::RED);
                canvas
                    .fill_rect(Rect::new(
                        (SQAURE_SIZE * file_index as u32).try_into().unwrap(),
                        (SQAURE_SIZE * (7 - rank_index) as u32).try_into().unwrap(),
                        SQAURE_SIZE,
                        SQAURE_SIZE,
                    ))
                    .unwrap();
            }
            Move::CastleShort => {
                let to = match color_to_move {
                    cherris_core::Color::White => Square::G1,
                    cherris_core::Color::Black => Square::G8,
                };

                let file_index = to.to_index() % 8;
                let rank_index = to.to_index() / 8;
                canvas.set_draw_color(Color::RED);
                canvas
                    .fill_rect(Rect::new(
                        (SQAURE_SIZE * file_index as u32).try_into().unwrap(),
                        (SQAURE_SIZE * (7 - rank_index) as u32).try_into().unwrap(),
                        SQAURE_SIZE,
                        SQAURE_SIZE,
                    ))
                    .unwrap();
            }
            Move::CastleLong => {
                let to = match color_to_move {
                    cherris_core::Color::White => Square::C1,
                    cherris_core::Color::Black => Square::C8,
                };

                let file_index = to.to_index() % 8;
                let rank_index = to.to_index() / 8;
                canvas.set_draw_color(Color::RED);
                canvas
                    .fill_rect(Rect::new(
                        (SQAURE_SIZE * file_index as u32).try_into().unwrap(),
                        (SQAURE_SIZE * (7 - rank_index) as u32).try_into().unwrap(),
                        SQAURE_SIZE,
                        SQAURE_SIZE,
                    ))
                    .unwrap();
            }
        }
    }
}

fn draw_position(
    canvas: &mut Canvas<Window>,
    piece_texture: &Texture,
    position: &Position,
    dragged_sqaure: Option<Square>,
) {
    for rank in Rank::iter() {
        for file in File::iter() {
            let rank_index = rank.to_index();
            let file_index = file.to_index();
            let square = Square::from((*file, *rank));
            if Some(square) == dragged_sqaure {
                continue;
            }

            if let Some(piece) = position.board.piece_on(square) {
                draw_piece(
                    canvas,
                    piece_texture,
                    piece,
                    Point::new(
                        (SQAURE_SIZE * file_index as u32).try_into().unwrap(),
                        (SQAURE_SIZE * (7 - rank_index) as u32).try_into().unwrap(),
                    ),
                );
            }
        }
    }
}

fn draw_piece(canvas: &mut Canvas<Window>, piece_texture: &Texture, piece: Piece, position: Point) {
    let role_index = match piece.role {
        Role::King => 0,
        Role::Queen => 1,
        Role::Bishop => 2,
        Role::Knight => 3,
        Role::Rook => 4,
        Role::Pawn => 5,
    };

    let color_index = match piece.color {
        cherris_core::Color::White => 0,
        cherris_core::Color::Black => 1,
    };

    canvas
        .copy(
            piece_texture,
            Rect::new(role_index * 335, color_index * 332, 333, 332),
            Rect::new(position.x, position.y, SQAURE_SIZE, SQAURE_SIZE),
        )
        .unwrap();
}

fn draw_board(canvas: &mut Canvas<Window>) {
    for rank in Rank::iter() {
        for file in File::iter() {
            let rank_index = rank.to_index();
            let file_index = file.to_index();
            let is_even = (file_index + rank_index) % 2 != 0;
            let square_color = match is_even {
                true => Color::RGB(240, 217, 181),
                false => Color::RGB(181, 136, 99),
            };

            canvas.set_draw_color(square_color);
            canvas
                .fill_rect(Rect::new(
                    (SQAURE_SIZE * file_index as u32).try_into().unwrap(),
                    (SQAURE_SIZE * (7 - rank_index) as u32).try_into().unwrap(),
                    SQAURE_SIZE,
                    SQAURE_SIZE,
                ))
                .unwrap();
        }
    }
}
