use crate::bitboard::Bitboard;
use crate::direction::Direction;
use crate::square::Square;

static mut BETWEEN_TABLE: [[Option<Bitboard>; Square::N]; Square::N] =
    [[None; Square::N]; Square::N];

pub fn between(s1: Square, s2: Square) -> Bitboard {
    unsafe { BETWEEN_TABLE[s1.array_idx()][s2.array_idx()].unwrap() }
}

// TODO: Tidy this up with more utility methods in Rank and File
fn generate_squares_between(s1: Square, s2: Square) -> Option<Bitboard> {
    let mut squares = Bitboard::EMPTY;

    if s1 == s2 {
        return None;
    }

    // Same rank
    if s1.rank() == s2.rank() {
        let mut current_square = std::cmp::min_by_key(s1, s2, |s| s.file());
        let end_square = std::cmp::max_by_key(s1, s2, |s| s.file());

        current_square = current_square.east();

        while current_square != end_square {
            squares.set_inplace(current_square);
            current_square = current_square.east();
        }

        return Some(squares);
    }

    // Same file
    if s1.file() == s2.file() {
        let mut current_square = std::cmp::min_by_key(s1, s2, |s| s.rank());
        let end_square = std::cmp::max_by_key(s1, s2, |s| s.rank());

        current_square = current_square.north();

        while current_square != end_square {
            squares.set_inplace(current_square);
            current_square = current_square.north();
        }

        return Some(squares);
    }

    // Diagonal
    if s1.file().idx().abs_diff(s2.file().idx()) == s1.rank().idx().abs_diff(s2.rank().idx()) {
        let mut current_square = std::cmp::min_by_key(s1, s2, |s| s.file());
        let end_square = std::cmp::max_by_key(s1, s2, |s| s.file());

        // We're starting with the leftmost of the two squares.
        // If that square is below our end square, we need to move up and to the right.
        // If that square is above our end square, we need to move below and to the right.
        let direction = if current_square.rank() < end_square.rank() {
            Direction::NorthEast
        } else {
            Direction::SouthEast
        };

        current_square = current_square.in_direction(direction);

        while current_square != end_square {
            squares.set_inplace(current_square);
            current_square = current_square.in_direction(direction);
        }

        return Some(squares);
    }

    // No path between these two squares
    None
}

pub fn init() {
    for s1 in Bitboard::FULL {
        for s2 in Bitboard::FULL {
            let between_squares = generate_squares_between(s1, s2);

            unsafe {
                BETWEEN_TABLE[s1.array_idx()][s2.array_idx()] = between_squares;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::squares::all::*;
    use spectral::assert_that;

    #[test]
    fn test_between_on_rank() {
        init();
        assert_eq!(between(B4, G4), C4 | D4 | E4 | F4);
    }

    #[test]
    fn test_between_on_rank_for_full_rank() {
        init();
        assert_eq!(between(A1, H1), B1 | C1 | D1 | E1 | F1 | G1);
    }

    #[test]
    fn test_between_on_file() {
        init();
        assert_eq!(between(C2, C7), C3 | C4 | C5 | C6);
    }

    #[test]
    fn test_between_on_file_for_full_file() {
        init();
        assert_eq!(between(H1, H8), H2 | H3 | H4 | H5 | H6 | H7);
    }

    #[test]
    fn test_between_on_diagonal() {
        init();
        assert_eq!(between(A1, H8), B2 | C3 | D4 | E5 | F6 | G7);
    }

    #[test]
    fn test_between_on_diagonal_descending() {
        init();
        assert_that(&between(A8, H1)).is_equal_to(B7 | C6 | D5 | E4 | F3 | G2);
    }
}