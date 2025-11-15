use crate::types::{Color, Move, Piece, PieceKind, Square};

pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    pub turn: Color,
}

impl Board {
    pub fn new() -> Self {
        Self {
            squares: Self::starting_position(),
            turn: Color::White,
        }
    }

    fn starting_position() -> [[Option<Piece>; 8]; 8] {
        let mut board = [[None; 8]; 8];

        use Color::*;
        use PieceKind::*;

        let back_rank = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];

        for (file, kind) in back_rank.iter().enumerate() {
            board[0][file] = Some(Piece {
                kind: *kind,
                color: White,
            });
            board[1][file] = Some(Piece {
                kind: Pawn,
                color: White,
            });
        }

        for (file, kind) in back_rank.iter().enumerate() {
            board[7][file] = Some(Piece {
                kind: *kind,
                color: Black,
            });
            board[6][file] = Some(Piece {
                kind: Pawn,
                color: Black,
            });
        }

        board
    }

    pub fn get(&self, sq: Square) -> Option<Piece> {
        if sq.0 > 7 || sq.1 > 7 {
            return None;
        }
        self.squares[sq.1 as usize][sq.0 as usize]
    }

    pub fn set(&mut self, sq: Square, piece: Option<Piece>) {
        if sq.0 <= 7 && sq.1 <= 7 {
            self.squares[sq.1 as usize][sq.0 as usize] = piece;
        }
    }

    fn apply_move(&mut self, mv: Move) {
        let piece = self.get(mv.from);
        self.set(mv.from, None);
        self.set(mv.to, piece);
    }

    pub fn make_move(&mut self, mv: Move) -> bool {
        if !self.is_legal_move(mv) {
            return false;
        }

        self.apply_move(mv);

        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        true
    }

    pub fn is_legal_move(&self, mv: Move) -> bool {
        if mv.from.0 > 7 || mv.from.1 > 7 || mv.to.0 > 7 || mv.to.1 > 7 {
            return false;
        }

        let piece = match self.get(mv.from) {
            Some(p) => p,
            None => return false,
        };

        if piece.color != self.turn {
            return false;
        }

        if let Some(dest) = self.get(mv.to) {
            if dest.color == piece.color {
                return false;
            }
        }

        match piece.kind {
            PieceKind::Pawn => self.legal_pawn_move(piece.color, mv),
            PieceKind::Knight => self.legal_knight_move(mv),
            PieceKind::Bishop => self.legal_bishop_move(mv),
            PieceKind::Rook => self.legal_rook_move(mv),
            PieceKind::Queen => self.legal_queen_move(mv),
            PieceKind::King => self.legal_king_move(mv),
        }
    }

    pub fn is_in_check() {
        todo!();
    }

    pub fn is_checkmate() {
        todo!();
    }

    pub fn is_stalemate() {
        todo!();
    }

    pub fn generate_legal_moves() {
        todo!();
    }

    pub fn handle_castling() {
        todo!();
    }

    pub fn handle_en_passant() {
        todo!();
    }

    pub fn handle_pawn_promotion() {
        todo!();
    }

    //helpers
    fn legal_pawn_move(&self, color: Color, mv: Move) -> bool {
        let dx = mv.to.0 as i8 - mv.from.0 as i8;
        let dy = mv.to.1 as i8 - mv.from.1 as i8;

        let forward = match color {
            Color::White => 1,
            Color::Black => -1,
        };

        if dx == 0 && dy == forward {
            return self.get(mv.to).is_none();
        }

        if dx == 0 && dy == forward * 2 {
            let start_rank = match color {
                Color::White => 1,
                Color::Black => 6,
            };
            if mv.from.1 == start_rank {
                let middle = Square(mv.from.0, (mv.from.1 as i8 + forward) as u8);
                return self.get(middle).is_none() && self.get(mv.to).is_none();
            }
        }

        if dy == forward && dx.abs() == 1 {
            return self.get(mv.to).is_some();
        }

        false
    }

    fn legal_knight_move(&self, mv: Move) -> bool {
        let dx = (mv.to.0 as i8 - mv.from.0 as i8).abs();
        let dy = (mv.to.1 as i8 - mv.from.1 as i8).abs();
        (dx == 1 && dy == 2) || (dx == 2 && dy == 1)
    }

    fn legal_bishop_move(&self, mv: Move) -> bool {
        let dx = (mv.to.0 as i8 - mv.from.0 as i8).abs();
        let dy = (mv.to.1 as i8 - mv.from.1 as i8).abs();

        if dx != dy {
            return false;
        }
        self.clear_diagonal(mv)
    }

    fn legal_rook_move(&self, mv: Move) -> bool {
        let dx = mv.to.0 as i8 - mv.from.0 as i8;
        let dy = mv.to.1 as i8 - mv.from.1 as i8;

        if dx != 0 && dy != 0 {
            return false;
        }
        self.clear_straight(mv)
    }

    fn legal_queen_move(&self, mv: Move) -> bool {
        let dx = (mv.to.0 as i8 - mv.from.0 as i8).abs();
        let dy = (mv.to.1 as i8 - mv.from.1 as i8).abs();
        if dx == dy {
            self.clear_diagonal(mv)
        } else if dx == 0 || dy == 0 {
            self.clear_straight(mv)
        } else {
            false
        }
    }

    fn legal_king_move(&self, mv: Move) -> bool {
        let dx = (mv.to.0 as i8 - mv.from.0 as i8).abs();
        let dy = (mv.to.1 as i8 - mv.from.1 as i8).abs();
        dx <= 1 && dy <= 1
    }

    fn clear_straight(&self, mv: Move) -> bool {
        let (x1, y1) = (mv.from.0 as i8, mv.from.1 as i8);
        let (x2, y2) = (mv.to.0 as i8, mv.to.1 as i8);

        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();

        let mut x = x1 + dx;
        let mut y = y1 + dy;

        while x != x2 || y != y2 {
            if self.get(Square(x as u8, y as u8)).is_some() {
                return false;
            }
            x += dx;
            y += dy;
        }

        true
    }

    fn clear_diagonal(&self, mv: Move) -> bool {
        let (x1, y1) = (mv.from.0 as i8, mv.from.1 as i8);
        let (x2, y2) = (mv.to.0 as i8, mv.to.1 as i8);

        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();

        let mut x = x1 + dx;
        let mut y = y1 + dy;

        while x != x2 && y != y2 {
            if self.get(Square(x as u8, y as u8)).is_some() {
                return false;
            }
            x += dx;
            y += dy;
        }

        true
    }
}
