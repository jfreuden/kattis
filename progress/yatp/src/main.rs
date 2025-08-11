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

type NodeType = u32;
type WeightType = u64;

/// A Bidirectional edge
#[derive(Debug, Copy, Clone, Eq)]
struct BiEdge {
    i: NodeType,
    j: NodeType,
    weight: WeightType,
}

impl BiEdge {
    fn new(i: NodeType, j: NodeType, weight: WeightType) -> BiEdge {
        if i == 0 || j == 0
        /*|| weight == 0*/
        {
            panic!("Invalid edge: ({}) - {} -> ({})", i, weight, j);
        }
        Self { i, j, weight }
    }

    #[inline(always)]
    fn connects(&self, node: NodeType) -> bool {
        self.i == node || self.j == node
    }

    #[inline(always)]
    fn connected_to(self, node: NodeType) -> Option<NodeType> {
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
        Self::new(arr[0] as NodeType, arr[1] as NodeType, arr[2] as WeightType)
    }
}

impl From<[u32; 3]> for BiEdge {
    fn from(arr: [u32; 3]) -> Self {
        Self::new(arr[0] as NodeType, arr[1] as NodeType, arr[2] as WeightType)
    }
}

impl From<[i32; 3]> for BiEdge {
    fn from(arr: [i32; 3]) -> Self {
        Self::new(arr[0] as NodeType, arr[1] as NodeType, arr[2] as WeightType)
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
    node_edges: Vec<Vec<BiEdge>>,
    plucked: Vec<bool>,
    nodes: Vec<WeightType>,
}
impl EdgeCache {
    fn new(edgelist: Vec<BiEdge>, static_nodes: &Vec<WeightType>) -> Self {
        let nodes = static_nodes.clone();
        let node_count = nodes.len();
        let mut node_edges = vec![Vec::<BiEdge>::new(); node_count * 2];

        let mut enriched_edgelist = edgelist;
        for j in 0..node_count {
            enriched_edgelist.push(BiEdge::new(
                (j + 1) as NodeType,
                (j + node_count + 1) as NodeType,
                0,
            ));
        }

        let plucked = vec![false; 2 * node_count];
        for edge in &enriched_edgelist {
            // for edge in get_edge_hierarchy(&enriched_edgelist)
            //     .iter()
            //     .rev()
            //     .flatten()
            // {
            node_edges[(edge.i - 1) as usize].push(*edge);
            node_edges[(edge.j - 1) as usize].push(*edge);
        }

        EdgeCache {
            node_edges,
            plucked,
            nodes,
        }
    }

    fn reset_for(&mut self, _node: NodeType) {
        self.plucked = vec![false; self.plucked.len()];
    }

    /// Returns the entire entry of all edges in a bucket, removing it from the cache
    #[inline(always)]
    fn pluck(&mut self, node: NodeType) -> (&Vec<BiEdge>, &Self) {
        let index = (node - 1) as usize;
        let plucked_entry = &mut self.plucked[index];
        if !*plucked_entry {
            *plucked_entry = true;
            (&self.node_edges[index], self)
        } else {
            (self.node_edges.last().unwrap(), self) // HACK: have to return an empty list, don't have one.
        }
    }

    #[inline(always)]
    fn contains(&self, node: NodeType) -> bool {
        let index = (node - 1) as usize;
        !self.plucked[index]
    }
}

/// Returns the minimum ending path above cutoff.
fn bfs_short_circuit(
    edge_cache: &mut EdgeCache,
    start_node: NodeType,
    node_count: NodeType,
    cost_caps: &mut [WeightType],
) -> WeightType {
    let start_index = (start_node - 1) as usize;
    let start_penalty = edge_cache.nodes[start_index];
    let mut pointer: NodeType = start_node as NodeType;
    let mut queue: std::collections::VecDeque<(NodeType, WeightType)> =
        std::collections::VecDeque::with_capacity(node_count as usize);
    let mut current_cutoff: WeightType = cost_caps[start_index];
    let mut current_cost: WeightType = 0;

    loop {
        let (adjacents, edge_cache) = edge_cache.pluck(pointer);
        for &edge in adjacents {
            let path_cost = current_cost + edge.weight;
            if edge.j > node_count {
                current_cutoff =
                    compute_new_cutoff(start_penalty, current_cutoff, edge_cache, edge, path_cost);
                // cost_caps[index_i] = std::cmp::min(cost_caps[index_i], full_path_cost);
            } else if let Some(attached) = edge.connected_to(pointer) {
                // add to queue
                if edge_cache.contains(attached) {
                    queue.push_back((attached, path_cost));
                }
            }
        }

        loop {
            if let Some(next_please) = queue.pop_front() {
                (pointer, current_cost) = next_please;
                if current_cost >= current_cutoff {
                    continue;
                } else {
                    break;
                }
            }
            return current_cutoff;
        }
    }
}

#[inline(always)]
fn compute_new_cutoff(
    start_penalty: WeightType,
    current_cutoff: WeightType,
    edge_cache: &EdgeCache,
    edge: BiEdge,
    path_cost: WeightType,
) -> WeightType {
    let index_i = (edge.i - 1) as usize;
    let end_penalty = edge_cache.nodes[index_i];
    let full_path_cost = path_cost + start_penalty * end_penalty;
    let out = std::cmp::min(current_cutoff, full_path_cost);
    out
}

fn solve(nodes: Vec<WeightType>, edges: Vec<BiEdge>) -> u64 {
    // BFS with a cost short circuit, on a list of edges including a set of synth edges with weight
    let node_count = nodes.len() as NodeType;
    let mut cost_caps: Vec<WeightType> = nodes.iter().map(|&x| x * x).collect();

    // let mut nodelist = get_nodes_in_hierarchy_order(&edges);
    // nodelist.reverse();
    let mut edge_cache = EdgeCache::new(edges, &nodes);
    (0..cost_caps.len())
        .map(|i| {
            let node = (i + 1) as NodeType;
            edge_cache.reset_for(node);

            let bfs_cost = bfs_short_circuit(&mut edge_cache, node, node_count, &mut cost_caps);
            // println!("cost_caps sum: {}", cost_caps.iter().sum::<WeightType>());
            bfs_cost as u64
        })
        .sum()
}

fn get_nodes_in_hierarchy_order(edge_weights: &Vec<BiEdge>) -> Vec<NodeType> {
    let node_count: usize = edge_weights.len() + 1;
    let mut working_edges = edge_weights.clone();
    let counts = working_edges
        .iter()
        .fold(vec![0 as NodeType; node_count], |mut acc_vec, edge| {
            let index_i = (edge.i - 1) as usize;
            let index_j = (edge.j - 1) as usize;
            acc_vec[index_i] += 1;
            acc_vec[index_j] += 1;
            acc_vec
        });
    let mut enumerated_counts: Vec<(NodeType, NodeType)> = counts
        .iter()
        .enumerate()
        .map(|(x, &y)| (x as NodeType, y as NodeType))
        .collect();
    enumerated_counts.sort_by(|(_, a), (_, b)| a.cmp(b));
    enumerated_counts.iter().map(|&(a, _)| a + 1).collect()
}

/// Returns list of edges based on how far each is from being a leaf-edge.
/// Note: This is different from "how far from a leaf", but rather how far away from being a leaf
/// itself. It is a measure of the centrality of a given node.
fn get_edge_hierarchy(edge_weights: &Vec<BiEdge>) -> Vec<Vec<BiEdge>> {
    let node_count: usize = edge_weights.len() + 1;
    let mut working_edges = edge_weights.clone();
    let mut layers = Vec::<Vec<BiEdge>>::new();
    let counts = working_edges
        .iter()
        .fold(vec![0; node_count], |mut acc_vec, edge| {
            let index_i = (edge.i - 1) as usize;
            let index_j = (edge.j - 1) as usize;
            acc_vec[index_i] += 1;
            acc_vec[index_j] += 1;
            acc_vec
        });

    let mut now_serving = 0;
    while !working_edges.is_empty() {
        now_serving += 1;

        let leaves: Vec<BiEdge> = working_edges
            .extract_if(.., |edge| {
                let index_i = (edge.i - 1) as usize;
                let index_j = (edge.j - 1) as usize;
                counts[index_i].le(&now_serving) || counts[index_j].le(&now_serving)
            })
            .collect();
        layers.push(leaves);
    }
    layers
}

fn main() {
    let number_nodes: usize = read_one();
    let node_penalties = read_vec::<WeightType>();
    let edge_weights: Vec<BiEdge> = {
        let mut out = Vec::with_capacity(2 * number_nodes - 1);
        for _ in 0..number_nodes - 1 {
            out.push(BiEdge::from(read_array::<WeightType, 3, _>()));
        }
        out
    };
    println!("{}", SELECTED_SOLVER(node_penalties, edge_weights))
}
const SELECTED_SOLVER: fn(Vec<WeightType>, Vec<BiEdge>) -> u64 = solve;

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
        let node_penalties = (1..101).collect::<Vec<WeightType>>();
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

        let node_penalties = (node_start..node_start + node_count).collect::<Vec<WeightType>>();
        let edge_weights: Vec<BiEdge> = (0..node_count - 1)
            .map(|i| {
                let j = i % node_count;
                [i + node_start, j + node_start + 1, 1].into()
            })
            .collect();
        assert_eq!(solve(node_penalties, edge_weights), 10000);
    }

    #[test]
    fn test_get_nodes_in_hierarchy_order() {
        let edge_weights: Vec<BiEdge> = vec![
            [3, 2, 8].into(),
            [5, 2, 10].into(),
            [4, 3, 10].into(),
            [2, 1, 2].into(),
        ];
        let nodelist = get_nodes_in_hierarchy_order(&edge_weights);
        assert_eq!(nodelist.len(), edge_weights.len() + 1);
        assert_eq!(nodelist, vec![1, 4, 5, 3, 2])
    }

    #[test]
    fn test_optsolve_50_nodes() {
        let node_count = 50;
        let node_start = 1;
        let mut node_penalties = (node_start..node_start + node_count).collect::<Vec<WeightType>>();
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
        let mut node_penalties = (node_start..node_start + node_count).collect::<Vec<WeightType>>();
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
        let mut node_penalties = (node_start..node_start + node_count).collect::<Vec<WeightType>>();
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
        let mut node_penalties = (node_start..node_start + node_count).collect::<Vec<WeightType>>();
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
        let mut node_penalties = (node_start..node_start + node_count).collect::<Vec<WeightType>>();
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

    #[test]
    fn test_optsolve_50000_nodes() {
        let node_count = 50000;
        let node_start = 1;
        let mut node_penalties = (node_start..node_start + node_count).collect::<Vec<WeightType>>();
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
    #[allow(dead_code)]
    fn test_optsolve_100000_nodes() {
        let node_count = 100000;
        let node_start = 1;
        let mut node_penalties = (node_start..node_start + node_count).collect::<Vec<WeightType>>();
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
    #[allow(dead_code)]
    fn test_optsolve_200000_nodes() {
        let node_count = 200000;
        let node_start = 1;
        let mut node_penalties = (node_start..node_start + node_count).collect::<Vec<WeightType>>();
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
        let mut node_penalties = (1..201).collect::<Vec<WeightType>>();
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
            .collect::<Vec<WeightType>>();

        let edge_weights: Vec<BiEdge> = (0..41).map(|i| [i + 1, i + 2, 100000].into()).collect();
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 196008)
    }

    #[test]
    fn test_long_cheap_path() {
        let node_penalties = std::iter::once(2)
            .chain(std::iter::repeat_n(70, 40))
            .chain(std::iter::once(2))
            .collect::<Vec<WeightType>>();

        let edge_weights: Vec<BiEdge> = (0..41).map(|i| [i + 1, i + 2, 1].into()).collect();
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 6028)
    }

    #[test]
    fn test_10k_star() {
        let node_count: WeightType = 10000;
        let edgeweights: WeightType = 1000000;
        let node_costs: WeightType = 100000;
        let node_penalties =
            std::iter::repeat_n(node_costs, node_count as usize).collect::<Vec<WeightType>>();
        let edge_weights: Vec<BiEdge> = (0..node_count - 1)
            .map(|i| [1, i + 2, edgeweights].into())
            .collect();

        let out = SELECTED_SOLVER(node_penalties, edge_weights);
        assert_eq!(out, 100000000000000);
    }

    #[test]
    fn test_get_edge_hierarchy() {
        let edge_weights: Vec<BiEdge> = vec![
            [3, 2, 8].into(),
            [5, 2, 10].into(),
            [4, 3, 10].into(),
            [2, 1, 2].into(),
        ];
        /*  1
            |
            2
           / \
          3   5
         /
        4           */

        let layers = get_edge_hierarchy(&edge_weights);
        assert_eq!(
            layers,
            vec![
                vec![
                    BiEdge::new(5, 2, 10),
                    BiEdge::new(4, 3, 10),
                    BiEdge::new(2, 1, 2),
                ],
                vec![BiEdge::new(3, 2, 8)],
            ]
        );
    }

    #[test]
    fn test_solve_2tree_10000() {
        // For a binary tree we can trivially make one by saying that left is 2*N and right is 2*N+1
        // what the parent value is. That should be enough to calculate the edges.

        let node_count: WeightType = 10000;
        let edgeweights: WeightType = 1000000;
        let node_costs: WeightType = 100000;
        let node_penalties =
            std::iter::repeat_n(node_costs, node_count as usize).collect::<Vec<WeightType>>();
        let edge_weights: Vec<BiEdge> = (1..node_count)
            .flat_map(|i| {
                let mut out = Vec::with_capacity(2);
                let j = 2 * i;
                if j <= node_count {
                    out.push(BiEdge::new(
                        i as NodeType,
                        j as NodeType,
                        (j * j * j + i) % edgeweights + 1,
                    ))
                }
                if (j + 1) <= node_count {
                    let j = j + 1;
                    out.push(BiEdge::new(
                        i as NodeType,
                        j as NodeType,
                        (j * j * j + i) % edgeweights + 1,
                    ))
                }
                out
            })
            .collect();

        let out = SELECTED_SOLVER(node_penalties, edge_weights);
        assert_eq!(out, 100000000000000);
    }

    #[test]
    fn test_solve_2tree_50000() {
        // For a binary tree we can trivially make one by saying that left is 2*N and right is 2*N+1
        // what the parent value is. That should be enough to calculate the edges.

        let node_count: WeightType = 50000;
        let edgeweights: WeightType = 1000000;
        let node_costs: WeightType = 100000;
        let node_penalties =
            std::iter::repeat_n(node_costs, node_count as usize).collect::<Vec<WeightType>>();
        let edge_weights: Vec<BiEdge> = (1..node_count)
            .flat_map(|i| {
                let mut out = Vec::with_capacity(2);
                let j = 2 * i;
                if j <= node_count {
                    out.push(BiEdge::new(
                        i as NodeType,
                        j as NodeType,
                        (j * j * j + i) % edgeweights + 1,
                    ))
                }
                if (j + 1) <= node_count {
                    let j = j + 1;
                    out.push(BiEdge::new(
                        i as NodeType,
                        j as NodeType,
                        (j * j * j + i) % edgeweights + 1,
                    ))
                }
                out
            })
            .collect();

        let out = SELECTED_SOLVER(node_penalties, edge_weights);
        assert_eq!(out, 500000000000000);
    }

    // #[test]
    #[allow(dead_code)]
    fn test_treestruct() {
        struct Tredge {
            i: std::rc::Weak<Node>,
            j: std::rc::Weak<Node>,
            weight: WeightType,
        }
        struct Node {
            friends: Vec<std::rc::Weak<Tredge>>,
            node: NodeType,
            penalty: WeightType,
        }

        struct Treeholder {
            plucked: Vec<bool>,
            // nodes: Vec<WeightType>,
            // Vector / array with references directly to each synth edge (for optimal starts)
            // references_to_whatever: Vec<&'a Tree<'a>>,
            trodes: Vec<std::rc::Rc<Node>>,
            tredges: Vec<std::rc::Rc<Tredge>>,
        }

        impl Treeholder {
            fn new(edge_weights: Vec<BiEdge>, penalties: &'_ Vec<WeightType>) -> Self {
                let node_count = edge_weights.len() + 1;
                let nodes = penalties.clone();
                let plucked = vec![false; node_count];

                // UGH. Curious, can I make a tree with just regular borrows if I promise to be careful?

                let _layers = get_edge_hierarchy(&edge_weights);

                let mut trodes = Vec::<std::rc::Rc<Node>>::with_capacity(2 * node_count);
                let mut tredges = Vec::<std::rc::Rc<Tredge>>::with_capacity(2 * node_count - 1);

                for i in 0..2 * node_count {
                    let node = i + 1;
                    let rc = std::rc::Rc::new(Node {
                        node: node as NodeType,
                        friends: Vec::new(),
                        penalty: *penalties.get(i).unwrap_or(&WeightType::default()),
                    });
                    trodes.push(rc);
                }

                // Make all the nodes first, and instead of making Tredges yet, make BiEdges and use
                // them as Tredge orders, then load all the Tredge references in later
                // oh shit that can't work. I need a weak reference to the Node in order to make the
                // Tredge, but I won't be able to edit the friends vector on the nodes if I do that.
                // even with this new way it won't work. I can't edit the vector and also have weak references into the node.

                for edge in edge_weights {
                    let index_i = (edge.i - 1) as usize;
                    let index_j = (edge.j - 1) as usize;

                    // Create Tredge with references to buds
                    let _rc = std::rc::Rc::new_cyclic(|future_weak| {
                        let rc_i = &mut trodes[index_i];
                        std::rc::Rc::get_mut(rc_i)
                            .unwrap()
                            .friends
                            .push(future_weak.clone());

                        let mut out = Tredge {
                            i: std::rc::Rc::downgrade(rc_i),
                            j: std::rc::Weak::new(),
                            weight: edge.weight,
                        };

                        let rc_j = &mut trodes[index_j];
                        std::rc::Rc::get_mut(rc_j)
                            .unwrap()
                            .friends
                            .push(future_weak.clone());
                        out.j = std::rc::Rc::downgrade(rc_j);

                        out
                    });

                    let rc = std::rc::Rc::new(Tredge {
                        i: std::rc::Rc::downgrade(&trodes[(edge.i - 1) as usize]),
                        j: std::rc::Rc::downgrade(&trodes[(edge.j - 1) as usize]),
                        weight: edge.weight,
                    });
                    let _weak = std::rc::Rc::downgrade(&rc);
                    tredges.push(rc);

                    // Add tredge reference to Node/trode
                }

                // After inserting, do the synth edges (other implementations could choose to put penalty on Node)
                let penalty = nodes[0];
                for i in 0..node_count {
                    let node_i = i + 1;
                    let node_j = node_i + node_count;

                    let rc = std::rc::Rc::new(Tredge {
                        i: std::rc::Rc::downgrade(&trodes[node_i - 1]),
                        j: std::rc::Rc::downgrade(&trodes[node_j - 1]),
                        weight: nodes[i] * penalty - penalty,
                    });
                    let weak = std::rc::Rc::downgrade(&rc);
                    tredges.push(rc);

                    std::rc::Rc::get_mut(&mut trodes[node_i - 1])
                        .unwrap()
                        .friends
                        .push(weak.clone());
                }

                Treeholder {
                    plucked,
                    trodes,
                    tredges,
                }
            }

            fn reset_for(&mut self, _node: NodeType) {
                self.plucked = vec![false; self.plucked.len()];
            }

            #[inline(always)]
            fn pluck(&mut self, node: NodeType) -> Vec<BiEdge> {
                // TODO: Add a "visited" or "plucked" data structure
                self.trodes[(node - 1) as usize]
                    .friends
                    .iter()
                    .map(|tredge_weak| {
                        let rc = tredge_weak.upgrade().unwrap();
                        BiEdge {
                            i: rc.i.upgrade().unwrap().node,
                            j: rc.j.upgrade().unwrap().node,
                            weight: rc.weight,
                        }
                    })
                    .collect()
            }

            #[inline(always)]
            fn contains(&self, node: NodeType) -> bool {
                !self.plucked[node as usize - 1]
            }
        }

        let node_penalties = vec![9, 7, 1, 1, 9];
        let edge_weights: Vec<BiEdge> = vec![
            [3, 2, 8].into(),
            [5, 2, 10].into(),
            [4, 3, 10].into(),
            [2, 1, 2].into(),
        ];

        let _beholder = Treeholder::new(edge_weights, &node_penalties);

        let answer = 0;
        assert_eq!(answer, 63);
    }
}
