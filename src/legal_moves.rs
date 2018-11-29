use crate::position_data::PositionData;

#[derive(Debug, PartialEq, Eq)]
pub struct LegalMoves(pub Vec<PositionData>);

impl LegalMoves {
    /// Given a `Vec` of `PositionData`, return an instance of `LegalMoves`.
    pub fn new(input: Vec<PositionData>) -> Result<Self, Box<dyn std::error::Error>> {
        if Self::is_triangular(input.len()) {
            Ok(LegalMoves(input))
        } else {
            Err("The length of the `LegalMoves` input must be a triangular number")?
        }
    }

    fn is_triangular(number: usize) -> bool {
        // See http://mathforum.org/library/drmath/view/57162.html
        Self::is_perfect_odd_sqrt(8 * number + 1)
    }

    // Shamelessly stolen from
    // https://gist.github.com/anthonyclays/6c10d17a9caf8767059b
    fn isqrt(n: usize) -> usize {
        if n == 0 {
            return n;
        }
        let mut s = (n as f64).sqrt() as usize;
        s = (s + n / s) >> 1;
        if s * s > n {
            s - 1
        } else {
            s
        }
    }

    fn is_perfect_odd_sqrt(n: usize) -> bool {
        match n & 0xf {
            0 | 1 | 4 | 9 => {
                let t = Self::isqrt(n);
                t * t == n && n % 2 != 0
            }
            _ => false,
        }
    }
}

// =================================================================================================
// Tests
// =================================================================================================

#[test]
fn test_is_triangular_3_is_true() {
    assert_eq!(LegalMoves::is_triangular(3), true);
}

#[test]
fn test_is_triangular_4_is_false() {
    assert_eq!(LegalMoves::is_triangular(4), false);
}

#[test]
fn test_is_triangular_6_is_true() {
    assert_eq!(LegalMoves::is_triangular(6), true);
}

#[test]
fn test_is_triangular_630_is_true() {
    assert_eq!(LegalMoves::is_triangular(630), true);
}

#[test]
fn test_is_triangular_631_is_false() {
    assert_eq!(LegalMoves::is_triangular(631), false);
}
