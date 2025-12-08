use chess_rust::game::Board;
use chess_rust::types::{Move, Square, Color};

fn mv(f1: u8, r1: u8, f2: u8, r2: u8) -> Move {
    Move { from: Square(f1, r1), to: Square(f2, r2) }
}

#[test]
fn pawn_forward_one() {
    let mut b = Board::new();
    assert!(b.make_move(mv(4, 1, 4, 2)));
}

#[test]
fn pawn_forward_two() {
    let mut b = Board::new();
    assert!(b.make_move(mv(4, 1, 4, 3)));
}

#[test]
fn pawn_forward_blocked() {
    let mut b = Board::new();
    assert!(b.make_move(mv(4, 1, 4, 2)));
    assert!(!b.make_move(mv(4, 1, 4, 3)));
}

#[test]
fn pawn_illegal_backward() {
    let mut b = Board::new();
    assert!(!b.make_move(mv(4, 1, 4, 0)));
}

#[test]
fn knight_can_jump() {
    let mut b = Board::new();
    assert!(b.make_move(mv(1, 0, 2, 2)));
}

#[test]
fn knight_illegal_move() {
    let mut b = Board::new();
    assert!(!b.make_move(mv(1, 0, 1, 2)));
}

#[test]
fn bishop_blocked_path() {
    let mut b = Board::new();
    assert!(!b.make_move(mv(2, 0, 4, 2)));
}

#[test]
fn rook_blocked_move() {
    let mut b = Board::new();
    assert!(!b.make_move(mv(0, 0, 0, 5)));
}

#[test]
fn cannot_capture_own_piece() {
    let mut b = Board::new();
    assert!(!b.make_move(mv(0, 0, 1, 0)));
}

#[test]
fn turn_switches_after_legal_move() {
    let mut b = Board::new();
    assert!(b.make_move(mv(4, 1, 4, 2)));
    assert_eq!(b.turn, Color::Black);
}
