use std::collections::HashSet;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate lazy_static;

pub mod game;
pub mod game_move;
pub mod game_state;
pub mod legal_moves;
pub mod position_data;
pub mod standard_game;

pub fn is_triangular(number: usize) -> bool {
    // See http://mathforum.org/library/drmath/view/57162.html
    is_perfect_odd_sqrt(8 * number + 1)
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

// =================================================================================================
// Tests
// =================================================================================================

#[test]
fn test_is_triangular_3_is_true() {
    assert_eq!(is_triangular(3), true);
}

#[test]
fn test_is_triangular_4_is_false() {
    assert_eq!(is_triangular(4), false);
}

#[test]
fn test_is_triangular_6_is_true() {
    assert_eq!(is_triangular(6), true);
}

#[test]
fn test_is_triangular_630_is_true() {
    assert_eq!(is_triangular(630), true);
}

#[test]
fn test_is_triangular_631_is_false() {
    assert_eq!(is_triangular(631), false);
}
