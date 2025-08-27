fn read_vec<T: std::str::FromStr, R: std::io::Read>(
    buf_reader: &mut std::io::BufReader<R>,
) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    use std::io::BufRead;
    let mut line = String::new();
    buf_reader.read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|tok| tok.parse::<T>().expect("Failed to parse input"))
        .collect()
}

type IndexType = usize;
type ValueType = i64;

struct FenwickTree {
    data_fenwick: Vec<ValueType>,
}

impl FenwickTree {
    fn new(array_len: usize) -> Self {
        FenwickTree {
            data_fenwick: vec![0 as ValueType; array_len],
        }
    }

    fn increment(&mut self, index: usize, value: ValueType) {
        let max_index = self.data_fenwick.len();
        let mut working_index = index as ValueType + 1;

        while working_index <= max_index as i64 {
            self.data_fenwick[working_index as usize - 1] += value;
            working_index = working_index + (working_index & -working_index);
        }
    }

    fn query(&self, index: usize) -> ValueType {
        let query_index = index;
        if query_index == 0 {
            0
        } else {
            let mut sum = 0;

            let mut working_index = query_index as ValueType;
            while working_index > 0 {
                sum += self.data_fenwick[working_index as usize - 1];
                working_index = working_index - (working_index & (-working_index));
            }
            sum
        }
    }
}

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
        Op { index, value }
    }

    fn is_query(&self) -> bool {
        self.value == ValueType::MAX
    }
}

fn main() {
    let stdin_lock = std::io::stdin().lock();
    let stdout_lock = std::io::stdout().lock();

    run_problem(stdin_lock, stdout_lock);
}

fn run_problem<R: std::io::Read, W: std::io::Write>(read_source: R, write_source: W) {
    let mut bufreader = std::io::BufReader::new(read_source);
    let [array_len, operations_count]: [usize; 2] = read_vec(&mut bufreader).try_into().unwrap();
    let mut operations_list = Vec::with_capacity(operations_count);

    let mut all_lines = String::new();
    std::io::Read::read_to_string(&mut bufreader, &mut all_lines).unwrap();
    for line in all_lines.split('\n') {
        let op: Vec<&str> = line.split_ascii_whitespace().map(|tok| tok).collect();
        match op.len() {
            2 => {
                // Query Operation
                let query_index = op[1].parse::<IndexType>().unwrap();
                operations_list.push(Op::new_query(query_index));
            }
            3 => {
                // Increment Operation
                let increment_index = op[1].parse::<IndexType>().unwrap();
                let increment_value = op[2].parse::<ValueType>().unwrap();
                operations_list.push(Op::new_increment(increment_index, increment_value));
            }
            _ => break,
        }
    }

    let mut fenwick = FenwickTree::new(array_len);
    let mut answers = Vec::<ValueType>::with_capacity(operations_count);

    for op in operations_list {
        if op.is_query() {
            let answer = fenwick.query(op.index);
            answers.push(answer);
        } else {
            fenwick.increment(op.index, op.value);
        }
    }

    let starting_cap = operations_count * 64;
    let mut bufwriter = std::io::BufWriter::with_capacity(starting_cap, write_source);
    for answer in answers {
        use std::io::Write;
        writeln!(&mut bufwriter, "{}", answer).unwrap();
    }
}

#[cfg(test)]
mod fenwick_tests {
    use super::*;

    fn fast_solve(array_len: usize, operations_list: Vec<Op>) -> Vec<ValueType> {
        let mut fenwick = FenwickTree::new(array_len);

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
        let answers = [
            -92, -92, -92, -92, -92, -92, -92, -92, -174, -94, -174, -92, -92, -142, -142, -142, 0,
            -92, -404, -92, -92, -278, -212, -92, -172, -172, -92, -172, -172, -172, -92, 0, -232,
            -276, -806, -628, -596, -800, -276, -664, -664, -276, -396, -376, -276, -954, -326,
            -960, -276, -696, -1096, -696, -276, -276, -884, -604, -508, -1020, -276, -276, -508,
            -1112, -276, -510, -926,
        ]
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
                    operation_list.push(Op::new_increment(
                        calc_increment_index(i),
                        calc_increment_value(i as i64),
                    ));
                }
                _ => {
                    operation_list.push(Op::new_query(calc_query_index(i)));
                }
            }
        }
        operation_list
    }

    #[test]
    fn test_maximal_limits() {


        run_problem(
            std::fs::File::open("./fenwick_max.in").unwrap(),
            std::fs::File::create("../../../../fenwick-test.out").unwrap(),
        );
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
