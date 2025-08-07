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
        if i == 0 || j == 0 /*|| weight == 0*/ {
            panic!("Invalid edge: ({}) - {} -> ({})", i, weight, j);
        }
        Self { i, j, weight }
    }

    fn connects(&self, node: u64) -> bool {
        self.i == node || self.j == node
    }

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

/// Returns the minimum ending path above cutoff.
fn bfs_short_circuit(edges: Vec<&BiEdge>, start_node: u64, node_count: u64, cutoff: u64) -> u64 {
    let mut working_edges = edges.clone();
    let mut pointer = start_node;
    let mut queue: std::collections::VecDeque<(u64, u64)> = std::collections::VecDeque::new();
    let mut current_cutoff = cutoff;
    let mut current_cost = 0;
    while let (adjacents, next_edges) = working_edges.iter().partition::<Vec<&BiEdge>, _>(|&edge| edge.connects(pointer)) {
        for edge in adjacents {
            let path_cost = current_cost + edge.weight;
            if path_cost > current_cutoff {
                continue
            } else if edge.i > node_count || edge.j > node_count {
                current_cutoff = std::cmp::min(current_cutoff, path_cost) // This edge is a synth under cutoff. Take it if its path is the min cost
            } else if !next_edges.is_empty(){
                // add to queue
                queue.push_front((edge.connected_to(pointer).unwrap(), path_cost));
            }
        }

        if let Some((ptr, ptr_miniumum)) = queue.pop_front() {
            pointer = ptr;
            current_cost = ptr_miniumum;
            working_edges = next_edges;
        } else {
            break
        }
    }

    current_cutoff
}

#[allow(dead_code)]
fn solve(nodes: Vec<u64>, edges: Vec<BiEdge>) -> u64 {
    // BFS with a cost short circuit, on a list of edges including a set of synth edges with weight
    let node_count = nodes.len() as u64;
    let template_synths: Vec<BiEdge> = nodes.iter().enumerate().map(|(i, penalty)| {
        let node = (i + 1) as u64;

        // later the weights will be
        //  weight = (weight - 1) * penalty
        // so I may as well subtract one in this step
        BiEdge::new(node, node_count + node, *penalty - 1)
    }).collect();


    nodes.iter().enumerate().map(|(i, penalty)| {
        let node = (i + 1) as u64;
        let mut prepped_synths = template_synths.clone();
        let synths = prepped_synths.iter_mut().map(|x| {
            x.weight *= penalty;
            x as &BiEdge
        }).chain(edges.iter()).collect(); // could just refrain from adding a synth if it's cost would go above the cutoff anyway
        let bfs_cost = bfs_short_circuit(synths, node, node_count, penalty * penalty - penalty);
        let out = penalty + bfs_cost;
        out
    }).sum()
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
const SELECTED_SOLVER: fn(Vec<u64>, Vec<BiEdge>) -> u64 = solve;


/// Converts a set of n nodes with penalties into additional synthetic weighted edges [n to 2n-1]
/// The edges are moved into the return vector
fn convert_penalties_to_edges(nodes: &Vec<u64>, edges: Vec<BiEdge>) -> Vec<BiEdge> {
    let mut out = edges;
    let mut synths = nodes.iter().enumerate().map(|(i, x)| {
        let node = (i + 1) as u64;
        let synth = nodes.len() as u64 + node;
        let weight = *x;
        BiEdge::new(node, synth, weight)
    }).collect();
    out.append(&mut synths);
    out
}


fn seek_path(edges: &Vec<BiEdge>, from: u64, to: u64) -> Vec<BiEdge> {
    if from == to {
        return edges.iter().find(|&edge| edge.connected_to(from) == Some(to)).copied().into_iter().collect::<Vec<BiEdge>>(); // No path needed to get to the destination
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

#[allow(dead_code)]
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
#[allow(dead_code)]
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

#[cfg(test)]
mod yatp_tests {
    use super::*;

    #[test]
    fn test_convert_penalties_to_edges() {
        let edge_weights: Vec<BiEdge> = vec![
            BiEdge::new(1, 2, 8),
            BiEdge::new(2, 3, 2),
            BiEdge::new(3, 4, 10),
            BiEdge::new(4, 5, 10),
        ];
        let nodes: Vec<u64> = vec![10, 20, 30, 40, 50];

        let synths = convert_penalties_to_edges(&nodes, edge_weights);

        assert_eq!(synths, vec![
            BiEdge::new(1, 2, 8),
            BiEdge::new(2, 3, 2),
            BiEdge::new(3, 4, 10),
            BiEdge::new(4, 5, 10),
            // append old nodes with connections to synths
            BiEdge::new(1, 5 + 1, 10),
            BiEdge::new(2, 5 + 2, 20),
            BiEdge::new(3, 5 + 3, 30),
            BiEdge::new(4, 5 + 4, 40),
            BiEdge::new(5, 5 + 5, 50),
            // append reflexive edges (TODO: are these necessary if the n=1 -> n+1 mapping has the weight already?)
        ]);
    }

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
        let node_penalties = (1..101).collect::<Vec<u64>>();
        let edge_weights: Vec<BiEdge> = (0..99)
            .map(|i| {
                let j = i % 100;
                [i + 1, j + 2, 1].into()
            })
            .collect();
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 10000);
    }

    #[test]
    fn test_optsolve_100_nodes() {
        let node_count = 100;
        let node_start = 1;

        let node_penalties = (node_start..node_start + node_count).collect::<Vec<u64>>();
        let edge_weights: Vec<BiEdge> = (0..node_count - 1)
            .map(|i| {
                let j = i % node_count;
                [i + node_start, j + node_start + 1, 1].into()
            })
            .collect();
        assert_eq!(solve(node_penalties, edge_weights), 10000);
    }

    #[test]
    fn test_optsolve_50_nodes() {
        let node_count = 50;
        let node_start = 1;
        let mut node_penalties = (node_start..node_start + node_count).collect::<Vec<u64>>();
        node_penalties.rotate_left(11);
        let edge_weights: Vec<BiEdge> = (0..node_count - 1)
            .map(|i| {
                let j = i % node_count;
                [i + node_start, j + node_start + 1, (i + j) % 29 + node_start].into()
            })
            .collect();
        assert_eq!(solve(node_penalties, edge_weights), 9697);
    }

    #[test]
    fn test_optsolve_500_nodes() {
        let node_count = 500;
        let node_start = 1;
        let mut node_penalties = (node_start..node_start + node_count).collect::<Vec<u64>>();
        node_penalties.rotate_left(77);
        let edge_weights: Vec<BiEdge> = (0..node_count - 1)
            .map(|i| {
                let j = i % node_count;
                [i + node_start, j + node_start + 1, (i + j) % 217 + node_start].into()
            })
            .collect();
        assert_eq!(solve(node_penalties, edge_weights), 7296625);
    }

    #[test]
    fn test_optsolve_1000_nodes() {
        let node_count = 1000;
        let node_start = 1;
        let mut node_penalties = (node_start..node_start + node_count).collect::<Vec<u64>>();
        node_penalties.rotate_left(97);
        let edge_weights: Vec<BiEdge> = (0..node_count - 1)
            .map(|i| {
                let j = i % node_count;
                [i + node_start, j + node_start + 1, (i + j) % 517 + node_start].into()
            })
            .collect();
        assert_eq!(solve(node_penalties, edge_weights), 58466345);
    }

    #[test]
    fn test_optsolve_2000_nodes() {
        let node_count = 2000;
        let node_start = 1;
        let mut node_penalties = (node_start..node_start + node_count).collect::<Vec<u64>>();
        node_penalties.rotate_left(939);
        let edge_weights: Vec<BiEdge> = (0..node_count - 1)
            .map(|i| {
                let j = i % node_count;
                [i + node_start, j + node_start + 1, (i + j) % 1217 + node_start].into()
            })
            .collect();
        assert_eq!(solve(node_penalties, edge_weights), 548823761);
    }

    #[test]
    fn test_optsolve_10000_nodes() {
        let node_count = 10000;
        let node_start = 1;
        let mut node_penalties = (node_start..node_start + node_count).collect::<Vec<u64>>();
        node_penalties.rotate_left(2839);
        let edge_weights: Vec<BiEdge> = (0..node_count - 1)
            .map(|i| {
                let j = i % node_count;
                [i + node_start, j + node_start + 1, (i + j) % 4117 + node_start].into()
            })
            .collect();
        assert_eq!(solve(node_penalties, edge_weights), 0);
    }

    // #[test]
    fn test_optsolve_50000_nodes() {
        let node_count = 50000;
        let node_start = 1;
        let mut node_penalties = (node_start..node_start + node_count).collect::<Vec<u64>>();
        node_penalties.rotate_left(2339);
        let edge_weights: Vec<BiEdge> = (0..node_count - 1)
            .map(|i| {
                let j = i % node_count;
                [i + node_start, j + node_start + 1, (i + j) % 1117 + node_start].into()
            })
            .collect();
        assert_eq!(solve(node_penalties, edge_weights), 0);
    }

    // #[test]
    fn test_optsolve_100000_nodes() {
        let node_count = 100000;
        let node_start = 1;
        let mut node_penalties = (node_start..node_start + node_count).collect::<Vec<u64>>();
        node_penalties.rotate_left(23789);
        let edge_weights: Vec<BiEdge> = (0..node_count - 1)
            .map(|i| {
                let j = i % node_count;
                [i + node_start, j + node_start + 1, (i + j) % 127 + node_start].into()
            })
            .collect();
        assert_eq!(solve(node_penalties, edge_weights), 0);
    }


    // #[test]
    fn test_optsolve_200000_nodes() {
        let node_count = 200000;
        let node_start = 1;
        let mut node_penalties = (node_start..node_start + node_count).collect::<Vec<u64>>();
        node_penalties.rotate_left(23789);
        let edge_weights: Vec<BiEdge> = (0..node_count - 1)
            .map(|i| {
                let j = i % node_count;
                [i + node_start, j + node_start + 1, (i + j) % 17 + node_start].into()
            })
            .collect();
        assert_eq!(solve(node_penalties, edge_weights), 0);
    }

    #[test]
    fn test_solve_200_nodes() {
        let mut node_penalties = (1..201).collect::<Vec<u64>>();
        node_penalties.rotate_left(23);
        let edge_weights: Vec<BiEdge> = (0..199)
            .map(|i| {
                let j = i % 200;
                [i + 1, j + 2, (i + j) % 17 + 1].into()
            })
            .collect();
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 149656);
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
