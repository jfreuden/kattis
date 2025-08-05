fn read_str() -> String {
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");
    response.trim_end().to_string()
}

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

/// Kattis YATP (Medium - 4.0)
/// This is Yet Another Tree Problem.
/// You are given a tree, where every node has a penalty and every edge has a weight.
/// The cost of a simple path between any two nodes is the sum of:
///  - the weights of the edges in the path
///  - the product of the penalties of the endpoint nodes.
/// Note that a path can have 0 edges, and the cost of such a path is simply the square of the penalty of the node.
///
/// For each node, compute the smallest cost of any path starting at that node.
/// The final answer is the sum of all of these minimum costs.

fn calculate_cost(nodes: Vec<u64>, edges: Vec<[u64; 3]>, from: u64, to: u64) -> u64 {
    let selected_node_penalty = nodes[from as usize];
    let current_node = Some(from);
    while current_node.is_some() {
        let next_nodes = edges.iter().fold(vec![], |mut connections, edge| {
            let &[a, b, _weight] = edge.try_into().unwrap();
            if a == current_node.unwrap() {
                connections.push(b)
            } else if b == current_node.unwrap() {
                connections.push(a)
            }
            connections
        });

        // BFS towards the target, once found, unwind the relevant edges (rework the below)

        let relevant_edges = edges.iter().filter(|&edge| {
            let &[a, b, _weight] = edge.try_into().unwrap();
            a == current_node.unwrap() || b == current_node.unwrap()
        });

    }

    todo!()
}

/// Eschew fancy data structures and do a bad-performance computation for test verification
fn brute_solve(nodes: Vec<u64>, edges: Vec<[u64; 3]>) -> u64   {
    // for node in nodes {
    //     let mut cost = node * node;
    //     for edge in edges {
    //         let this_cost = node + edge;
    //     }
    // }
    todo!()
}

fn solve(nodes: Vec<u64>, edges: Vec<[u64; 3]>) -> u64 {
    todo!()
}

fn main() {
    let number_nodes: usize = read_one();
    let node_penalties = read_vec::<u64>();
    let edge_weights: Vec<[u64; 3]> = {
        let mut out = Vec::new();
        for _ in 0..number_nodes - 1 {
            out.push(read_array());
        }
        out
    };
    println!("{}", SELECTED_SOLVER(node_penalties, edge_weights))
}
const SELECTED_SOLVER: fn(Vec<u64>, Vec<[u64; 3]>) -> u64 = solve;

#[cfg(test)]
mod yatp_tests {
    use super::*;

    #[test]
    fn test_cost_stay_at_home() {
        todo!();

        {
            let cheapnode = ();
            let pricynode_a = ();
            let pricynode_b = ();
            let cheapedges = ();
            // todo: assert that cost of travelling to other pricy node is too high.
        }
        {
            let cheapnode = ();
            let othercheap_a = ();
            let othercheap_b = ();
            let pricyedges = ();
            // todo: assert that the cost of travelling the pricy edges is too high.
        }

    }

    #[test]
    fn kattis_testcase() {
        let node_penalties = vec![
            9, 7, 1, 1, 9
        ];
        let edge_weights = vec![
            [3, 2, 8],
            [5, 2, 10],
            [4, 3, 10],
            [2, 1, 2],
        ];

        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 63);
    }
}

