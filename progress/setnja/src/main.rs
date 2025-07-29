/// MediocreBigint is a bad implementation of bigint making use of smaller limbs to handle carries
/// Each digit is base 10 input or output, i.e. between 0 and 9
/// The digit capacity is 8 bits, i.e. between 0 and 255
/// the carry hides in the higher bits and get normalized back out into the digits
/// output must simply read out the digits as a string
#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct MediocreBigint {
    digits: Vec<u8>

}

impl MediocreBigint {
    fn new() -> MediocreBigint {
        let segments = Vec::new();
        MediocreBigint { digits: segments }
    }

    fn is_normalized(&self) -> bool {
        self.digits.iter().fold(true, |normal, &digit| { normal && digit < 10_u8 })
    }

    fn normalize(&mut self) {
        let leftover = self.digits.iter_mut().rfold(
            0,
            |carry, x| {
                let original = *x as u16;
                let carry_sum: u16 = original + carry as u16;
                let (div, modulo) = (carry_sum / 10, carry_sum % 10);
                *x = modulo as u8;
                div as u8
            });
        let (div, modulo) = (leftover / 10, leftover % 10);
        let mut newfront = vec![];
        if div > 0 {
            newfront.push(div);
        }
        if modulo > 0 {
            newfront.push(modulo);
        }
        if newfront.len() > 0 {
            newfront.append(self.digits.as_mut());
            self.digits = newfront;
        }
        // can go from the tail and carry upwards
        // can reverse and return a new normalized
        // can split out normalized vector, and a carry vector, then shift over

        // I hate the idea this could have multiple carry
        //    000 255 255 255 255 255
        //     25  25  25  25  25   0
        //          5   5   5   5   5
        //    =======================
        //     25  30  30  30  30   5
        // which is, and this is true, NOT decimal.
        //      5   0   0   0   0   5
        //+ 2   3   3   3   3   0   0
        // ==========================
        //= 2   8   3   3   3   0   5
        // Maybe I do the split and add version, but I do it by hundreds AND tens?
        // Still can end up with multiple overflow tho... or can I?
        //
        //    000 000 000 000 255 255
        //ones                  5   5
        //tens              5   5   0
        //hundreds      2   2   0   0
        // ==========================
        //              2   7  10   5
        // similar issue. All for the following reasons:
        // ones has max 9.
        // tens has max 9.
        // hundreds has max 2.
        // So it can always overflow again. That being said, 9+9+2 is the max that could be demanded
        // in a normalization stage. so a max carry of 2.
        // Is there a better way of splitting out or handling the carries? It would be nice
        // maybe just being okay adding up to 25 was better. It can never go over 34 that way.

    }

}

impl std::ops::Add for MediocreBigint {
    type Output = MediocreBigint;

    fn add(self, rhs: Self) -> Self::Output {
        // Go one digit at a time and add the digits from the left and right to the output, pushing to the vector.
        let left_digits: Vec<u8> = self.digits.iter().rev().cloned().collect();
        let right_digits: Vec<u8> = rhs.digits.iter().rev().cloned().collect();
        let digit_count = std::cmp::max(left_digits.len(), right_digits.len());
        let mut output_digits: Vec<u8> = Vec::with_capacity(digit_count);

        let mut left_iter = left_digits.iter();
        let mut right_iter = right_digits.iter();
        let mut carry: u8 = 0;
        for _ in 0..digit_count {
            let left = left_iter.next();
            let right = right_iter.next();

            if left.is_none() && right.is_none() {
                panic!("This should not happen");
            } else {
                let left_value = *left.unwrap_or(&0u8);
                let right_value = *right.unwrap_or(&0u8);
                let (out_value, overflowed) = left_value.overflowing_add(right_value);
                if overflowed {
                    // what happens when I add 000 255 to 255 255 and carry happens?
                    // carries at most 25 up, but gets added in the next stage
                    // also, I have to handle when new digit is added
                    // the case where I overflowed and then added a carry is okay, it'll just be non-normalized
                    // but the case where I did NOT overflow and then added a carry that will is honestly more annoying
                    todo!("overflow");
                }
                output_digits.push(out_value);
            }
        }
        if carry > 0 {
            output_digits.push(carry);
            carry = 0;
        }

        output_digits.reverse();
        return MediocreBigint{digits: output_digits}
    }
}

impl std::ops::Mul for MediocreBigint {
    type Output = MediocreBigint;
    fn mul(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Div for MediocreBigint {
    type Output = MediocreBigint;
    fn div(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::fmt::Display for MediocreBigint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert_eq!(self.is_normalized(), true);

        // Read digit by digit and write out char by char
        if self.digits.len() > 0 {
            self.digits.iter().map(ToString::to_string).collect::<String>().fmt(f)
        } else {
            0u8.fmt(f)
        }

    }
}

impl std::str::FromStr for MediocreBigint {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Read char by char and write out digit by digit
        let mut segments = if s.len() > 0 {
            Vec::with_capacity(s.len())
        } else {
            Vec::new()
        };

        for char in s.chars() {
            segments.push(u8::from_str_radix(&char.to_string(), 10)?);
        }

        Ok(MediocreBigint { digits: segments })
    }
}




fn main() {
    use std::str::FromStr;
    println!("{}", MediocreBigint::from_str("").unwrap());
}


// implement operator for a custom shitbigint struct (positives only)

// use i64 and then use negative to do the carry?
// use something else?
// type is basically a VecDeque which can scale upward to add "digits"
// with the custom add, will need to add with carry
// multiply, probably by splitting the bits and then doing it


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_add_zero() {
        let a = MediocreBigint { digits: vec![1, 0, 0, 1, 1] };
        let zero = MediocreBigint::new();
        assert_eq!(zero.to_string(), "0");
        let c = a.clone() + zero.clone();
        assert_eq!(c, a);

        let d = zero + a.clone();
        assert_eq!(d, a);
    }

    #[test]
    fn test_add_small() {
        let a = MediocreBigint { digits: vec![2] };
        let b = MediocreBigint { digits: vec![2] };
        let c = a.clone() + b;
        assert_eq!(c.digits, vec![4]);
    }

    #[test]
    fn test_add_big() {
        let a = MediocreBigint { digits: vec![2] };
        let b = MediocreBigint { digits: vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] };
        let c = a.clone() + b;
        assert_eq!(c.digits, vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
    }

    #[test]
    fn test_add_carry() {
        let a = MediocreBigint { digits: vec![1, 7, 7, 6] };
        let b = MediocreBigint { digits: vec![7, 6] };
        let mut c = a + b;
        c.normalize();
        use std::str::FromStr;
        assert_eq!(c, MediocreBigint::from_str(&(1776 + 76).to_string()).unwrap());
    }

    #[test]
    fn test_add_mega_denorm() {
        let a = MediocreBigint { digits: vec![255] };
        let mut b = a.clone();
        b.normalize();
        assert_eq!(a.digits, vec![255]);
        assert_eq!(b.digits, vec![2, 5, 5]);

        let mut c = a.clone() + a.clone();


        let mut d = a.clone() + b.clone();


        let mut e = b.clone() + b.clone();


        c.normalize();
        d.normalize();
        e.normalize();
        assert_eq!(c, d);
        assert_eq!(d, e);
    }

    #[test]
    fn test_create() {
        let int = MediocreBigint::new();
        assert_eq!(int.digits.len(), 0);
    }

    #[test]
    fn test_empty_is_zero() {
        let int = MediocreBigint::new();
        assert_eq!(int.to_string(), "0");
    }

    #[test]
    fn test_is_normalized() {
        let a = MediocreBigint { digits: vec![] };
        let b = MediocreBigint { digits: vec![1, 1, 2] };
        let c = MediocreBigint { digits: vec![1, 20, 0, 0]};
        assert!(a.is_normalized());
        assert!(b.is_normalized());
        assert_eq!(c.is_normalized(), false);
    }

    #[test]
    fn test_clone() {
        let a = MediocreBigint { digits: vec![1, 1, 1, 2, 1] };
        let mut b = a.clone();
        assert_eq!(a, b);
        b.digits[0] = 2;
        assert_ne!(a, b);
    }

    #[test]
    fn test_normalize() {
        let mut a = MediocreBigint { digits: vec![1, 1, 1, 10, 1] };
        let b = a.clone();
        assert_eq!(a, b);
        a.normalize();
        assert_ne!(a, b);
        assert!(a.is_normalized());
        assert!(a.digits.eq(&vec![1, 1, 2, 0, 1]));

    }

    #[test]
    fn test_normalize_zero() {
        let mut a = MediocreBigint { digits: vec![] };
        a.normalize();
        assert_eq!(a.digits, vec![]);
    }

    #[test]
    fn test_normalize_create_digit() {
        let mut a = MediocreBigint { digits: vec![10] };
        a.normalize();
        assert_eq!(a.digits, vec![1, 0]);
    }


}
