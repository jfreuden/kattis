fn read_vec<T: std::str::FromStr>() -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|tok| tok.parse::<T>().expect("Failed to parse input"))
        .collect()
}

fn try_read_array<T: std::str::FromStr, const K: usize, E: std::fmt::Debug>() -> Result<[T; K], E>
where
    T::Err: std::fmt::Debug,
    [T; K]: TryFrom<Vec<T>, Error = E>,
{
    read_vec::<T>().try_into()
}

fn read_array<T: std::str::FromStr, const K: usize, E: std::fmt::Debug>() -> [T; K]
where
    T::Err: std::fmt::Debug,
    [T; K]: TryFrom<Vec<T>, Error = E>,
{
    try_read_array().unwrap()
}

type IndexType = usize;
type ValueType = i64;

struct Op {
    index: IndexType,
    value: ValueType,
}

impl Op {
    fn new_query(index: IndexType) -> Self {
        Op {
            index,
            value: ValueType::MAX,
        }
    }

    fn new_increment(index: IndexType, value: ValueType) -> Self {
        Op {
            index,
            value,
        }
    }

    fn is_query(&self) -> bool {
        self.value == ValueType::MAX
    }
}

#[inline(always)]
fn bit_query_indices(query_index: usize) -> Vec<usize> {
    let logged_size = (query_index.ilog2() + 1) as usize;
    let mut query_indices = Vec::with_capacity(logged_size);
    let mut mask = 0usize;
    let mut last_pushed = 0;
    while query_index & mask != query_index {
        let candidate = query_index & (!mask);
        if candidate != last_pushed {
            query_indices.push(candidate);
            last_pushed = candidate;
        }
        mask = (mask << 1) + 1;
    }
    query_indices.reverse();
    query_indices
}

#[inline(always)]
fn bit_increment_indices(increment_index: usize, starting_bit_value: usize, max_index: usize) -> Vec<usize> {
    let working_index = !increment_index;
    let mut bit_value = starting_bit_value;

    let mut index_boost = 0;
    let mut write_indices = Vec::with_capacity((max_index.ilog2() + 1) as usize);
    while working_index != 0 && bit_value > 0 {
        if bit_value + index_boost > max_index {
            // TODO: Find a better way to avoid these invalid indices
        }
        else if (working_index & bit_value) != 0 {
            write_indices.push(bit_value + index_boost);
        } else {
            index_boost += bit_value;
        }

        bit_value >>= 1;
    }
    write_indices.reverse();
    write_indices
}

#[inline(always)]
fn standard_increment_indices(increment_index: usize, max_index: usize) -> Vec<usize> {
    let mut query_indices = Vec::with_capacity((max_index.ilog2() + 1) as usize);
    let mut working_index = increment_index as ValueType + 1;

    while working_index <= max_index as i64 {
        query_indices.push(working_index as IndexType);

        working_index = working_index + (working_index & -working_index);
    }

    query_indices
}

#[inline(always)]
fn standard_query_indices(query_index: usize, max_index: usize) -> Vec<usize> {
    let mut write_indices = Vec::with_capacity((max_index.ilog2() + 1) as usize);
    let mut working_index = query_index as ValueType;
    while working_index > 0 {
        write_indices.push(working_index as IndexType);

        working_index = working_index - (working_index & (-working_index));
    }

    write_indices
}

struct FenwickTree {
    data_fenwick: Vec<ValueType>,
    standard_formulas: bool,
    _starting_bit_value: usize
}

impl FenwickTree {
    fn new(array_len: usize) -> Self {
        let starting_bit_value = {
            let two_pow = (array_len + 1).next_power_of_two() >> 1;
            if two_pow == 0 {
                1usize
            } else {
                two_pow
            }
        };
        FenwickTree {
            data_fenwick: vec![0 as ValueType; array_len],
            standard_formulas: false,
            _starting_bit_value: starting_bit_value,
        }
    }

    fn new_standard(array_len: usize) -> Self {
        FenwickTree {
            data_fenwick: vec![0 as ValueType; array_len],
            standard_formulas: true,
            _starting_bit_value: 0,
        }
    }

    fn increment(&mut self, index: usize, value: ValueType) {
        let increment_indices = if self.standard_formulas {
            standard_increment_indices(index, self.data_fenwick.len())
        } else {
            bit_increment_indices(index, self._starting_bit_value, self.data_fenwick.len())
        };
        for i in increment_indices {
            self.data_fenwick[i - 1] += value;
        }
    }

    fn query(&self, index: usize) -> ValueType {
        let query_index = index;
        let answer = if query_index == 0 {
            0
        } else {
            let query_indices = if self.standard_formulas {
                standard_query_indices(query_index, self.data_fenwick.len())
            } else {
                bit_query_indices(query_index)
            };
            query_indices.iter().map(|&i| self.data_fenwick[i - 1]).sum()
        };
        answer
    }
}

fn main() {
    let [array_len, operations_count]: [usize; 2] = read_array();

    let mut fenwick = FenwickTree::new(array_len);

    for _ in 0..operations_count {
        let op = read_vec::<String>();
        match op.len() {
            2 => {
                // Query Operation
                let [key, index]: [String; 2] = op.try_into().unwrap();
                if key != "?" {
                    panic!("Invalid operation")
                }
                let query_index = index.parse::<IndexType>().unwrap();
                println!("{}", fenwick.query(query_index));
            }
            3 => {
                // Increment Operation
                let [key, index, delta]: [String; 3] = op.try_into().unwrap();
                if key != "+" {
                    panic!("Invalid operation")
                }
                let increment_index = index.parse::<IndexType>().unwrap();
                let increment_value = delta.parse::<ValueType>().unwrap();
                fenwick.increment(increment_index, increment_value);
            }
            _ => panic!("Invalid operation"),
        }
    }
}

#[cfg(test)]
mod fenwick_tests {
    use super::*;

    fn fast_solve(array_len: usize, operations_list: Vec<Op>) -> Vec<ValueType> {
        let mut fenwick = FenwickTree::new_standard(array_len);

        let operations_count = operations_list.len();
        let mut answers = Vec::<ValueType>::with_capacity(operations_count);

        for op in operations_list {
            if op.is_query() {
                let answer = fenwick.query(op.index);
                answers.push(answer);
            } else {
                fenwick.increment(op.index, op.value);
            }
        }

        answers
    }

    #[test]
    fn test_bit_query_indices() {
        let max_index = 12;
        for i in 1..=max_index {
            let proposed_query_indices = bit_query_indices(i);
            println!("Prefix Query {:2} ({:06b} -> {:06b}) of {:2}: {:?}", i, i, !i & 0b111111, max_index, proposed_query_indices);
        }
    }

    #[test]
    fn test_bit_increment_indices() {
        let max_index: usize = 12;
        let starting_bit_value = {
            let two_pow = (max_index + 1).next_power_of_two() >> 1;
            if two_pow == 0 {
                1usize
            } else {
                two_pow
            }
        };

        let mut keepalive_counter = 0;
        for i in 0..max_index {
            let proposed_assign_indices = bit_increment_indices(i, starting_bit_value, max_index);
            println!("Assign {:2} ({:06b} -> {:06b}) of {:2}: {:?}\r", i, i, !i & 0b111111, max_index, proposed_assign_indices);
            keepalive_counter += proposed_assign_indices.len();
        }

        println!("Incremented assignment {}", keepalive_counter);

    }


    #[test]
    fn test_solve_sample_1() {
        let array_len = 10 as usize;
        let operation_list: Vec<Op> = vec![
            Op::new_increment(7, 23),
            Op::new_query(8),
            Op::new_increment(3, 17),
            Op::new_query(8),
        ];
        let query_results = fast_solve(array_len, operation_list);

        let answers = vec![23, 40];
        assert_eq!(query_results, answers)
    }

    #[test]
    fn test_solve_sample_2() {
        let array_len = 5 as usize;
        let operation_list: Vec<Op> = vec![
            Op::new_increment(0, -43),
            Op::new_increment(4, 1),
            Op::new_query(0),
            Op::new_query(5),
        ];
        let query_results = fast_solve(array_len, operation_list);

        let answers = vec![0, -42];
        assert_eq!(query_results, answers)
    }

    #[test]
    fn test_solve_100_ops() {
        let shared_size = 100;
        let array_len = shared_size;
        let operations_count = shared_size;

        let operation_list = generate_test_ops(array_len, operations_count);

        let query_results = fast_solve(array_len, operation_list);
        println!("{:?}", query_results);
        let answers = [-92, -92, -92, -92, -92, -92, -92, -92, -174, -94, -174, -92, -92, -142, -142, -142, 0, -92, -404, -92, -92, -278, -212, -92, -172, -172, -92, -172, -172, -172, -92, 0, -232, -276, -806, -628, -596, -800, -276, -664, -664, -276, -396, -376, -276, -954, -326, -960, -276, -696, -1096, -696, -276, -276, -884, -604, -508, -1020, -276, -276, -508, -1112, -276, -510, -926]
        .to_vec();
        assert_eq!(query_results[operations_count - 100..], answers)
    }

    fn generate_test_ops(array_len: usize, operations_count: usize) -> Vec<Op> {
        let calc_increment_index = |i: usize| (i * i) % array_len;
        let calc_increment_value = |i: i64| (i * i - 7893 * i) % array_len as i64;
        let calc_query_index =
            |i: usize| (17 * i % array_len * i % array_len * i + 209) % array_len;

        let mut operation_list: Vec<Op> = vec![];
        for i in 0..operations_count {
            match i % 17 {
                0 | 1 | 7 | 8 | 14 | 16 | 17 => {
                    operation_list.push(
                        Op::new_increment(calc_increment_index(i), calc_increment_value(i as i64))
                    );
                }
                _ => {
                    operation_list.push(
                        Op::new_query(calc_query_index(i))
                    );
                }
            }
        }
        operation_list
    }

    #[test]
    fn test_maximal_limits() {
        let shared_size = 5000000;
        let array_len = shared_size;
        let operations_count = shared_size;
        let operation_list = generate_test_ops(array_len, operations_count);

        let _query_results = fast_solve(array_len, operation_list);
        // println!("{:?}", _query_results);
        assert!(true);
    }

    #[test]
    fn test_no_operations() {
        let shared_size = 5000000;
        let array_len = shared_size;
        let operations_count = 0;
        let operation_list = generate_test_ops(array_len, operations_count);
        let query_results = fast_solve(array_len, operation_list);
        assert_eq!(query_results.len(), 0);
    }
}
