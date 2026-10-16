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

const ROOK_DIRS: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const BISHOP_DIRS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

fn sliding_attacks(sq: Square, occ: Bitboard, dirs: &[(i8, i8)]) -> Bitboard {
    let mut attacks = EMPTY;
    for &(df, dr) in dirs {
        let mut f = file_of(sq) as i8 + df;
        let mut r = rank_of(sq) as i8 + dr;
        while (0..8).contains(&f) && (0..8).contains(&r) {
            let s = bb(make_square(f as u8, r as u8));
            attacks |= s;
            if occ & s != 0 {
                break;
            }
            f += df;
            r += dr;
        }
    }
    attacks
}

pub fn rook_attacks_slow(sq: Square, occ: Bitboard) -> Bitboard {
    sliding_attacks(sq, occ, &ROOK_DIRS)
}

pub fn bishop_attacks_slow(sq: Square, occ: Bitboard) -> Bitboard {
    sliding_attacks(sq, occ, &BISHOP_DIRS)
}

fn relevant_mask(sq: Square, dirs: &[(i8, i8)]) -> Bitboard {
    let mut mask = EMPTY;
    for &(df, dr) in dirs {
        let mut f = file_of(sq) as i8 + df;
        let mut r = rank_of(sq) as i8 + dr;
        while (0..8).contains(&(f + df)) && (0..8).contains(&(r + dr)) {
            mask |= bb(make_square(f as u8, r as u8));
            f += df;
            r += dr;
        }
    }
    mask
}

pub fn rook_mask(sq: Square) -> Bitboard {
    relevant_mask(sq, &ROOK_DIRS)
}

pub fn bishop_mask(sq: Square) -> Bitboard {
    relevant_mask(sq, &BISHOP_DIRS)
}

pub fn subsets(mask: Bitboard) -> Vec<Bitboard> {
    let mut all = Vec::with_capacity(1usize << count(mask));
    let mut sub: Bitboard = 0;
    loop {
        all.push(sub);
        sub = sub.wrapping_sub(mask) & mask;
        if sub == 0 {
            break;
        }
    }
    all
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

        #[test]
    fn slow_slider_attacks() {
        let occ = bb(44) | bb(26);
        let a = rook_attacks_slow(28, occ);
        assert!(has(a, 44) && !has(a, 52)); 
        assert!(has(a, 26) && !has(a, 25)); 
        assert_eq!(count(a), 10);
        assert_eq!(count(rook_attacks_slow(0, EMPTY)), 14); 
        let b = bishop_attacks_slow(2, bb(20));
        assert_eq!(b, bb(11) | bb(20) | bb(9) | bb(16));
    }

    #[test]
    fn mask_sizes() {
        assert_eq!(count(rook_mask(0)), 12); 
        assert_eq!(count(rook_mask(28)), 10); 
        assert_eq!(count(bishop_mask(0)), 6); 
        assert_eq!(count(bishop_mask(28)), 9); 
        assert_eq!(count(bishop_mask(27)), 9); 
        assert!(!has(rook_mask(28), 60) && !has(rook_mask(28), 4)); 
    }

    #[test]
    fn subset_enumeration() {
        let mask = bb(1) | bb(9) | bb(40);
        let subs = subsets(mask);
        assert_eq!(subs.len(), 8);
        assert!(subs.iter().all(|&s| s & !mask == 0));
        let mut sorted = subs.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), 8); 
        assert_eq!(subsets(EMPTY), vec![EMPTY]);
    }
}