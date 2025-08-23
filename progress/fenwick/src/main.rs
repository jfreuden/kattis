fn read_one<T: std::str::FromStr>() -> T
where
    T::Err: std::fmt::Debug,
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<T>().unwrap()
}

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

struct IncrementOp {
    index: IndexType,
    value: ValueType,
}

struct QueryOp {
    index: IndexType
}

trait ProblemOperation: std::any::Any {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
impl ProblemOperation for IncrementOp {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
impl ProblemOperation for QueryOp {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

type Op = Box<dyn ProblemOperation>;

/// Remove any increment operation that will never be queried
/// Since the query is a prefix-sum, this amounts to:
/// "Remove any increment operation with index greater than the highest query index"
fn remove_end_increments(operations_list: &mut Vec<Op>) {
    // Find highest query
    let mut highest_index = usize::MAX;
    for op in operations_list.iter() {
        if let Some(query) = op.as_any().downcast_ref::<QueryOp>() {
            highest_index = std::cmp::max(highest_index, query.index);
        }
    }
    // extractif any increments
    let trashed_increments: Vec<Op> = operations_list.extract_if(.., |op| {
        if let Some(increment) = op.as_any().downcast_ref::<IncrementOp>() {
            if increment.index > highest_index {
                return true
            }
        }
        false
    }).collect();
    eprintln!("Removed {} end increments that will never be queried.", trashed_increments.len());
}

/// Combine increment operations that will always collectively affect the first (and all following) queries
fn lump_front_increments(operations_list: &mut Vec<Op>) {
    // Essentially, all early-zoned increments between queries can be combined
    todo!()
}

fn brute_solve(array_len: usize, operations_list: Vec::<Op>) -> Vec<ValueType> {
    let mut dumb_array = vec![ValueType::default(); array_len];
    let mut query_answers = Vec::<ValueType>::new();
    for op in operations_list {
        if let Some(query) = op.as_any().downcast_ref::<QueryOp>() {
            let answer = dumb_array.iter().take(query.index).sum();
            query_answers.push(answer);
        } else if let Some(increment) = op.as_any().downcast_ref::<IncrementOp>() {
            dumb_array[increment.index] += increment.value;
        }
    }
    query_answers
}

fn fast_solve(array_len: usize, mut operations_list: Vec::<Op>) -> Vec<ValueType> {
    remove_end_increments(&mut operations_list);
    lump_front_increments(&mut operations_list);

    todo!()
}

const SELECTED_SOLVER: fn(usize, Vec::<Op>) -> Vec<ValueType> = brute_solve;

fn main() {
    let [array_len, operations_count]: [usize; 2] = read_array();
    let mut operations_list = Vec::<Op>::with_capacity(operations_count);
    for _ in 0..operations_count {
        let op = read_vec::<String>();
        match op.len() {
            2 => { // Query Operation
                let [key, index]: [String; 2] = op.try_into().unwrap();
                if key != "?" {
                    panic!("Invalid operation")
                }
                operations_list.push(Box::new(QueryOp { index: index.parse::<IndexType>().unwrap() }));
            },
            3 => { // Increment Operation
                let [key, index, delta]: [String; 3] = op.try_into().unwrap();
                if key != "+" {
                    panic!("Invalid operation")
                }
                operations_list.push(Box::new(IncrementOp { index: index.parse::<IndexType>().unwrap(), value: delta.parse::<ValueType>().unwrap()}))
            },
            _ => panic!("Invalid operation")
        }
    }

    let query_results = SELECTED_SOLVER(array_len, operations_list);
    for result in query_results {
        println!("{}", result)
    }
}

#[cfg(test)]
mod fenwick_tests {
    use super::*;

    #[test]
    fn test_solve_sample_1() {
        let array_len = 10 as usize;
        let operation_list: Vec<Op> = vec![
            Box::new(IncrementOp { index: 7, value: 23 }),
            Box::new(QueryOp { index: 8 }),
            Box::new(IncrementOp { index: 3, value: 17 }),
            Box::new( QueryOp { index: 8 }),
        ];
        let query_results = SELECTED_SOLVER(array_len, operation_list);
        for result in query_results {
            println!("{}", result)
        }
    }

    #[test]
    fn test_solve_sample_2() {
        let array_len = 5 as usize;
        let operation_list: Vec<Op> = vec![
            Box::new(IncrementOp { index: 0, value: -43 }),
            Box::new(IncrementOp { index: 4, value: 1 }),
            Box::new(QueryOp { index: 0 }),
            Box::new( QueryOp { index: 5 }),
        ];
        let query_results = SELECTED_SOLVER(array_len, operation_list);
        for result in query_results {
            println!("{}", result)
        }
    }
}