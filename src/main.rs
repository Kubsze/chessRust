mod types;
mod game;
mod ui;

use game::Board;
use ui::UI;

fn main() {
    println!("=== Terminal Chess ===");
    println!("Enter moves like: e2e4");
    println!("Enter 'q' to quit.\n");

    let mut board = Board::new();

    loop {
        UI::draw_board(&board);

        let mv = match UI::read_move() {
            Some(m) => m,
            None => {
                println!("\nExiting game.");
                break;
            }
        };

        if board.make_move(mv) {
            continue;
        } else {
            println!("Illegal move. Try again.\n");
        }
    }
}