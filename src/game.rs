use crate::types::{Color, Move, Piece, PieceKind, Square};

#[derive(Clone)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    pub turn: Color,

    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,

    pub en_passant_target: Option<Square>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            squares: Self::starting_position(),
            turn: Color::White,
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
            en_passant_target: None,
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
        let piece_opt = self.get(mv.from);
        if piece_opt.is_none() {
            return;
        }
        let piece = piece_opt.unwrap();

        self.en_passant_target = None;

        if piece.kind == PieceKind::King && (mv.from.1 == mv.to.1) {
            let dx = mv.to.0 as i8 - mv.from.0 as i8;
            if dx.abs() == 2 {
                self.set(mv.from, None);
                self.set(mv.to, Some(piece));

                match piece.color {
                    Color::White => {
                        self.white_kingside = false;
                        self.white_queenside = false;
                        if dx > 0 {
                            let rook_from = Square(7, 0);
                            let rook_to = Square(5, 0);
                            let rook = self.get(rook_from);
                            self.set(rook_from, None);
                            self.set(rook_to, rook);
                        } else {
                            let rook_from = Square(0, 0);
                            let rook_to = Square(3, 0);
                            let rook = self.get(rook_from);
                            self.set(rook_from, None);
                            self.set(rook_to, rook);
                        }
                    }
                    Color::Black => {
                        self.black_kingside = false;
                        self.black_queenside = false;
                        if dx > 0 {
                            let rook_from = Square(7, 7);
                            let rook_to = Square(5, 7);
                            let rook = self.get(rook_from);
                            self.set(rook_from, None);
                            self.set(rook_to, rook);
                        } else {
                            let rook_from = Square(0, 7);
                            let rook_to = Square(3, 7);
                            let rook = self.get(rook_from);
                            self.set(rook_from, None);
                            self.set(rook_to, rook);
                        }
                    }
                }
                return;
            }
        }

        if piece.kind == PieceKind::Pawn {
            let dx = mv.to.0 as i8 - mv.from.0 as i8;
            let dy = mv.to.1 as i8 - mv.from.1 as i8;
            let forward = match piece.color {
                Color::White => 1,
                Color::Black => -1,
            };

            if dx.abs() == 1 && dy == forward {
                if let Some(ep) = self.en_passant_target {
                    if ep.0 == mv.to.0 && ep.1 == mv.to.1 {
                        let captured_pawn_sq = Square(mv.to.0, mv.from.1);
                        self.set(captured_pawn_sq, None);
                    }
                }
            }

            let last_rank = match piece.color {
                Color::White => 7,
                Color::Black => 0,
            };
            if mv.to.1 == last_rank {
                self.set(mv.from, None);
                self.set(
                    mv.to,
                    Some(Piece {
                        kind: PieceKind::Queen,
                        color: piece.color,
                    }),
                );

                return;
            }

            if dx == 0 {
                if (mv.from.1 as i8 - mv.to.1 as i8).abs() == 2 {
                    let ep_rank = ((mv.from.1 as i8 + mv.to.1 as i8) / 2) as u8;
                    self.en_passant_target = Some(Square(mv.from.0, ep_rank));
                }
            }
        }

        if piece.kind == PieceKind::King {
            match piece.color {
                Color::White => {
                    self.white_kingside = false;
                    self.white_queenside = false;
                }
                Color::Black => {
                    self.black_kingside = false;
                    self.black_queenside = false;
                }
            }
        }

        if piece.kind == PieceKind::Rook {
            match piece.color {
                Color::White => {
                    if mv.from == Square(0, 0) {
                        self.white_queenside = false;
                    }
                    if mv.from == Square(7, 0) {
                        self.white_kingside = false;
                    }
                }
                Color::Black => {
                    if mv.from == Square(0, 7) {
                        self.black_queenside = false;
                    }
                    if mv.from == Square(7, 7) {
                        self.black_kingside = false;
                    }
                }
            }
        }

        if let Some(captured) = self.get(mv.to) {
            if captured.kind == PieceKind::Rook {
                match captured.color {
                    Color::White => {
                        if mv.to == Square(0, 0) {
                            self.white_queenside = false;
                        }
                        if mv.to == Square(7, 0) {
                            self.white_kingside = false;
                        }
                    }
                    Color::Black => {
                        if mv.to == Square(0, 7) {
                            self.black_queenside = false;
                        }
                        if mv.to == Square(7, 7) {
                            self.black_kingside = false;
                        }
                    }
                }
            }
        }

        self.set(mv.from, None);
        self.set(mv.to, Some(piece));
    }

    pub fn make_move(&mut self, mv: Move) -> bool {
        if !self.is_legal_move(mv, self.turn) {
            return false;
        }

        if self.would_cause_self_check(mv, self.turn) {
            return false;
        }

        self.apply_move(mv);

        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        true
    }

    pub fn is_legal_move(&self, mv: Move, color: Color) -> bool {
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

    pub fn is_in_check(&self, color: Color) -> bool {
        let king_sq = self.find_king(color);
        if king_sq.is_none() {
            return false;
        }
        let king_sq = king_sq.unwrap();
        self.is_square_attacked(king_sq, opposite_color(color))
    }

    pub fn is_checkmate(&self, color: Color) -> bool {
        if !self.is_in_check(color) {
            return false;
        }
        let moves = self.generate_legal_moves(color);
        moves.is_empty()
    }

    pub fn is_stalemate(&self, color: Color) -> bool {
        if self.is_in_check(color) {
            return false;
        }
        let moves = self.generate_legal_moves(color);
        moves.is_empty()
    }

    pub fn generate_legal_moves(&self, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();

        for y in 0..8 {
            for x in 0..8 {
                let from = Square(x as u8, y as u8);
                if let Some(p) = self.get(from) {
                    if p.color != color {
                        continue;
                    }

                    for ty in 0..8 {
                        for tx in 0..8 {
                            let to = Square(tx as u8, ty as u8);
                            let mv = Move { from, to };

                            if !self.is_legal_move(mv, color) {
                                continue;
                            }
                            if self.would_cause_self_check(mv, color) {
                                continue;
                            }

                            moves.push(mv);
                        }
                    }
                }
            }
        }

        moves
    }

    pub fn is_square_attacked(&self, sq: Square, by_color: Color) -> bool {
    for y in 0..8 {
        for x in 0..8 {
            let from = Square(x as u8, y as u8);
            if let Some(p) = self.get(from) {
                if p.color != by_color { continue; }

                let mv = Move { from, to: sq };

                match p.kind {
                    PieceKind::Pawn => {
                        let dx = sq.0 as i8 - from.0 as i8;
                        let dy = sq.1 as i8 - from.1 as i8;
                        let forward = if by_color == Color::White { 1 } else { -1 };
                        if dy == forward && dx.abs() == 1 { return true; }
                    },
                    PieceKind::Knight => {
                        if self.legal_knight_move(mv) { return true; }
                    },
                    PieceKind::Bishop => {
                        if (sq.0 as i8 - from.0 as i8).abs() == (sq.1 as i8 - from.1 as i8).abs() {
                            if self.clear_diagonal(mv) { return true; }
                        }
                    },
                    PieceKind::Rook => {
                        if sq.0 == from.0 || sq.1 == from.1 {
                            if self.clear_straight(mv) { return true; }
                        }
                    },
                    PieceKind::Queen => {
                        if (sq.0 == from.0 || sq.1 == from.1 && self.clear_straight(mv))
                            || ((sq.0 as i8 - from.0 as i8).abs() == (sq.1 as i8 - from.1 as i8).abs() && self.clear_diagonal(mv))
                        {
                            return true;
                        }
                    },
                    PieceKind::King => {
                        let dx = (sq.0 as i8 - from.0 as i8).abs();
                        let dy = (sq.1 as i8 - from.1 as i8).abs();
                        if dx <= 1 && dy <= 1 { return true; }
                    }
                }
            }
        }
    }
    false
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
            if self.get(mv.to).is_some() {
                return true;
            }

            if let Some(ep) = self.en_passant_target {
                if mv.to.0 == ep.0 && mv.to.1 == ep.1 {
                    return true;
                }
            }
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
        if dx <= 1 && dy <= 1 {
            true
        } else if dx == 2 && dy == 0 {
            let can = match self.get(mv.from) {
                Some(p) if p.kind == PieceKind::King => {
                    if p.color == Color::White {
                        self.can_castle_kingside(Color::White) && mv.to.0 == 6
                            || self.can_castle_queenside(Color::White) && mv.to.0 == 2
                    } else {
                        self.can_castle_kingside(Color::Black) && mv.to.0 == 6
                            || self.can_castle_queenside(Color::Black) && mv.to.0 == 2
                    }
                }
                _ => false,
            };
            can
        } else {
            false
        }
    }

    fn clear_straight(&self, mv: Move) -> bool {
    let (x1, y1) = (mv.from.0 as i8, mv.from.1 as i8);
    let (x2, y2) = (mv.to.0 as i8, mv.to.1 as i8);

    let dx = (x2 - x1).signum();
    let dy = (y2 - y1).signum();

    let mut x = x1 + dx;
    let mut y = y1 + dy;

    loop {
        if x == x2 && y == y2 {
            break;
        }

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

    loop {
        if x == x2 && y == y2 {
            break;
        }

        if self.get(Square(x as u8, y as u8)).is_some() {
            return false;
        }

        x += dx;
        y += dy;
    }

    true
}


    fn find_king(&self, color: Color) -> Option<Square> {
        for y in 0..8 {
            for x in 0..8 {
                if let Some(p) = self.squares[y][x] {
                    if p.kind == PieceKind::King && p.color == color {
                        return Some(Square(x as u8, y as u8));
                    }
                }
            }
        }
        None
    }

    fn would_cause_self_check(&self, mv: Move, color: Color) -> bool {
        let mut cloned = self.clone();
        cloned.apply_move(mv);
        cloned.is_in_check(color)
    }

    fn can_castle_kingside(&self, color: Color) -> bool {
        match color {
            Color::White => {
                if !self.white_kingside {
                    return false;
                }
                if self.get(Square(5, 0)).is_some() {
                    return false;
                }
                if self.get(Square(6, 0)).is_some() {
                    return false;
                }
                if let Some(r) = self.get(Square(7, 0)) {
                    if r.kind != PieceKind::Rook || r.color != Color::White {
                        return false;
                    }
                } else {
                    return false;
                }
                true
            }
            Color::Black => {
                if !self.black_kingside {
                    return false;
                }
                if self.get(Square(5, 7)).is_some() {
                    return false;
                }
                if self.get(Square(6, 7)).is_some() {
                    return false;
                }
                if let Some(r) = self.get(Square(7, 7)) {
                    if r.kind != PieceKind::Rook || r.color != Color::Black {
                        return false;
                    }
                } else {
                    return false;
                }
                true
            }
        }
    }

    fn can_castle_queenside(&self, color: Color) -> bool {
        match color {
            Color::White => {
                if !self.white_queenside {
                    return false;
                }
                // squares b1(1,0), c1(2,0), d1(3,0) must be empty (king passes through d1)
                if self.get(Square(1, 0)).is_some() {
                    return false;
                }
                if self.get(Square(2, 0)).is_some() {
                    return false;
                }
                if self.get(Square(3, 0)).is_some() {
                    return false;
                }
                if let Some(r) = self.get(Square(0, 0)) {
                    if r.kind != PieceKind::Rook || r.color != Color::White {
                        return false;
                    }
                } else {
                    return false;
                }
                true
            }
            Color::Black => {
                if !self.black_queenside {
                    return false;
                }
                if self.get(Square(1, 7)).is_some() {
                    return false;
                }
                if self.get(Square(2, 7)).is_some() {
                    return false;
                }
                if self.get(Square(3, 7)).is_some() {
                    return false;
                }
                if let Some(r) = self.get(Square(0, 7)) {
                    if r.kind != PieceKind::Rook || r.color != Color::Black {
                        return false;
                    }
                } else {
                    return false;
                }
                true
            }
        }
    }

    pub fn moveHistoryString(&self) -> String {
        String::new()
    }
}

fn opposite_color(c: Color) -> Color {
    match c {
        Color::White => Color::Black,
        Color::Black => Color::White,
    }
}
