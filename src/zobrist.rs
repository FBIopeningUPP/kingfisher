use crate::types::*;
use std::sync::OnceLock;

pub struct Rng(u64);

impl Rng {
    pub const fn new(seed: u64) -> Rng {
        Rng(seed)
    }

    pub fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

pub struct Zobrist {
    pub piece: [[u64; 64]; 12], 
    pub side: u64,              
    pub castling: [u64; 16],    
    pub ep_file: [u64; 8],  
}

static ZOBRIST: OnceLock<Zobrist> = OnceLock::new();

pub fn keys() -> &'static Zobrist {
    ZOBRIST.get_or_init(|| {
        let mut rng = Rng::new(0x9E37_79B9_7F4A_7C15);
        let mut z = Zobrist {
            piece: [[0; 64]; 12],
            side: 0,
            castling: [0; 16],
            ep_file: [0; 8],
        };
        for p in 0..12 {
            for sq in 0..64 {
                z.piece[p][sq] = rng.next();
            }
        }
        z.side = rng.next();
        for c in 0..16 {
            z.castling[c] = rng.next();
        }
        for f in 0..8 {
            z.ep_file[f] = rng.next();
        }
        z
    })
}

pub fn piece_key(code: PieceCode, sq: Square) -> u64 {
    keys().piece[code as usize][sq as usize]
}

pub fn side_key() -> u64 {
    keys().side
}

pub fn castling_key(rights: u8) -> u64 {
    keys().castling[rights as usize]
}

pub fn ep_key(ep: Square) -> u64 {
    if ep == NO_SQUARE { 0 } else { keys().ep_file[file_of(ep) as usize]}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rng_is_deterministic() {
        let mut a = Rng::new(12345);
        let mut b = Rng::new(12345);
        for _ in 0..100 {
            assert_eq!(a.next(), b.next());
        }
    }

    #[test]
    fn keys_are_nonzero_and_distinct() {
        let z = keys();
        assert_ne!(z.side, 0);
        assert_ne!(piece_key(0, 0), piece_key(0, 1));
        assert_ne!(piece_key(0, 0), piece_key(1, 0));
        assert_ne!(castling_key(1), castling_key(2));
    }

    #[test]
    fn no_duplicate_piece_keys() {
        let z = keys();
        let mut all: Vec<u64> = z.piece.iter().flatten().copied().collect();
        all.sort();
        let len = all.len();
        all.dedup();
        assert_eq!(all.len(), len);
    }

    #[test]
    fn ep_key_handles_none() {
        assert_eq!(ep_key(NO_SQUARE), 0);
        assert_ne!(ep_key(make_square(4, 2)), 0); // e3
        assert_eq!(ep_key(make_square(4, 2)), ep_key(make_square(4, 5))); // same f
    }
}