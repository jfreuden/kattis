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
    index: IndexType,
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
    let trashed_increments: Vec<Op> = operations_list
        .extract_if(.., |op| {
            if let Some(&IncrementOp { index, value: _ }) =
                op.as_any().downcast_ref::<IncrementOp>()
            {
                if index > highest_index {
                    return true;
                }
            }
            false
        })
        .collect();
    eprintln!(
        "Removed {} end increments that will never be queried.",
        trashed_increments.len()
    );
}

/// Combine increment operations that will always collectively affect the first (and all following) queries
fn lump_front_increments(operations_list: &mut Vec<Op>) {
    // Essentially, all early-zoned increments between queries can be combined
    todo!()
}

fn brute_solve(array_len: usize, operations_list: Vec<Op>) -> Vec<ValueType> {
    let mut dumb_sum_array = vec![ValueType::default(); array_len];
    let mut query_answers = Vec::<ValueType>::new();

    let op_count = operations_list.len();
    let batch_len = 10;

    for (batch, op_batch) in operations_list.chunks(batch_len).enumerate() {
        for op in op_batch {
            if let Some(query) = op.as_any().downcast_ref::<QueryOp>() {
                let answer = if query.index == 0 {
                    0
                } else {
                    dumb_sum_array[query.index - 1]
                };
                query_answers.push(answer);
            } else if let Some(increment) = op.as_any().downcast_ref::<IncrementOp>() {
                for val in dumb_sum_array[increment.index..].iter_mut() {
                    *val += increment.value;
                }
            }
        }
        eprintln!(
            "Completed {} / {}",
            batch * batch_len + op_batch.len(),
            op_count
        );
    }
    query_answers
}

fn get_optype_counts(ops: &Vec<Op>) -> (usize, usize) {
    let mut increment_count = 0;
    let mut query_count = 0;
    for op in ops {
        if op.as_any().downcast_ref::<QueryOp>().is_some() {
            query_count += 1;
        } else if let Some(increment) = op.as_any().downcast_ref::<IncrementOp>() {
            increment_count += 1;
        }
    }
    (increment_count, query_count)
}

fn fast_solve(array_len: usize, mut operations_list: Vec<Op>) -> Vec<ValueType> {
    remove_end_increments(&mut operations_list);
    lump_front_increments(&mut operations_list);

    let operations_count = operations_list.len();
    let (increment_count, query_count) = get_optype_counts(&operations_list);
    let mut dependent_increments_lists = Vec::<Vec<IncrementOp>>::with_capacity(increment_count);
    let mut queries = Vec::<QueryOp>::with_capacity(query_count);

    let mut dependent_increments = Vec::<IncrementOp>::new();
    // todo: above was the "let's organize all the dependent increments to go before the relevant query" work.

    let mut data_fenwick = vec![0 as ValueType; array_len];

    for op in operations_list {
        if let Some(query) = op.as_any().downcast_ref::<QueryOp>() {
            let answer = if query.index == 0 {
                0
            } else {
                let mut part_sum = 0;
                // todo: its not o(1) lookup. write the index processing methods.

                data_fenwick[query.index - 1]
            };
        } else if let Some(increment) = op.as_any().downcast_ref::<IncrementOp>() {
        }
    }

    todo!()
}

const SELECTED_SOLVER: fn(usize, Vec<Op>) -> Vec<ValueType> = brute_solve;

fn main() {
    let [array_len, operations_count]: [usize; 2] = read_array();
    let mut operations_list = Vec::<Op>::with_capacity(operations_count);
    for _ in 0..operations_count {
        let op = read_vec::<String>();
        match op.len() {
            2 => {
                // Query Operation
                let [key, index]: [String; 2] = op.try_into().unwrap();
                if key != "?" {
                    panic!("Invalid operation")
                }
                operations_list.push(Box::new(QueryOp {
                    index: index.parse::<IndexType>().unwrap(),
                }));
            }
            3 => {
                // Increment Operation
                let [key, index, delta]: [String; 3] = op.try_into().unwrap();
                if key != "+" {
                    panic!("Invalid operation")
                }
                operations_list.push(Box::new(IncrementOp {
                    index: index.parse::<IndexType>().unwrap(),
                    value: delta.parse::<ValueType>().unwrap(),
                }))
            }
            _ => panic!("Invalid operation"),
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
            Box::new(IncrementOp {
                index: 7,
                value: 23,
            }),
            Box::new(QueryOp { index: 8 }),
            Box::new(IncrementOp {
                index: 3,
                value: 17,
            }),
            Box::new(QueryOp { index: 8 }),
        ];
        let query_results = SELECTED_SOLVER(array_len, operation_list);

        let answers = vec![23, 40];
        assert_eq!(query_results, answers)
    }

    #[test]
    fn test_solve_sample_2() {
        let array_len = 5 as usize;
        let operation_list: Vec<Op> = vec![
            Box::new(IncrementOp {
                index: 0,
                value: -43,
            }),
            Box::new(IncrementOp { index: 4, value: 1 }),
            Box::new(QueryOp { index: 0 }),
            Box::new(QueryOp { index: 5 }),
        ];
        let query_results = SELECTED_SOLVER(array_len, operation_list);

        let answers = vec![0, -42];
        assert_eq!(query_results, answers)
    }

    #[test]
    fn test_solve_maximal_limits() {
        let shared_size = 5000000;
        let array_len = shared_size;
        let operations_count = shared_size;

        let calc_increment_index = |i: usize| (i * i) % array_len;
        let calc_increment_value = |i: i64| (i * i - 7893 * i) % array_len as i64;
        let calc_query_index =
            |i: usize| (17 * i % array_len * i % array_len * i + 209) % array_len;

        let mut operation_list: Vec<Op> = vec![];
        for i in 0..operations_count {
            match i % 17 {
                0 | 1 | 7 | 8 | 14 | 16 | 17 => {
                    operation_list.push(Box::new(IncrementOp {
                        index: calc_increment_index(i),
                        value: calc_increment_value(i as i64),
                    }));
                }
                _ => {
                    operation_list.push(Box::new(QueryOp {
                        index: calc_query_index(i),
                    }));
                }
            }
        }

        let query_results = SELECTED_SOLVER(array_len, operation_list);
        println!("{:?}", query_results);
        let answers = [
            2562526284382,
            3621678267650,
            267364417242,
            1294683465160,
            3312346904256,
            2793920088052,
            3731956426586,
            257918645348,
            1167691769248,
            2064155451020,
            287882189922,
            1134921427868,
            1968572114670,
            2794132214400,
            3606647800468,
            794667534770,
            3827969787644,
            159511411788,
            875892949814,
            1581611597054,
            2278437897064,
            4295726128694,
            551250654474,
            1191890890870,
            1821003732792,
            2440308179650,
            3650046532060,
            1541221867048,
            2087936518386,
            2625939010564,
            3154855570510,
            3675461291350,
            774154807902,
            1254582368302,
            1724477327402,
            2185966728414,
            2639132151604,
            3519107184884,
            776462482756,
            1169305538478,
            1552030633046,
            1926406037888,
            2295447356890,
            3348756495774,
            3686547186792,
            4014602427816,
            4331873895986,
            250171691596,
            851649662352,
            1969898135102,
            2233132305510,
            2489221119562,
            2738969077336,
            2981139017630,
            3671047154496,
            3887609521614,
            4096416543844,
            4298711911872,
            102326962590,
            479073403636,
            1171038636864,
            1329013086964,
            1482990047132,
            1629970422092,
            1773051493416,
            2172150703016,
            2297666972436,
            2416369498628,
            2529815528996,
            2639127173660,
            2847868783156,
            3210665488864,
            3291132903924,
            3369129274276,
            3442947409494,
            3512071179528,
            3704172477196,
            3760408280050,
            3813867245164,
            3863829209922,
            3912108529410,
            3998247693190,
            4136399382978,
            4165721408566,
            4192294007674,
            4216931579006,
            4239633902936,
            4295712665662,
            4311282477824,
            4324888491890,
            4337297471766,
            4347735758038,
            4365429899492,
            4387055348436,
            4390207631994,
            4392752161222,
            4394557676000,
            4395794700624,
            1014107192,
            1139759910,
        ]
        .to_vec();
        assert_eq!(query_results[operations_count - 100..], answers)
    }
}
