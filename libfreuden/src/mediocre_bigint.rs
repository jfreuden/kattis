/// MediocreBigint is a bad implementation of bigint making use of smaller limbs to handle carries
/// Each digit is base 10 input or output, i.e. between 0 and 9
/// The digit capacity is 8 bits, i.e. between 0 and 255
/// the carry hides in the higher bits and get normalized back out into the digits
/// output must simply read out the digits as a string
pub type MediocreDigitSize = u64;
pub type MediocreCarrySize = u64;
const LIMIT_POWER: u32 = 17;
const LIMIT_BASE: MediocreCarrySize = 10;
pub const BIGINT_LIMIT: MediocreCarrySize = MediocreCarrySize::pow(LIMIT_BASE, LIMIT_POWER);

#[derive(Debug, Clone, PartialOrd, PartialEq, Default)]
pub struct MediocreBigint {
    // TODO: Correct the setnja test usage of this so that I can make this not public (and anagramcounting)
    pub digits: Vec<MediocreDigitSize>,
}

impl MediocreBigint {
    pub fn new() -> MediocreBigint {
        let segments = Vec::new();
        MediocreBigint { digits: segments }
    }

    #[allow(unused)]
    pub fn is_normalized(&self) -> bool {
        self.digits
            .iter()
            .all(|digit| *digit < (BIGINT_LIMIT as MediocreDigitSize))
    }

    pub fn normalize(&mut self) {
        let leftover = self.digits.iter_mut().fold(0, |carry, x| {
            let original = *x as MediocreCarrySize;
            let carry_sum: MediocreCarrySize = original + carry as MediocreCarrySize;
            let (div, modulo) = (carry_sum / BIGINT_LIMIT, carry_sum % BIGINT_LIMIT);
            *x = modulo as MediocreDigitSize;
            div as MediocreDigitSize
        });
        let (div, modulo) = (
            leftover / BIGINT_LIMIT as MediocreDigitSize,
            leftover % BIGINT_LIMIT as MediocreDigitSize,
        );
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
        let mut lhs = self;
        lhs += rhs;
        lhs
    }
}

impl std::ops::AddAssign for MediocreBigint {
    fn add_assign(&mut self, rhs: Self) {
        // Delegate to the reference implementation
        self.add_assign(&rhs);
    }
}

impl std::ops::AddAssign<&MediocreBigint> for MediocreBigint {
    fn add_assign(&mut self, rhs: &MediocreBigint) {
        let overlap = std::cmp::min(self.digits.len(), rhs.digits.len());
        let mut left_iter = self.digits[..overlap].iter_mut();
        let mut right_iter = rhs.digits[..overlap].iter();
        let mut carry: MediocreCarrySize = 0;

        for (left, right) in left_iter.zip(right_iter) {
            // Handle when we have both values
            carry += *left as MediocreCarrySize + *right as MediocreCarrySize;
            (carry, *left) = (
                carry / BIGINT_LIMIT,
                (carry % BIGINT_LIMIT) as MediocreDigitSize,
            );
        }

        left_iter = self.digits[overlap..].iter_mut();
        right_iter = rhs.digits[overlap..].iter();

        if let Some(left) = left_iter.next() {
            *left += carry as MediocreDigitSize;
        } else if let Some(right) = right_iter.next() {
            self.digits.push(*right + carry as MediocreDigitSize);
            self.digits
                .append(right_iter.copied().collect::<Vec<_>>().as_mut());
        } else if carry > 0 {
            self.digits.push(carry as MediocreDigitSize);
        }
    }
}

impl std::ops::Mul for MediocreBigint {
    type Output = MediocreBigint;
    fn mul(self, rhs: Self) -> Self::Output {
        // Use the optimized MulAssign implementation
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl std::ops::MulAssign for MediocreBigint {
    fn mul_assign(&mut self, rhs: Self) {
        // Delegate to the reference implementation
        self.mul_assign(&rhs);
    }
}

impl std::ops::MulAssign<&MediocreBigint> for MediocreBigint {
    fn mul_assign(&mut self, rhs: &MediocreBigint) {
        // Handle empty cases
        if self.digits.is_empty() || rhs.digits.is_empty() {
            self.digits.clear();
            return;
        }

        // Special case for single digit multiplication to avoid unnecessary allocations
        if rhs.digits.len() == 1 && rhs.digits[0] < BIGINT_LIMIT as MediocreDigitSize {
            let r_digit = rhs.digits[0];
            if r_digit == 0 {
                self.digits.clear();
                return;
            }
            if r_digit == 1 {
                return; // Identity
            }

            let mut carry = MediocreCarrySize::default();
            // Multiply in place
            for digit in &mut self.digits {
                let product =
                    (*digit as MediocreCarrySize) * (r_digit as MediocreCarrySize) + carry;
                let (div, modulo) = (product / BIGINT_LIMIT, product % BIGINT_LIMIT);
                *digit = modulo as MediocreDigitSize;
                carry = div;
            }

            // Handle any remaining carry
            while carry > 0 {
                let (div, modulo) = (carry / BIGINT_LIMIT, carry % BIGINT_LIMIT);
                self.digits.push(modulo as MediocreDigitSize);
                carry = div;
            }
            return;
        }

        // For general case, we need a separate result vector
        // Store original digits and clear self to reuse it
        let original_digits = std::mem::take(&mut self.digits);
        let original_self = MediocreBigint {
            digits: original_digits,
        };

        // Pre-allocate result with enough capacity for the worst case
        let mut result = MediocreBigint::new();
        result
            .digits
            .reserve(original_self.digits.len() + rhs.digits.len() + 1);

        // For each digit in rhs, multiply it with all digits in original_self
        for (i, &r_digit) in rhs.digits.iter().enumerate() {
            // Skip multiplication by zero
            if r_digit == 0 {
                continue;
            }

            let mut partial_result = Vec::with_capacity(i + original_self.digits.len() + 2);

            // Add zeros for the shift based on position
            partial_result.extend(std::iter::repeat(0).take(i));

            let mut carry = MediocreCarrySize::default();

            // Multiply each digit of original_self by the current digit of rhs
            for &l_digit in original_self.digits.iter() {
                let product =
                    (l_digit as MediocreCarrySize) * (r_digit as MediocreCarrySize) + carry;
                let (div, modulo) = (product / BIGINT_LIMIT, product % BIGINT_LIMIT);
                partial_result.push(modulo as MediocreDigitSize);
                carry = div;
            }

            // Handle any remaining carry
            while carry > 0 {
                let (div, modulo) = (carry / BIGINT_LIMIT, carry % BIGINT_LIMIT);
                partial_result.push(modulo as MediocreDigitSize);
                carry = div;
            }

            // Create partial_bigint without cloning
            let partial_bigint = MediocreBigint {
                digits: partial_result,
            };

            // Add this partial result to the total
            result += partial_bigint;
        }

        // Move the result back into self
        self.digits = result.digits;
    }
}

impl std::ops::Div for MediocreBigint {
    type Output = MediocreBigint;
    fn div(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        result /= rhs;
        result
    }
}

impl std::ops::DivAssign for MediocreBigint {
    fn div_assign(&mut self, rhs: Self) {
        self.div_assign(&rhs);
    }
}

impl std::ops::DivAssign<&MediocreBigint> for MediocreBigint {
    fn div_assign(&mut self, rhs: &MediocreBigint) {
        // Zero divisor check (consider empty or all-zero as zero)
        let is_rhs_zero = rhs.digits.is_empty() || rhs.digits.iter().all(|&d| d == 0);
        if is_rhs_zero {
            panic!("attempt to divide by zero");
        }

        // If self is zero, result is zero
        let is_self_zero = self.digits.is_empty() || self.digits.iter().all(|&d| d == 0);
        if is_self_zero {
            self.digits.clear();
            return;
        }

        // Optimize single-limb divisor
        if rhs.digits.len() == 1 {
            let divisor = rhs.digits[0] as u128;
            if divisor == 1 {
                return; // identity
            }

            let base = BIGINT_LIMIT as u128;
            let mut rem: u128 = 0;

            // Prepare quotient vector with same length
            let n = self.digits.len();
            let mut q: Vec<MediocreDigitSize> = vec![0; n];

            // Process from most significant limb to least
            for i in (0..n).rev() {
                let cur = rem * base + self.digits[i] as u128;
                let qdigit = cur / divisor;
                rem = cur % divisor;
                q[i] = qdigit as MediocreDigitSize;
            }

            // Trim leading zeros (highest limbs)
            while q.last().copied() == Some(0) {
                q.pop();
            }

            self.digits = q;
            return;
        }

        // TODO: multi-limb divisor not yet implemented for this mediocre bigint
        // Implementing full Knuth division is beyond current needs of this project.
        unimplemented!("Division by multi-limb MediocreBigint is not implemented");
    }
}

impl std::fmt::Display for MediocreBigint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display = self.clone();
        display.normalize();
        // Read digit by digit and write out char by char
        if display.digits.len() > 0 {
            let mut string = String::new();
            for digit in display.digits {
                let flipped = digit.to_string().chars().rev().collect::<String>();
                let fmtted = format!("{:0<width$}", flipped, width = (LIMIT_POWER) as usize);
                let padded = fmtted;
                string.push_str(&padded);
            }
            string
                .trim_end_matches('0')
                .chars()
                .rev()
                .collect::<String>()
                .fmt(f)
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
        for chunk in s.as_bytes().rchunks(LIMIT_POWER as usize) {
            let substr = String::from_utf8_lossy(chunk);
            let digit = substr.parse()?;
            segments.push(digit);
        }
        Ok(MediocreBigint { digits: segments })
    }
}
