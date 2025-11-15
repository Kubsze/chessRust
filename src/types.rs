#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Square(pub u8, pub u8);
// 0–7 for file/rank

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}
