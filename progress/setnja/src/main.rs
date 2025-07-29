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


        todo!()
    }

}

impl std::ops::Add for MediocreBigint {
    type Output = MediocreBigint;

    fn add(self, rhs: Self) -> Self::Output {
        // go through the digits and add rhs to self
        // if one of the additions would overflow, carry the remainder into the next digit
    }
}

impl std::ops::Mul for MediocreBigint {
    type Output = MediocreBigint;
    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Div for MediocreBigint {
    type Output = MediocreBigint;
    fn div(self, rhs: Self) -> Self::Output {
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
        let c = a.clone() + zero;
        assert_eq!(c, a);

        let d = c.clone() + a.clone();
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
        let c = a + b;
        use std::str::FromStr;
        assert_eq!(c, MediocreBigint::from_str(&(1776 + 76).to_string()).unwrap());
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
