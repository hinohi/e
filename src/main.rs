use num::{Integer, ToPrimitive, bigint::BigUint};
use rayon::prelude::*;

struct Fraction {
    numerator: BigUint,
    denominator: BigUint,
}

impl Fraction {
    pub fn new(numerator: BigUint, denominator: BigUint) -> Fraction {
        Fraction {
            numerator,
            denominator,
        }
    }

    fn next_digit(&mut self) -> u32 {
        self.numerator *= BigUint::new(vec![10]);
        let d = (&self.numerator / &self.denominator).to_u32().unwrap();
        match d {
            0 => (),
            1 => self.numerator -= &self.denominator,
            _ => self.numerator -= &self.denominator * &BigUint::new(vec![d as _]),
        }
        d
    }
}

fn e_digits(digits: &[u8]) -> String {
    let mut buf = vec![b' ', b'2', b'.'];
    for digit in &digits[1..] {
        buf.push(*digit + b'0');
        if buf.len() % 61 == 60 {
            buf.push(b'\n');
        }
    }
    String::from_utf8(buf).unwrap()
}

fn calc_e(precision: usize) -> String {
    let mut digits = vec![2, 5];
    let mut terms = Vec::new();
    let mut i = BigUint::new(vec![2]);
    let mut numerator = BigUint::new(vec![10]);
    let mut denominator = BigUint::new(vec![2]);
    for _ in 3..10 {
        i.inc();
        denominator *= &i;
        terms.push(Fraction::new(numerator.clone(), denominator.clone()));
    }

    i.inc();
    denominator *= &i;
    let mut next_term = Fraction::new(numerator.clone(), denominator.clone());
    let mut current_precision_index = 0;
    for p in 0..precision {
        eprintln!("{} {}", p, terms.len());
        let mut digit = terms.par_iter_mut().map(|term| term.next_digit()).sum::<u32>();
        let d = next_term.next_digit();
        numerator *= BigUint::new(vec![10]);
        if d > 0 {
            digit += d;
            terms.push(next_term);
            i.inc();
            denominator *= &i;
            next_term = Fraction::new(numerator.clone(), denominator.clone());
        }
        digits.push(0);
        let mut index = digits.len() - 1;
        while digit > 0 {
            digits[index] += (digit % 10) as u8;
            digit /= 10;
            for j in (1..=index).rev() {
                if digits[j] >= 10 {
                    digits[j - 1] += digits[j] / 10;
                    digits[j] %= 10;
                } else {
                    break;
                }
            }
            index -= 1;
        }
        current_precision_index = index;
    }
    e_digits(&digits[..=current_precision_index])
}

fn main() {
    println!("{}", calc_e(100000));
}
