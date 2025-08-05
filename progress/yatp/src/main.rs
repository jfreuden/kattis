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

/// A Bidirectional edge
#[derive(Debug, Copy, Clone)]
struct BiEdge {
    i: u64,
    j: u64,
    weight: u64,
}

impl BiEdge {
    fn new(i: u64, j: u64, weight: u64) -> BiEdge {
        Self { i, j, weight }
    }

    fn connects(&self, node: u64) -> bool {
        self.i == node || self.j == node
    }


    // TODO: INTEGRATE THIS
    fn connected_to(self, node: u64) -> Option<u64> {
        if self.i == node {
            Some(self.j)
        } else if self.j == node {
            Some(self.i)
        } else {
            None
        }
    }
}
impl std::fmt::Display for BiEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Edge({} -> {}, weight: {})", self.i, self.j, self.weight)
    }
}

impl PartialEq for BiEdge {
    fn eq(&self, other: &BiEdge) -> bool {
        self.connects(other.i) && self.connects(other.j) && self.weight == other.weight
    }
}

fn seek_path(edges: Vec<BiEdge>, from: u64, to: u64) -> Vec<BiEdge> {
    if from == to {
        return vec![]; // No path needed to get to the destination
    }

    println!("seek_path edges: {:?}, from {} to {}", edges, from, to);
    // the issue is the to/from here. I took care of the "connects" down below but it's making the trip harder than needed because of the indirectionality
    // yeah the "edge.connects(from)" is making us miss the edge
    for &edge in &edges {
        if edge.connects(from) {
            return if edge.connects(to) {
                std::iter::once(edge).collect()
            } else {
                let next_edges = edges.iter().filter(|&candidate| candidate.connects(from) && *candidate != edge).copied().collect();
                let next = if edge.i == from { edge.j } else { edge.i };
                std::iter::once(edge).chain(seek_path(next_edges, next, to)).collect()
            }
        }
    }
    edges // No path to get to the destination
}


fn calculate_cost(nodes: Vec<u64>, edges: Vec<[u64; 3]>, from: u64, to: u64) -> u64 {
    let selected_node_penalty = nodes[from as usize];
    let cheapest_path_cost = selected_node_penalty * selected_node_penalty;
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
/// (even memoization or caching would improve this implementation)
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
    fn test_seek_path() {
        let edge_weights: Vec<BiEdge> = vec![
            BiEdge::new(3, 2, 8),
            BiEdge::new(5, 2, 10),
            BiEdge::new(4, 3, 10),
            BiEdge::new(2, 1, 2),
        ];

        assert_eq!(seek_path(edge_weights.clone(), 1, 2), vec![BiEdge::new(2, 1, 2)]);
        assert_eq!(seek_path(edge_weights.clone(), 2, 1), vec![BiEdge::new(2, 1, 2)]);

    }

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

