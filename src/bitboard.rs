use crate::types::*;

pub type Bitboard = u64;

pub const EMPTY: Bitboard = 0;
pub const FULL: Bitboard = !0;

pub const FILE_A: Bitboard = 0x0101_0101_0101_0101;
pub const FILE_B: Bitboard = FILE_A << 1;
pub const FILE_G: Bitboard = FILE_A << 6;
pub const FILE_H: Bitboard = FILE_A << 7;

pub const RANK_1: Bitboard = 0xFF;
pub const RANK_2: Bitboard = RANK_1 << 8;
pub const RANK_3: Bitboard = RANK_1 << 16;
pub const RANK_4: Bitboard = RANK_1 << 24;
pub const RANK_5: Bitboard = RANK_1 << 32;
pub const RANK_6: Bitboard = RANK_1 << 40;
pub const RANK_7: Bitboard = RANK_1 << 48;
pub const RANK_8: Bitboard = RANK_1 << 56;

pub const NOT_FILE_A: Bitboard = !FILE_A;
pub const NOT_FILE_H: Bitboard = !FILE_H;
pub const NOT_FILE_AB: Bitboard = !(FILE_A | FILE_B);
pub const NOT_FILE_GH: Bitboard = !(FILE_G | FILE_H);

pub const fn bb(sq: Square) -> Bitboard {
    1u64 << sq
}

pub const fn count(b: Bitboard) -> u32 {
    b.count_ones()
}

pub const fn is_empty(b: Bitboard) -> bool {
    b == 0
}

pub const fn has(b: Bitboard, sq: Square) -> bool {
    (b >> sq) & 1 != 0
}

pub const fn lsb(b: Bitboard) -> Square {
    b.trailing_zeros() as Square
}

pub fn pop_lsb(b: &mut Bitboard) -> Square {
    let sq = lsb(*b);
    *b &= *b - 1;
    sq
}

pub const fn file_mask(sq: Square) -> Bitboard {
    FILE_A << file_of(sq)
}

pub const fn rank_mask(sq: Square) -> Bitboard {
    RANK_1 << (rank_of(sq) * 8)
}

pub const fn north(b: Bitboard) -> Bitboard {
    b << 8
}

pub const fn south(b: Bitboard) -> Bitboard {
    b >> 8
}

pub const fn east(b: Bitboard) -> Bitboard {
    (b << 1) & NOT_FILE_A
}

pub const fn west(b: Bitboard) -> Bitboard {
    (b >> 1) & NOT_FILE_H
}

pub const fn north_east(b: Bitboard) -> Bitboard {
    (b << 9) & NOT_FILE_A
}

pub const fn north_west(b: Bitboard) -> Bitboard {
    (b << 7) & NOT_FILE_H
}

pub const fn south_east(b: Bitboard) -> Bitboard {
    (b >> 7) & NOT_FILE_A
}

pub const fn south_west(b: Bitboard) -> Bitboard {
    (b >> 9) & NOT_FILE_H
}

pub fn print_bb(b: Bitboard) {
    for rank in (0..8).rev() {
        let mut line = String::new();
        for file in 0..8 {
            line.push(if has(b, make_square(file, rank)) { '1' } else { '.' });
            line.push(' ');
        }
        println!("{} {}", rank + 1, line);
    }
    println!("  a b c d e f g h");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masks() {
        assert_eq!(count(FILE_A), 8);
        assert_eq!(count(RANK_4), 8);
        assert!(has(FILE_H, 63) && has(FILE_H, 7) && !has(FILE_H, 6));
        assert_eq!(file_mask(28), FILE_A << 4); // e4 is on the e-file
        assert_eq!(rank_mask(28), RANK_4);
    }

    #[test]
    fn bit_ops() {
        let mut b = bb(3) | bb(10) | bb(63);
        assert_eq!(count(b), 3);
        assert_eq!(pop_lsb(&mut b), 3);
        assert_eq!(pop_lsb(&mut b), 10);
        assert_eq!(pop_lsb(&mut b), 63);
        assert!(is_empty(b));
    }

    #[test]
    fn shifts_do_not_wrap() {
        assert_eq!(east(bb(7)), EMPTY); 
        assert_eq!(west(bb(0)), EMPTY); 
        assert_eq!(east(bb(0)), bb(1));
        assert_eq!(north(bb(0)), bb(8));
        assert_eq!(north_east(bb(7)), EMPTY);
        assert_eq!(north_west(bb(0)), EMPTY);
        assert_eq!(south_east(bb(56)), bb(49));
        assert_eq!(south_west(bb(56)), EMPTY);
    }
}