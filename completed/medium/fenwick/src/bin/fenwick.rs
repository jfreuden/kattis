// noinspection
#[allow(unused)]
macro_rules! import {($name:ident) => {
    #[cfg(not(feature="libfreuden"))]
    mod $name;
    #[cfg(not(feature="libfreuden"))]
    #[allow(unused_imports)]
    use $name::*;

    #[cfg(feature="libfreuden")]
    #[allow(unused_imports)]
    use libfreuden::$name::*;
    };}

import!(input);
import!(fenwick_tree);

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

fn do_fenwick_operations(fenwick: &mut FenwickTree, operations_list: Vec<Op>) -> Vec<ValueType> {
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

fn parse_bytes(bytes: &[u8]) -> (i64, usize) {
    let mut index = 0usize;
    while index < bytes.len() {
        let b = unsafe { *bytes.get_unchecked(index) };
        if b > b' ' { break; }
        index += 1;
    }
    let is_negative = if unsafe { *bytes.get_unchecked(index) } == b'-' {
        index += 1;
        true
    } else {
        false
    };
    let mut v: usize = 0;
    while index < bytes.len() {
        let b = unsafe { *bytes.get_unchecked(index) };
        if b.is_ascii_digit() {
            v = v * 10 + (b - b'0') as usize;
            index += 1;
        } else {
            break;
        }
    }
    if is_negative {
        (-(v as i64), index)
    } else {
        (v as i64, index)
    }
}

fn run_problem<R: std::io::Read, W: std::io::Write>(read_source: R, mut write_source: W) {
    let mut bufreader = std::io::BufReader::new(read_source);
    let [array_len, operations_count]: [usize; 2] = read_vec(&mut bufreader).try_into().unwrap();
    let mut operations_list = Vec::with_capacity(operations_count);

    let mut all_lines = String::new();
    std::io::Read::read_to_string(&mut bufreader, &mut all_lines).unwrap();

    let mut index = 0usize;
    let bytes = all_lines.as_bytes();
    'outer: while index < bytes.len() {
        loop {
            if index >= bytes.len() {
                break 'outer;
            }
            let b = unsafe { *bytes.get_unchecked(index) };
            if b > b' ' { break; }
            index += 1;
        }
        let opchar = unsafe { *bytes.get_unchecked(index) };
        index += 2;
        let (op_index, offset) = parse_bytes(&bytes[index..]);
        index += offset;
        if opchar == b'?' {
            operations_list.push(Op::new_query(op_index as IndexType));
        } else if opchar == b'+' {
            let (op_value, offset) = parse_bytes(&bytes[index..]);
            index += offset;
            operations_list.push(Op::new_increment(op_index as IndexType, op_value as ValueType));
        } else {
            break
        }
    }

    let mut fenwick = FenwickTree::new(array_len);
    let answers = do_fenwick_operations(&mut fenwick, operations_list);

    let charholder = convert_answers_to_printbuffer(answers);
    write_source.write_all(charholder.as_slice()).unwrap();
    write_source.flush().unwrap();
}

fn convert_answers_to_printbuffer(answers: Vec<ValueType>) -> Vec<u8> {
    let mut charholder = Vec::<u8>::with_capacity(1024 * 1024 * 64);
    for answer in answers {
        if answer.is_negative() {
            charholder.push(b'-');
        } else if answer == 0 {
            charholder.push(b'0');
            charholder.push(b'\n');
            continue;
        }

        let mut temp = answer.unsigned_abs() as IndexType;

        let digit_count = (temp.checked_ilog10().unwrap() + 1) as usize;
        let mut offset = 1usize;
        charholder.append(&mut vec![b' '; digit_count]);
        let end = charholder.len();
        while temp > 0 {
            let modulo = (temp % 10) as u8;
            temp /= 10;
            charholder[end - offset] = b'0' + modulo;
            offset += 1;
        }
        charholder.push(b'\n');
    }
    charholder
}

fn main() {
    let stdin_lock = std::io::stdin().lock();
    let stdout_lock = std::io::stdout().lock();

    run_problem(stdin_lock, stdout_lock);
}

#[cfg(test)]
mod fenwick_tests {
    use super::*;

    fn fast_solve(array_len: usize, operations_list: Vec<Op>) -> Vec<ValueType> {
        let mut fenwick = FenwickTree::new(array_len);
        do_fenwick_operations(&mut fenwick, operations_list)
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
        let calc_increment_value = |i: ValueType| (i * i - 7893 * i) % array_len as ValueType;
        let calc_query_index =
            |i: usize| (17 * i % array_len * i % array_len * i + 209) % array_len;

        let mut operation_list: Vec<Op> = vec![];
        for i in 0..operations_count {
            match i % 17 {
                0 | 1 | 7 | 8 | 14 | 16 | 17 => {
                    operation_list.push(Op::new_increment(
                        calc_increment_index(i),
                        calc_increment_value(i as ValueType),
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
            std::fs::File::create("./fenwick-test.out").unwrap(),
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
