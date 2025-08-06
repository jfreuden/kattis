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
        if i == 0 || j == 0 || weight == 0 {
            panic!("Invalid edge: ({}) - {} -> ({})", i, weight, j);
        }
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

impl From<[u64; 3]> for BiEdge {
    fn from(arr: [u64; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }
}

impl std::fmt::Display for BiEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Edge({} -> {}, weight: {})", self.i, self.j, self.weight)
    }
}

impl PartialEq for BiEdge {
    fn eq(&self, other: &BiEdge) -> bool {
        self::BiEdge::eq(&self, &other)
    }
}

impl PartialEq<&BiEdge> for BiEdge {
    fn eq(&self, other: &&BiEdge) -> bool {
        self.connects(other.i) && self.connects(other.j) && self.weight == other.weight
    }
}

fn seek_path(edges: &Vec<BiEdge>, from: u64, to: u64) -> Vec<BiEdge> {
    if from == to {
        return vec![]; // No path needed to get to the destination
    }
    for edge in edges {
        if let Some(attached) = edge.connected_to(from) {
            return if attached == to {
                std::iter::once(*edge).collect()
            } else {
                let next_edges = edges
                    .iter()
                    .filter(|&candidate| candidate != edge)
                    .cloned()
                    .collect::<Vec<BiEdge>>();
                let path = seek_path(&next_edges, attached, to);
                if path.is_empty() {
                    continue; // this branch was a dud, check the other edges
                } else {
                    std::iter::once(*edge).chain(path).collect()
                }
            };
        }
    }
    vec![] // No path to get to the destination
}

fn calculate_cost(nodes: &Vec<u64>, edges: &Vec<BiEdge>, from: u64, to: u64) -> u64 {
    let start_node_penalty = nodes[from as usize];
    let end_node_penalty = nodes[to as usize];
    let path = seek_path(&edges, from + 1, to + 1);
    let cost =
        path.iter().fold(0, |acc, edge| acc + edge.weight) + start_node_penalty * end_node_penalty;
    cost
}

/// Eschew fancy data structures and do a bad-performance computation for test verification
/// (even memoization or caching would improve this implementation)
fn brute_solve(nodes: Vec<u64>, edges: Vec<BiEdge>) -> u64 {
    let nodecount = nodes.len();
    (0..nodecount)
        .into_iter()
        .map(|i| {
            let costs_for_i = (0..nodecount)
                .into_iter()
                .map(|j| calculate_cost(&nodes, &edges, i as u64, j as u64));
            costs_for_i.min().unwrap()
        })
        .sum()
}

#[allow(dead_code)]
fn solve(nodes: Vec<u64>, edges: Vec<BiEdge>) -> u64 {
    drop(nodes);
    drop(edges);
    todo!() // TODO: Build data structures to solve this problem
}

fn main() {
    let number_nodes: usize = read_one();
    let node_penalties = read_vec::<u64>();
    let edge_weights: Vec<BiEdge> = {
        let mut out = Vec::new();
        for _ in 0..number_nodes - 1 {
            out.push(BiEdge::from(read_array()));
        }
        out
    };
    println!("{}", SELECTED_SOLVER(node_penalties, edge_weights))
}
const SELECTED_SOLVER: fn(Vec<u64>, Vec<BiEdge>) -> u64 = brute_solve;

#[cfg(test)]
mod yatp_tests {
    use super::*;

    #[test]
    fn test_edge_new() {
        let edge = BiEdge::new(3, 2, 8);
        assert_eq!(
            edge,
            BiEdge {
                i: 3,
                j: 2,
                weight: 8
            }
        );
    }

    #[test]
    fn test_edge_connects() {
        let edge = BiEdge::new(3, 2, 8);
        assert!(edge.connects(3));
        assert!(edge.connects(2));
        assert!(!edge.connects(1));
    }

    #[test]
    fn test_edge_connected_to() {
        let edge = BiEdge::new(3, 2, 8);
        assert_eq!(edge.connected_to(3), Some(2));
        assert_eq!(edge.connected_to(2), Some(3));
        assert_eq!(edge.connected_to(1), None);
    }

    #[test]
    fn test_edge_display() {
        let edge = BiEdge::new(3, 2, 8);
        assert_eq!(format!("{}", edge), "Edge(3 -> 2, weight: 8)");
    }

    #[test]
    fn test_edge_eq() {
        let edge_a = BiEdge::new(3, 2, 8);
        let edge_b = BiEdge::new(3, 2, 8);
        let edge_c = BiEdge::new(3, 2, 9);
        assert_eq!(edge_a, edge_b);
        assert_ne!(edge_a, edge_c);
    }

    #[test]
    fn test_seek_path_empty() {
        let edge_weights: Vec<BiEdge> = vec![];
        assert_eq!(seek_path(&edge_weights, 1, 2), Vec::<BiEdge>::new());
    }

    #[test]
    fn test_seek_path_single() {
        let edge_weights: Vec<BiEdge> = vec![BiEdge::new(3, 2, 8)];
        assert_eq!(seek_path(&edge_weights, 3, 2), vec![BiEdge::new(3, 2, 8)]);
    }

    #[test]
    fn test_seek_path_straight() {
        let edge_weights: Vec<BiEdge> = vec![
            BiEdge::new(1, 2, 8),
            BiEdge::new(2, 3, 2),
            BiEdge::new(3, 4, 10),
            BiEdge::new(4, 5, 10),
        ];
        assert_eq!(seek_path(&edge_weights, 1, 5), edge_weights.clone());
        assert_eq!(
            seek_path(&edge_weights, 5, 1),
            edge_weights.iter().rev().collect::<Vec<_>>()
        )
    }

    #[test]
    fn test_seek_path_a() {
        let edge_weights: Vec<BiEdge> = vec![
            BiEdge::new(3, 2, 8),
            BiEdge::new(5, 2, 10),
            BiEdge::new(4, 3, 10),
            BiEdge::new(2, 1, 2),
        ];

        assert_eq!(seek_path(&edge_weights, 1, 2), vec![BiEdge::new(2, 1, 2)]);
    }

    #[test]
    fn test_seek_path_b() {
        let edge_weights: Vec<BiEdge> = vec![
            BiEdge::new(3, 2, 8),
            BiEdge::new(5, 2, 10),
            BiEdge::new(4, 3, 10),
            BiEdge::new(2, 1, 2),
        ];

        assert_eq!(seek_path(&edge_weights, 2, 1), vec![BiEdge::new(2, 1, 2)]);
    }

    #[test]
    fn kattis_testcase() {
        let node_penalties = vec![9, 7, 1, 1, 9];
        let edge_weights: Vec<BiEdge> = vec![
            [3, 2, 8].into(),
            [5, 2, 10].into(),
            [4, 3, 10].into(),
            [2, 1, 2].into(),
        ];

        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 63);
    }

    #[test]
    fn test_solve_100_nodes() {
        let node_penalties = (0..100).collect::<Vec<u64>>();
        let edge_weights: Vec<BiEdge> = (0..99)
            .map(|i| {
                let j = (i + 1) % 100;
                [i as u64 + 1, j as u64 + 2, 1].into()
            })
            .collect();
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 1225);
    }

    #[test]
    fn test_solve_200_nodes() {
        let mut node_penalties = (0..200).collect::<Vec<u64>>();
        node_penalties.rotate_left(23);
        let edge_weights: Vec<BiEdge> = (0..199)
            .map(|i| {
                let j = (i + 1) % 200;
                [i + 1, j + 2, (i + j) % 17 + 1].into()
            })
            .collect();
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 8200);
    }

    #[test]
    fn test_expensive_path() {
        let node_penalties = std::iter::once(2)
            .chain(std::iter::repeat_n(70, 40))
            .chain(std::iter::once(2))
            .collect::<Vec<u64>>();

        let edge_weights: Vec<BiEdge> = (0..41).map(|i| [i + 1, i + 2, 100000].into()).collect();
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 196008)
    }

    #[test]
    fn test_long_cheap_path() {
        let node_penalties = std::iter::once(2)
            .chain(std::iter::repeat_n(70, 40))
            .chain(std::iter::once(2))
            .collect::<Vec<u64>>();

        let edge_weights: Vec<BiEdge> = (0..41).map(|i| [i + 1, i + 2, 1].into()).collect();
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 6028)
    }

    #[test]
    fn test_seekpath_malformed_edgelist_orphaned() {
        // The node is present in the edgelist, but it's orphaned from the other side of the route
        let edge_weights: Vec<BiEdge> = vec![
            [1, 2, 8].into(),
            [3, 2, 10].into(),
            [4, 3, 10].into(),
            [5, 6, 2].into(),
        ]; // 1 can get to 4, but no further. (not sure if I should use len or not)
        assert_eq!(seek_path(&edge_weights, 1, 6), Vec::<BiEdge>::new());
        assert_eq!(seek_path(&edge_weights, 6, 1), Vec::<BiEdge>::new());
    }

    #[test]
    fn test_seekpath_malformed_node_missing() {
        // The node doesn't appear in the edgelist whatsoever

        let edge_weights: Vec<BiEdge> = vec![
            [3, 2, 8].into(),
            [6, 2, 10].into(),
            [4, 3, 10].into(),
            [2, 1, 2].into(),
        ]; // Node 5 totally missing, but node 6 is present. (not sure if I should use len or not)
        assert_eq!(seek_path(&edge_weights, 1, 5), Vec::<BiEdge>::new());
        assert_eq!(seek_path(&edge_weights, 5, 1), Vec::<BiEdge>::new());
    }
}
