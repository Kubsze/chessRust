use crate::game::Board;
use crate::types::{Color, Move, Piece, PieceKind, Square};
use std::io::{self, Write};

pub struct UI;

impl UI {
    pub fn draw_board(board: &Board) {
        println!("\n    a   b   c   d   e   f   g   h");
        println!("  +---+---+---+---+---+---+---+---+");

        for rank in (0..8).rev() {
            print!("{} |", rank + 1);

            for file in 0..8 {
                let sq = Square(file, rank);
                match board.get(sq) {
                    Some(piece) => {
                        print!(" {} |", Self::piece_char(piece));
                    }
                    None => {
                        print!("   |");
                    }
                }
            }

            println!(" {}", rank + 1);
            println!("  +---+---+---+---+---+---+---+---+");
        }

        println!("    a   b   c   d   e   f   g   h\n");
        println!("Turn: {:?}\n", board.turn);
    }

    pub fn read_move() -> Option<Move> {
        print!("Enter move (e.g. e2e4, or 'q' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return None;
        }

        let input = input.trim().to_lowercase();

        if input == "q" || input == "quit" {
            return None;
        }

        Self::parse_move(&input)
    }

    pub fn parse_move(text: &str) -> Option<Move> {
        if text.len() != 4 {
            return None;
        }

        let bytes = text.as_bytes();

        let f1 = Self::file_to_index(bytes[0] as char)?;
        let r1 = Self::rank_to_index(bytes[1] as char)?;
        let f2 = Self::file_to_index(bytes[2] as char)?;
        let r2 = Self::rank_to_index(bytes[3] as char)?;

        Some(Move {
            from: Square(f1, r1),
            to: Square(f2, r2),
        })
    }

    fn file_to_index(c: char) -> Option<u8> {
        match c {
            'a'..='h' => Some((c as u8) - b'a'),
            _ => None,
        }
    }

    fn rank_to_index(c: char) -> Option<u8> {
        match c {
            '1'..='8' => Some((c as u8) - b'1'),
            _ => None,
        }
    }

    fn piece_char(piece: Piece) -> char {
        match (piece.color, piece.kind) {
            (Color::White, PieceKind::Pawn) => '♟',
            (Color::White, PieceKind::Rook) => '♜',
            (Color::White, PieceKind::Knight) => '♞',
            (Color::White, PieceKind::Bishop) => '♝',
            (Color::White, PieceKind::Queen) => '♛',
            (Color::White, PieceKind::King) => '♚',

            (Color::Black, PieceKind::Pawn) => '♙',
            (Color::Black, PieceKind::Rook) => '♖',
            (Color::Black, PieceKind::Knight) => '♘',
            (Color::Black, PieceKind::Bishop) => '♗',
            (Color::Black, PieceKind::Queen) => '♕',
            (Color::Black, PieceKind::King) => '♔',
        }
    }
}
