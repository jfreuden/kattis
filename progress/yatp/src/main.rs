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
    i: u32,
    j: u32,
    weight: u64,
}

impl BiEdge {
    fn new(i: u64, j: u64, weight: u64) -> BiEdge {
        if i == 0 || j == 0
        /*|| weight == 0*/
        {
            panic!("Invalid edge: ({}) - {} -> ({})", i, weight, j);
        }
        Self {
            i: i as u32,
            j: j as u32,
            weight,
        }
    }

    #[inline(always)]
    fn connects(&self, node: u32) -> bool {
        self.i == node || self.j == node
    }

    #[inline(always)]
    fn connected_to(self, node: u32) -> Option<u32> {
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
    #[inline(always)]
    fn eq(&self, other: &BiEdge) -> bool {
        self::BiEdge::eq(&self, &other)
    }
}

impl PartialEq<&BiEdge> for BiEdge {
    #[inline(always)]
    fn eq(&self, other: &&BiEdge) -> bool {
        self.connects(other.i) && self.connects(other.j) && self.weight == other.weight
    }
}

#[derive(Debug)]
struct EdgeCache {
    edges: std::collections::HashMap<u32, Vec<BiEdge>>,
    plucked: std::collections::HashSet<u32>,
    nodes: Vec<u64>,
}
impl EdgeCache {
    fn new(edgelist: Vec<&'_ BiEdge>, static_nodes: &'_ Vec<u64>) -> Self {
        let nodes = static_nodes.clone();
        let node_count = nodes.len() as u64;
        let mut edges = std::collections::HashMap::with_capacity((2 * node_count) as usize);
        let plucked = std::collections::HashSet::with_capacity((2 * node_count) as usize);
        for edge in edgelist {
            edges.entry(edge.i).or_insert_with(Vec::new).push(*edge);
            edges.entry(edge.j).or_insert_with(Vec::new).push(*edge);
        }

        let penalty = nodes[0];
        for (&j, edgelist) in edges.iter_mut() {
            edgelist.push(
                BiEdge::new(
                    j as u64,
                    node_count + j as u64,
                    (nodes[(j - 1) as usize]) * penalty - penalty)
            );
        }

        EdgeCache { edges, plucked, nodes }
    }

    fn reset_for(&mut self, node: u32) {
        self.plucked.clear();
        let penalty = self.nodes[(node - 1) as usize];
        for (j, edgelist) in self.edges.iter_mut() {
            let mut edit_me = edgelist.pop().unwrap();
            edit_me.weight = self.nodes[(j - 1) as usize] * penalty - penalty;
            edgelist.push(edit_me);
        }
    }

    /// Returns the entire entry of all edges in a bucket, removing it from the cache
    #[inline(always)]
    fn pluck(&mut self, node: u32) -> Vec<BiEdge> {
        if (self.plucked.insert(node)) {
            self.edges.get(&node).unwrap().clone()
        } else {
            Vec::new()
        }
    }

    #[inline(always)]
    fn contains(&self, node: u32) -> bool {
        !self.plucked.contains(&node)
    }
}

/// Returns the minimum ending path above cutoff.
fn bfs_short_circuit(
    edge_cache: &mut EdgeCache,
    start_node: u64,
    node_count: u32,
    cutoff: u64,
) -> u32 {
    let mut pointer: u32 = start_node as u32;
    let mut queue: std::collections::VecDeque<(u32, u32)> = std::collections::VecDeque::new();
    let mut current_cutoff: u32 = cutoff as u32;
    let mut current_cost: u32 = 0;

    loop {
        let adjacents = edge_cache.pluck(pointer);

        for edge in adjacents {
            let path_cost = current_cost + edge.weight as u32;
            if path_cost > current_cutoff {
                continue;
            } else if edge.i > node_count || edge.j > node_count {
                current_cutoff = std::cmp::min(current_cutoff, path_cost) // This edge is a synth under cutoff. Take it if its path is the min cost
            } else if let Some(attached) = edge.connected_to(pointer) {
                // add to queue
                if edge_cache.contains(attached) {
                    queue.push_back((attached, path_cost));
                }
            }
        }

        if let Some((ptr, ptr_minimum)) = queue.pop_front() {
            pointer = ptr;
            current_cost = ptr_minimum
        } else {
            break;
        }
    }

    current_cutoff
}

fn solve(nodes: Vec<u64>, edges: Vec<BiEdge>) -> u64 {
    // BFS with a cost short circuit, on a list of edges including a set of synth edges with weight
    let node_count = nodes.len() as u64;
    let mut edge_cache = EdgeCache::new(edges.iter().collect(), &nodes);

    nodes
        .iter()
        .enumerate()
        .map(|(i, penalty)| {
            let node = (i + 1) as u32;
            edge_cache.reset_for(node);

            let bfs_cost = bfs_short_circuit(
                &mut edge_cache,
                node as u64,
                node_count as u32,
                penalty * penalty - penalty,
            );
            let out = penalty + bfs_cost as u64;
            out
        })
        .sum()
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
                [
                    i + node_start,
                    j + node_start + 1,
                    (i + j) % 29 + node_start,
                ]
                .into()
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
                [
                    i + node_start,
                    j + node_start + 1,
                    (i + j) % 217 + node_start,
                ]
                .into()
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
                [
                    i + node_start,
                    j + node_start + 1,
                    (i + j) % 517 + node_start,
                ]
                .into()
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
                [
                    i + node_start,
                    j + node_start + 1,
                    (i + j) % 1217 + node_start,
                ]
                .into()
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
                [
                    i + node_start,
                    j + node_start + 1,
                    (i + j) % 4117 + node_start,
                ]
                .into()
            })
            .collect();
        assert_eq!(solve(node_penalties, edge_weights), 52691143621);
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
                [
                    i + node_start,
                    j + node_start + 1,
                    (i + j) % 1117 + node_start,
                ]
                .into()
            })
            .collect();
        assert_eq!(solve(node_penalties, edge_weights), 571803609907);
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
                [
                    i + node_start,
                    j + node_start + 1,
                    (i + j) % 127 + node_start,
                ]
                .into()
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
                [
                    i + node_start,
                    j + node_start + 1,
                    (i + j) % 17 + node_start,
                ]
                .into()
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

    // #[test]
    // fn test_edge_cache() {
    //     let edge_weights: Vec<BiEdge> = vec![
    //         BiEdge::new(3, 2, 8),
    //         BiEdge::new(5, 2, 10),
    //         BiEdge::new(4, 3, 10),
    //         BiEdge::new(2, 1, 2),
    //     ];
    //
    //     let mut cache = EdgeCache::new(edge_weights.iter().collect());
    //
    //     assert_eq!(cache.pluck(1), vec![&BiEdge::new(2, 1, 2)]);
    //     assert_eq!(cache.pluck(2).contains(&&BiEdge::new(3, 2, 8)), true);
    // }
}
