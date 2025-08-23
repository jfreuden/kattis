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
fn remove_end_incremements(operations_list: &mut Vec<Op>) {
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
}

/// Combine increment operations that will always collectively affect the first (and all following) queries
fn lump_front_increments(operations_list: &mut Vec<Op>) {
    // Essentially, all early-zoned increments between queries can be combined
    todo!()
}

fn brute_solve(array_len: usize, operations_list: Vec::<Op>) -> Vec<ValueType> {
    todo!()
}

fn fast_solve(array_len: usize, mut operations_list: Vec::<Op>) -> Vec<ValueType> {
    remove_end_incremements(&mut operations_list);
    lump_front_increments(&mut operations_list);

    todo!()
}

const SELECTED_SOLVER: fn(usize, Vec::<Op>) -> Vec<ValueType> = fast_solve;

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

    let query_results = brute_solve(array_len, operations_list);
    for result in query_results {
        println!("{}", result)
    }
}
