use std::io::{self, Write};

use num::{bigint::BigInt, Integer, ToPrimitive, Zero};

/// k番目の連分数項を返す
/// e = [2; 1, 2, 1, 1, 4, 1, 1, 6, 1, 1, 8, ...]
fn cf_term(k: usize) -> u64 {
    if k == 0 {
        return 2;
    }
    if k % 3 == 2 {
        (2 * (k / 3 + 1)) as u64
    } else {
        1
    }
}

struct EDigits {
    q: BigInt,
    r: BigInt,
    s: BigInt,
    t: BigInt,
    k: usize,
}

impl EDigits {
    fn new() -> Self {
        EDigits {
            q: BigInt::from(1),
            r: BigInt::from(0),
            s: BigInt::from(0),
            t: BigInt::from(1),
            k: 0,
        }
    }

    fn absorb(&mut self) {
        let a = BigInt::from(cf_term(self.k));
        self.k += 1;
        let new_q = &self.q * &a + &self.r;
        let new_r = self.q.clone();
        let new_s = &self.s * &a + &self.t;
        let new_t = self.s.clone();
        self.q = new_q;
        self.r = new_r;
        self.s = new_s;
        self.t = new_t;
    }

    fn extract(&self) -> Option<u8> {
        if self.s.is_zero() || (&self.s + &self.t).is_zero() {
            return None;
        }
        let d1 = (&self.q).div_floor(&self.s);
        let d2 = (&self.q + &self.r).div_floor(&(&self.s + &self.t));
        if d1 == d2 {
            d1.to_u8()
        } else {
            None
        }
    }

    fn produce(&mut self, d: u8) {
        let d = BigInt::from(d);
        let new_q = BigInt::from(10) * &self.q - BigInt::from(10) * &d * &self.s;
        let new_r = BigInt::from(10) * &self.r - BigInt::from(10) * &d * &self.t;
        self.q = new_q;
        self.r = new_r;
    }
}

impl Iterator for EDigits {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        loop {
            if let Some(d) = self.extract() {
                self.produce(d);
                return Some(d);
            }
            self.absorb();
        }
    }
}

fn main() {
    let no_newline = std::env::args().any(|a| a == "--raw" || a == "-r");
    let out = io::stdout();
    let mut writer = io::BufWriter::new(out.lock());
    let mut col = 0;
    for digit in EDigits::new() {
        if write!(writer, "{}", digit).is_err() {
            return;
        }
        if !no_newline {
            col += 1;
            if col == 60 {
                if writeln!(writer).is_err() {
                    return;
                }
                col = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cf_term() {
        // e = [2; 1, 2, 1, 1, 4, 1, 1, 6, 1, 1, 8, ...]
        assert_eq!(cf_term(0), 2);
        assert_eq!(cf_term(1), 1);
        assert_eq!(cf_term(2), 2);
        assert_eq!(cf_term(3), 1);
        assert_eq!(cf_term(4), 1);
        assert_eq!(cf_term(5), 4);
        assert_eq!(cf_term(6), 1);
        assert_eq!(cf_term(7), 1);
        assert_eq!(cf_term(8), 6);
        assert_eq!(cf_term(9), 1);
        assert_eq!(cf_term(10), 1);
        assert_eq!(cf_term(11), 8);
    }

    #[test]
    fn test_first_20_digits() {
        let digits: Vec<u8> = EDigits::new().take(20).collect();
        // e = 2.7182818284590452353...
        assert_eq!(
            digits,
            vec![2, 7, 1, 8, 2, 8, 1, 8, 2, 8, 4, 5, 9, 0, 4, 5, 2, 3, 5, 3]
        );
    }
}
