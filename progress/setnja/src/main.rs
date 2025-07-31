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
        let leftover = self.digits.iter_mut().fold(
            0,
            |carry, x| {
                let original = *x as u16;
                let carry_sum: u16 = original + carry as u16;
                let (div, modulo) = (carry_sum / 10, carry_sum % 10);
                *x = modulo as u8;
                div as u8
            });
        let (div, modulo) = (leftover / 10, leftover % 10);
        if modulo > 0 {
            self.digits.push(modulo);
        }
        if div > 0 {
            self.digits.push(div);
        }
    }
}

impl std::ops::Add for MediocreBigint {
    type Output = MediocreBigint;

    fn add(self, rhs: Self) -> Self::Output {
        let mut lhs = self.clone();
        lhs += rhs;
        lhs
    }
}

impl std::ops::AddAssign for MediocreBigint {
    fn add_assign(&mut self, rhs: Self) {
        // Go one digit at a time and add the digits from the left and right to the output, pushing to the vector.
        let mut left_iter = self.digits.iter_mut();
        let mut right_iter = rhs.digits.iter();
        let mut carry: u16 = 0;
        let mut right: Option<&u8> = None;
        for left in left_iter {
            right = right_iter.next();
            let left_value = *left;
            let right_value = *right.unwrap_or(&0u8);
            carry += left_value as u16 + right_value as u16;
            let (div, modulo) = if carry > 9 { (carry / 10, carry % 10) } else { (0, carry) };
            *left = modulo as u8;
            carry = div;
        }
        right = right_iter.next();
        while right.is_some() {
            carry += *right.unwrap() as u16;
            let (div, modulo) = if carry > 9 { (carry / 10, carry % 10) } else { (0, carry) };
            self.digits.push(modulo as u8);
            carry = div;
            right = right_iter.next();
        }
        while carry > 0 {
            let (div, modulo) = (carry / 10, carry % 10);
            self.digits.push(modulo as u8);
            carry = div;
        }
    }
}

impl std::ops::Mul for MediocreBigint {
    type Output = MediocreBigint;
    fn mul(self, rhs: Self) -> Self::Output {
        // Implement multiplication using the standard algorithm
        // For each digit in rhs, multiply it with all digits in self
        // Then add the results together with appropriate shifts
        
        if self.digits.is_empty() || rhs.digits.is_empty() {
            return MediocreBigint::new();
        }
        
        let mut result = MediocreBigint::new();
        
        for (i, &r_digit) in rhs.digits.iter().enumerate() {
            let mut partial_result = Vec::with_capacity(i + self.digits.len() + 2);
            
            // Add zeros for the shift based on position
            for _ in 0..i {
                assert!(partial_result.capacity() > partial_result.len());
                partial_result.push(0);
            }
            
            let mut carry = 0u16;
            
            // Multiply each digit of self by the current digit of rhs
            for &l_digit in self.digits.iter() {
                let product = (l_digit as u16) * (r_digit as u16) + carry;
                let (div, modulo) = (product / 10, product % 10);
                assert!(partial_result.capacity() > partial_result.len());
                partial_result.push(modulo as u8);
                carry = div;
            }
            
            // Handle any remaining carry
            while carry > 0 {
                let (div, modulo) = (carry / 10, carry % 10);
                assert!(partial_result.capacity() > partial_result.len());
                partial_result.push(modulo as u8);
                carry = div;
            }

            let partial_bigint = MediocreBigint { digits: partial_result };
            
            // Add this partial result to the total
            result = result + partial_bigint;
        }
        
        result
    }
}

impl std::ops::MulAssign for MediocreBigint {
    fn mul_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl std::fmt::Display for MediocreBigint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert_eq!(self.is_normalized(), true);

        // Read digit by digit and write out char by char
        if self.digits.len() > 0 {
            self.digits.iter().rev().map(ToString::to_string).collect::<String>().fmt(f)
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
        segments.reverse();
        Ok(MediocreBigint { digits: segments })
    }
}

fn read_str() -> String {
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");
    response.trim_end().to_string()
}

fn solve(walk: &str) -> String {
    use std::str::FromStr;

    let mut accumulator = MediocreBigint::from_str("1").unwrap();
    let mut multiplexor : MediocreBigint = accumulator.clone();

    let five = MediocreBigint::from_str("5").unwrap();
    let three = MediocreBigint::from_str("3").unwrap();
    let two = MediocreBigint::from_str("2").unwrap();
    let steps = walk.chars();
    for step in steps {
        match step {
            '*' => {
                accumulator = accumulator.clone() * five.clone() + multiplexor.clone();
                multiplexor = multiplexor * three.clone();
            }
            'P' => {
                accumulator = accumulator
            }
            'L' => {
                accumulator = accumulator * two.clone()
            }
            'R' => {
                accumulator = accumulator * two.clone() + multiplexor.clone();
            }
            _ => panic!("This should not happen"),
        }
    }
    accumulator.normalize();
    accumulator.to_string()
}

fn main() {
    let walk = read_str();
    println!("{}", solve(walk.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_maximal_star() {
        solve("*".repeat(10000).as_str());
    }

    #[test]
    fn test_solve_middle_star() {
        assert_eq!(solve("P*P"), "6");
    }

    #[test]
    fn test_solve_repeated_alternating() {
        assert_eq!(solve("LLLLLRRRRRLLLLLRRRRRLLLLLRRRRRLLLLL"), "35400942560");
    }

    #[test]
    fn test_solve_left_star() {
        assert_eq!(solve("L*"), "11");
    }

    #[test]
    fn test_solve_right_star() {
        assert_eq!(solve("R*"), "16");
    }

    #[test]
    fn test_solve_double_star() {
        assert_eq!(solve("**"), "33");
    }

    #[test]
    fn test_solve_right_star_right() {
        assert_eq!(solve("R*R"), "35");
    }

    #[test]
    fn test_solve_left_star_right() {
        assert_eq!(solve("L*R"), "25");
    }

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
    fn test_mul_zero() {
        let a = MediocreBigint { digits: vec![1, 0, 0, 1, 1] };
        let zero = MediocreBigint::new();
        assert_eq!(zero.to_string(), "0");
        let c = a.clone() * zero.clone();
        assert_eq!(c.to_string(), "0");

        let d = zero.clone() * a.clone();
        assert_eq!(d.to_string(), "0");
    }
    
    #[test]
    fn test_mul_small() {
        let a = MediocreBigint { digits: vec![2] };
        let b = MediocreBigint { digits: vec![3] };
        let c = a * b;
        assert_eq!(c.digits, vec![6]);
    }
    
    #[test]
    fn test_mul_carry() {
        let a = MediocreBigint { digits: vec![7] };
        let b = MediocreBigint { digits: vec![8] };
        let c = a * b;
        assert_eq!(c.digits, vec![6, 5]);
    }
    
    #[test]
    fn test_mul_larger() {
        use std::str::FromStr;
        let a = MediocreBigint::from_str("123").unwrap();
        let b = MediocreBigint::from_str("456").unwrap();
        let c = a * b;
        assert_eq!(c, MediocreBigint::from_str("56088").unwrap());
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
        let b = MediocreBigint { digits: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2] };
        let c = a.clone() + b;
        assert_eq!(c.digits, vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
    }

    #[test]
    fn test_add_carry() {
        let a = MediocreBigint::from_str("1776").unwrap();
        let b = MediocreBigint::from_str("76").unwrap();
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
        assert_eq!(b.digits, vec![5, 5, 2]);

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
        let b = MediocreBigint { digits: vec![2, 1, 1] };
        let c = MediocreBigint { digits: vec![0, 0, 1, 20]};
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
        let mut a = MediocreBigint { digits: vec![1, 10, 1, 1, 1] };
        let b = a.clone();
        assert_eq!(a, b);
        a.normalize();
        assert_ne!(a, b);
        assert!(a.is_normalized());
        assert!(a.digits.eq(&vec![1, 0, 2, 1, 1]));
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
        assert_eq!(a.digits, vec![0, 1]);
    }
}
