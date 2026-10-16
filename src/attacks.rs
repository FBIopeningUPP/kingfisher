use crate::bitboard::*;
use crate::types::*;
use std::sync::OnceLock;

pub struct LeaperTables {
    pub knight: [Bitboard; 64],
    pub king: [Bitboard; 64],
    pub pawn: [[Bitboard; 64]; 2],
}

fn knight_from(b: Bitboard) -> Bitboard {
    ((b << 17) & NOT_FILE_A)
        | ((b << 15) & NOT_FILE_H)
        | ((b << 10) & NOT_FILE_AB)
        | ((b << 6) & NOT_FILE_GH)
        | ((b >> 17) & NOT_FILE_H)
        | ((b >> 15) & NOT_FILE_A)
        | ((b >> 10) & NOT_FILE_GH)
        | ((b >> 6) & NOT_FILE_AB) 
}

fn king_from(b: Bitboard) -> Bitboard {
    north(b) | south(b) | east(b) | west(b)
        | north_east(b) | north_west(b) | south_east(b) | south_west(b)
}

static LEAPERS: OnceLock<LeaperTables> = OnceLock::new();

fn leapers() -> &'static LeaperTables {
    LEAPERS.get_or_init(|| {
        let mut t = LeaperTables {
            knight: [EMPTY; 64],
            king: [EMPTY; 64],
            pawn: [[EMPTY; 64]; 2],
        };
        for sq in 0..64 {
            let b = bb(sq as Square);
            t.knight[sq] = knight_from(b);
            t.king[sq] = king_from(b);
            t.pawn[Color::White.idx()][sq] = north_east(b) | north_west(b);
            t.pawn[Color::Black.idx()][sq] = south_east(b) | south_west(b);
        }
        t
    })
}
pub fn knight_attacks(sq: Square) -> Bitboard {
    leapers().knight[sq as usize]
}

pub fn king_attacks(sq: Square) -> Bitboard {
    leapers().king[sq as usize]
}

pub fn pawn_attacks(color: Color, sq: Square) -> Bitboard {
    leapers().pawn[color.idx()][sq as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knight_table() {
        assert_eq!(knight_attacks(0), bb(17) | bb(10));
        assert_eq!(count(knight_attacks(28)), 8);
        assert_eq!(count(knight_attacks(7)), 2); 
        assert_eq!(count(knight_attacks(63)), 2);
    }

    #[test]
    fn king_table() {
        assert_eq!(king_attacks(0), bb(1) | bb(8) | bb(9));
        assert_eq!(count(king_attacks(28)), 8);
        assert_eq!(count(king_attacks(63)), 3);
        assert_eq!(count(king_attacks(7)), 3); 
    }

    #[test]
    fn pawn_table() {
        assert_eq!(pawn_attacks(Color::White, 12), bb(19) | bb(21));
        assert_eq!(pawn_attacks(Color::White, 8), bb(17));
        assert_eq!(pawn_attacks(Color::White, 15), bb(22));
        assert_eq!(pawn_attacks(Color::Black, 52), bb(43) | bb(45));
        assert_eq!(pawn_attacks(Color::White, 60), EMPTY);
    }
}