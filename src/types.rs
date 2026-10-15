#[derive(Clone, Copy, PartialEq, Eq, Debug)]

pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    pub const fn idx(self) -> usize {
        self as usize
    }

    pub const fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl Piece {
    pub const ALL: [Piece; 6] = [
        Piece::Pawn,
        Piece::Knight,
        Piece::Bishop,
        Piece::Rook,
        Piece::Queen,
        Piece::King,
    ];
    
    pub const fn idx(self) -> usize {
        self as usize
    }

    pub const fn from_idex(i: usize) -> Piece {
        match i {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            _ => panic!("bad piece index"),
        }
    }
}

pub type PieceCode = u8;
pub const NO_PIECE: PieceCode = 12;

pub const fn make_piece(c: Color, p: Piece) -> PieceCode {
    (c as u8) * 6 + p as u8
}

pub const fn piece_of(code: PieceCode) -> Piece {
    Piece::from_idx((code % 6) as usize)
}

pub const fn color_of(code: PieceCode) -> Color {
    if code < 6 {Color::White} else {Color::Black}
}

const PIECE_CHARS: &[u8; 12] = b"PNBRQKpnbrqk";

pub fn piece_to_char(code: PieceCode) -> char {
    if code == NO_PIECE {'.'} else {PIECE_CHARS[code as usize] as char}
}

pub fn piece_from_char(ch: char) -> Option<PieceCode> {
    PIECE_CHARS.iter().position(|&b| b as char == ch).map(|i| i as PieceCode)
}

pub type Square = u8;
pub const NO_SQUARE: Square = 64;

pub const fn make_square(file: u8, rank: u8) -> Square {
    rank * 8 + file
}

pub const fn file_of(sq: Square) -> u8 {
    sq & 7
}

pub const fn rank_of(sq: Square) -> u8 {
    sq >> 3
}

pub fn square_to_string(sq: Square) -> String {
    format!("{}{}", (b'a' + file_of(sq)) as char, (b'1' + rank_of(sq)) as char)
}

pub fn square_from_string(s: &str) -> Option<Square> {
    let b = s.as_bytes();
    if b.len() != 2 {
        return None;
    }
    let file = b[0].wrapping_sub(b'a');
    let rank = b[1].wrapping_sub(b'1');
    if file < 8 && rank < 8 { Some(make_square(file, rank)) } else { None }
}