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

    pub const fn from_idx(i: usize) -> Piece {
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

pub const CASTLE_WK: u8 = 1;
pub const CASTLE_WQ: u8 = 2;
pub const CASTLE_BK: u8 = 4;
pub const CASTLE_BQ: u8 = 8;

pub const FLAG_QUIET: u8 = 0;
pub const FLAG_DOUBLE_PUSH: u8 = 1;
pub const FLAG_KING_CASTLE: u8 = 2;
pub const FLAG_QUEEN_CASTLE: u8 = 3;
pub const FLAG_CAPTURE: u8 = 4;
pub const FLAG_EN_PASSANT: u8 = 5;
pub const FLAG_PROMO_N: u8 = 8;
pub const FLAG_PROMO_B: u8 = 9;
pub const FLAG_PROMO_R: u8 = 10;
pub const FLAG_PROMO_Q: u8 = 11;
pub const FLAG_PROMO_CAP_N: u8 = 12;
pub const FLAG_PROMO_CAP_B: u8 = 13;
pub const FLAG_PROMO_CAP_R: u8 = 14;
pub const FLAG_PROMO_CAP_Q: u8 = 15;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]

pub struct Move(pub u16);

impl Move {
    pub const NULL: Move = Move(0);

    pub const fn new(from: Square, to: Square, flag: u8) -> Move {
        Move((from as u16) | ((to as u16) << 6) | ((flag as u16) << 12))
    }

    pub const fn from(self) -> Square {
        (self.0 & 63) as Square
    }

    pub const fn to(self) -> Square {
        ((self.0 >> 6) & 63) as Square
    }

    pub const fn flag(self) -> u8 {
        (self.0 >> 12) as u8
    }

    pub const fn is_capture(self) -> bool {
        self.flag() & 4 != 0
    }

    pub const fn is_promotion(self) -> bool {
        self.flag() & 8 != 0
    }

    pub const fn is_en_passant(self) -> bool {
        self.flag() == FLAG_EN_PASSANT
    }

    pub const fn is_double_push(self) -> bool {
        self.flag() == FLAG_DOUBLE_PUSH
    }

    pub const fn is_castle(self) -> bool {
        self.flag() == FLAG_KING_CASTLE || self.flag() == FLAG_QUEEN_CASTLE
    }

    pub const fn promo_piece(self) -> Piece {
        Piece::from_idx(1 + (self.flag() & 3) as usize)
    }

    pub fn to_uci(self) -> String {
        let mut s = format!("{}{}", square_to_string(self.from()), square_to_string(self.to()));
        if self.is_promotion() {
            s.push(match self.promo_piece() {
                Piece::Knight => 'n',
                Piece::Bishop => 'b',
                Piece::Rook => 'r',
                _ => 'q',
            });
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_layout() {
        assert_eq!(make_square(0, 0), 0);
        assert_eq!(make_square(7, 0), 7);
        assert_eq!(make_square(0, 7), 56);
        assert_eq!(make_square(7, 7), 63);
        assert_eq!(square_to_string(28), "e4");
        assert_eq!(square_from_string("e4"), Some(28));
        assert_eq!(square_from_string("i9"), None);
    }

    #[test]
    fn piece_codes() {
        let code = make_piece(Color::Black, Piece::Queen);
        assert_eq!(code, 10);
        assert_eq!(piece_of(code), Piece::Queen);
        assert_eq!(color_of(code), Color::Black);
        assert_eq!(piece_to_char(code), 'q');
        assert_eq!(piece_from_char('K'), Some(5));
        assert_eq!(piece_from_char('x'), None);
    }

    #[test]
    fn move_packing() {
        let m = Move::new(12, 28, FLAG_DOUBLE_PUSH);
        assert_eq!((m.from(), m.to(), m.flag()), (12, 28, FLAG_DOUBLE_PUSH));
        assert!(m.is_double_push() && !m.is_capture());
        assert_eq!(m.to_uci(), "e2e4");

        let p = Move::new(52, 60, FLAG_PROMO_CAP_Q); 
        assert!(p.is_capture() && p.is_promotion());
        assert_eq!(p.promo_piece(), Piece::Queen);
        assert_eq!(p.to_uci(), "e7e8q");

        assert!(Move::new(4, 6, FLAG_KING_CASTLE).is_castle());
        assert!(Move::new(36, 43, FLAG_EN_PASSANT).is_capture());
    }
}