mod game;
mod types;
mod ui;
mod engine;

use game::Board;
use types::{Color, Move};
use ui::UI;
use engine::Engine;

fn main() {
    println!("=== Terminal Chess ===");
    println!("1. Human vs Human");
    println!("2. Human vs Stockfish");
    println!("Choose mode: ");

    let mode = UI::read_line().trim().to_string();
    let vs_engine = mode == "2";

    let mut engine = if vs_engine { Some(Engine::new()) } else { None };

    let mut board = Board::new();
    let mut move_list = String::new();

    loop {
        UI::draw_board(&board);

        if board.is_checkmate(board.turn) {
            println!("Checkmate! {:?} loses.", board.turn);
            println!("{:?} wins!", opposite(board.turn));
            break;
        }

        if board.is_stalemate(board.turn) {
            println!("Stalemate! It's a draw.");
            break;
        }

        if board.is_in_check(board.turn) {
            println!("{:?} is in check.", board.turn);
        }

        let mv = if vs_engine && board.turn == Color::Black {
            let engine_move = engine.as_mut().unwrap().bestMove(&move_list)
                .expect("Stockfish did not return move");

            let parsed = UI::parseMove(&engine_move)
                .expect("Engine returned invalid move");

            println!("Stockfish plays: {}", engine_move);
            parsed
        } else {
            match UI::readMove() {
                Some(m) => m,
                None => {
                    println!("Exiting game.");
                    break;
                }
            }
        };

        if board.make_move(mv) {
            move_list.push_str(&UI::move_to_string(mv));
            move_list.push(' ');
        } else {
            println!("Illegal move.\n");
            continue;
        }
    }
}

fn opposite(c: Color) -> Color {
    match c {
        Color::White => Color::Black,
        Color::Black => Color::White,
    }
}
