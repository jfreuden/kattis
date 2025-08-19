mod tests;

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

fn get_nodes_in_hierarchy_order(path_counts: &Vec<usize>) -> Vec<NodeType> {
    let mut enumerated_counts: Vec<(NodeType, NodeType)> = path_counts
        .iter()
        .enumerate()
        .map(|(node_index, &counts)| (node_index as NodeType, counts as NodeType))
        .collect();
    // If both nodes are leaves, then make the lower index the parent, and the higher index the child.
    enumerated_counts.sort_by(
        |(a_index, a), (b_index, b)|
            a.cmp(b).then(a_index.cmp(b_index).reverse())
    );
    enumerated_counts.iter().map(|&(a, _)| a + 1).collect()
}

/// Returns list of edges based on how far each is from being a leaf-edge.
/// Note: This is different from "how far from a leaf", but rather how far away from being a leaf
/// itself. It is a measure of the centrality of a given node.
fn get_edge_hierarchy(edge_weights: &Vec<BiEdge>, path_counts: &Vec<usize>) -> Vec<Vec<BiEdge>> {
    let mut working_edges = edge_weights.clone();
    let mut layers = Vec::<Vec<BiEdge>>::new();
    let mut now_serving = 0;
    while !working_edges.is_empty() {
        let leaves: Vec<BiEdge> = working_edges
            .extract_if(.., |edge| {
                let index_i = (edge.i - 1) as usize;
                let index_j = (edge.j - 1) as usize;
                let i_is_leaf = path_counts[index_i].le(&now_serving);
                let j_is_leaf = path_counts[index_j].le(&now_serving);
                i_is_leaf || j_is_leaf
            })
            .collect();
        if leaves.is_empty() {
            continue;
        }
        layers.push(leaves);
        now_serving += 1;
    }
    layers
}

fn get_node_pathcounts(edge_weights: &Vec<BiEdge>) -> Vec<usize> {
    let node_count = edge_weights.len() + 1;
    let mut working_edges = edge_weights.clone();
    let mut path_counts = vec![0; node_count];
    let mut step_counts = vec![0; node_count];
    let mut node_edgelists = vec![Vec::<&BiEdge>::new(); node_count];

    step_counts = working_edges
        .iter()
        .fold(step_counts, |mut step_vec, edge| {
            let index_i = (edge.i - 1) as usize;
            let index_j = (edge.j - 1) as usize;
            step_vec[index_i] += 1;
            step_vec[index_j] += 1;
            node_edgelists[index_i].push(edge);
            node_edgelists[index_j].push(edge);
            step_vec
        });

    let leaf_nodes: Vec<NodeType> = step_counts.iter().enumerate().filter_map(|(node_index, &node_count)| {
        if node_count <= 1 {
            Some((node_index + 1) as NodeType)
        } else {
            None
        }
    }).collect();

    let mut visit_queue: std::collections::VecDeque<(NodeType, usize)> = std::collections::VecDeque::with_capacity(node_count);
    for leaf_node in leaf_nodes {
        for &edge in &node_edgelists[leaf_node as usize - 1] {
            visit_queue.push_back((edge.connected_to(leaf_node).unwrap(), 0));
        }
        node_edgelists[leaf_node as usize - 1].clear()
    }

    while let Some((node, layer_value)) = visit_queue.pop_front() {
        let index = (node - 1) as usize;
        if node_edgelists[index].is_empty() {
            continue
        }
        path_counts[index] = layer_value + 1;
        for &edge in &node_edgelists[index] {
            let attached = edge.connected_to(node).unwrap();
            visit_queue.push_back((attached, layer_value + 1));
        }
        node_edgelists[index].clear();
    }
    path_counts
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
const SELECTED_SOLVER: fn(Vec<WeightType>, Vec<BiEdge>) -> u64 = convex_solve;

#[derive(Eq, Copy, Clone, Debug)]
struct HullPart {
    range_start: WeightType,
    path_cost: WeightType,
    end_penalty: WeightType,
}

impl PartialEq for HullPart {
    fn eq(&self, other: &Self) -> bool {
        self.path_cost == other.path_cost &&
            self.end_penalty == other.end_penalty
    }
}

impl PartialOrd for HullPart {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.cmp(other)
        )
    }
}

impl Ord for HullPart {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.end_penalty.cmp(&other.end_penalty)
            .then(self.path_cost.cmp(&other.path_cost))
    }
}

// fn get_intersected_hullpart(last_addition: &HullPart, candidate: &HullPart) -> HullPart {
//
// }

/// Helper data structure for O(1) queries of minimum path + penalty costs.
#[derive(Debug)]
struct ConvexHull {
    penalty: WeightType,
    parent_edge: Option<BiEdge>,
    hull_parts: Vec<HullPart>,
    children: Vec<NodeType>,
}

fn generate_hullparts(node_order: &Vec<NodeType>, node_hulls: &mut Vec<ConvexHull>) {
    for &node in node_order.iter() {
        let node_index = (node - 1) as usize;
        let mut combined_hullparts: Vec<HullPart> = node_hulls[node_index].hull_parts.clone();
        let my_children = &node_hulls[node_index].children;
        let mut filtered_hullparts = if my_children.is_empty() {
            vec![combined_hullparts.pop().unwrap()]
        } else {
            let childrens_hulls: Vec<&ConvexHull> = my_children.iter().map(
                |&child| &node_hulls[(child - 1) as usize]
            ).collect();

            let mut hullpart_heap = Vec::<HullPart>::new();
            hullpart_heap.push(combined_hullparts.pop().unwrap());
            for child_hull in childrens_hulls {
                let edited_hull_parts =
                    child_hull.hull_parts.iter().map(|&hullpart| HullPart {
                        range_start: 0,
                        path_cost: hullpart.path_cost + child_hull.parent_edge.unwrap().weight,
                        end_penalty: hullpart.end_penalty,
                    });
                hullpart_heap.extend(edited_hull_parts);
            }
            hullpart_heap.sort();

            let (mut filtered_hullparts, _) = hullpart_heap.iter().fold(
                (Vec::<HullPart>::with_capacity(hullpart_heap.len()), WeightType::MAX),
                |(mut vec, mut min_tercept), hullpart | {
                    if hullpart.path_cost < min_tercept {
                        vec.push(*hullpart);
                        min_tercept = hullpart.path_cost;
                    }
                (vec, min_tercept)
            });
            filtered_hullparts
        };
        // Now that we have the MinHeap (lowest-slope, lowest-intercept), we need to pull all valid edges into a vector
        // skipping edges with higher intercepts
        let hullpart_list = finish_hull(&mut filtered_hullparts);
        combined_hullparts.extend(hullpart_list);
        node_hulls[node_index].hull_parts = combined_hullparts;
    }
}

fn finish_hull(filtered_hullparts: &mut Vec<HullPart>) -> Vec<HullPart> {
    let mut hullpart_list = Vec::<HullPart>::new();

    while let Some(mut hullpart) = filtered_hullparts.pop() {
        // Let's make a 'stutter'. This is where we use a while let as an if let, because we might have to retry after popping.
        'inner: while let Some(&last_addition) = hullpart_list.last() {
            // intersection x = (q - p) / (m - n)
            let intercept_diff = hullpart.path_cost - last_addition.path_cost;
            let slope_diff = last_addition.end_penalty - hullpart.end_penalty;
            let intercept = intercept_diff.div_ceil(slope_diff) as WeightType;
            if intercept.le(&last_addition.range_start) {
                hullpart_list.pop();
                continue 'inner;
            }
            hullpart.range_start = intercept;
            break 'inner;
        }
        hullpart_list.push(hullpart);
    }
    hullpart_list
}

fn get_layers_set_hull_relationships(edge_weights: &Vec<BiEdge>, path_counts: &Vec<usize>, mut node_hulls: &mut Vec<ConvexHull>) -> Vec<Vec<BiEdge>> {
    let mut working_edges = edge_weights.clone();
    let path_counts_max = *path_counts.iter().max().unwrap();
    let mut layers = vec![Vec::<BiEdge>::new(); path_counts_max + 1];

    for edge in working_edges.iter() {
        let index_i = (edge.i - 1) as usize;
        let index_j = (edge.j - 1) as usize;

        // Look at the path_counts and place them in the layer bucket for whichever node is in a lower count.
        let i_hierarchy = path_counts[index_i];
        let j_hierarchy = path_counts[index_j];

        // If both nodes in this edge are leaves, then make the lower index the parent, and the higher index the child.
        // If one is a leaf, then make the leaf the child.
        let (node_index, parent_index) = match i_hierarchy.cmp(&j_hierarchy).then(index_i.cmp(&index_j).reverse()) {
            std::cmp::Ordering::Less => (index_i, index_j),
            std::cmp::Ordering::Equal => panic!("Invalid edge: Identical `path_count` and node index"),
            std::cmp::Ordering::Greater => (index_j, index_i),
        };

        node_hulls[parent_index].children.push((node_index + 1) as NodeType);
        node_hulls[node_index].parent_edge = Some(*edge);
        layers[std::cmp::min(i_hierarchy, j_hierarchy)].push(*edge);
    }
    layers
}

fn create_hull_blanks(node_penalties: &Vec<WeightType>, node_count: usize) -> Vec<ConvexHull> {
    let mut node_hulls = Vec::<ConvexHull>::with_capacity(node_count);
    for &penalty in node_penalties.iter() {
        node_hulls.push(ConvexHull {
            penalty,
            parent_edge: None,
            hull_parts: vec![HullPart {
                range_start: 0,
                path_cost: 0,
                end_penalty: penalty,
            }],
            children: Vec::<NodeType>::new(),
        });
    }
    node_hulls
}

/// A solver making use of convex hulls and a hierarchical tree decomposition.
fn convex_solve(node_penalties: Vec<WeightType>, edge_weights: Vec<BiEdge>) -> u64 {
    let node_count: usize = node_penalties.len();
    let mut node_hulls = create_hull_blanks(&node_penalties, node_count);

    // TODO: Combine path_counts and hull_relationships code, returning the node order instead of layers
    let path_counts = get_node_pathcounts(&edge_weights);
    let node_order = get_nodes_in_hierarchy_order(&path_counts);
    let _ = get_layers_set_hull_relationships(&edge_weights, &path_counts, &mut node_hulls);
    generate_hullparts(&node_order, &mut node_hulls);

    // Going up the tree for a query you are adding the path distances and simply checking if there is a new minimum.
    // Going down the tree for building the hulls, you are merging hulls.

    // TODO: Write the sampling code, particularly the part with the climb up the parent edges.
    node_order.iter().map(|&node| {
        let mut node_index = (node - 1) as usize;
        let mut best_min = WeightType::MAX;
        let mut convex_hull = &node_hulls[node_index];
        let start_penalty = convex_hull.penalty;
        let mut path_offset = 0;
        loop {
            best_min = std::cmp::min(best_min, convex_hull.hull_parts.iter().map(|hullpart| {
                // TODO: Binary Search for only the one sampling point, instead of a min.
                path_offset + hullpart.path_cost + start_penalty * hullpart.end_penalty
            }).min().unwrap());

            if let Some(parent_edge) = &convex_hull.parent_edge {
                node_index = (parent_edge.connected_to((node_index + 1) as NodeType).unwrap() - 1) as usize;
                convex_hull = &node_hulls[node_index];
                path_offset += parent_edge.weight;
            } else {
                break;
            }
        }
        best_min
    }).sum()
}

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
    fn test_solve_linetree_100_nodes() {
        let node_count = 100;
        let node_start = 1;

        let node_penalties = (node_start..node_start + node_count).collect::<Vec<WeightType>>();
        let edge_weights: Vec<BiEdge> = (0..node_count - 1)
            .map(|i| {
                let j = i % node_count;
                [i + node_start, j + node_start + 1, 1].into()
            })
            .collect();
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 10000);
    }

    #[test]
    fn test_get_nodes_in_hierarchy_order() {
        let edge_weights: Vec<BiEdge> = vec![
            [3, 2, 8].into(),
            [5, 2, 10].into(),
            [4, 3, 10].into(),
            [2, 1, 2].into(),
        ];
        let path_counts = get_node_pathcounts(&edge_weights);
        let nodelist = get_nodes_in_hierarchy_order(&path_counts);
        assert_eq!(nodelist.len(), edge_weights.len() + 1);
        assert_eq!(nodelist, vec![5, 4, 1, 3, 2])
    }

    #[test]
    fn test_solve_linetree_50_nodes() {
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
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 9697);
    }

    #[test]
    fn test_solve_linetree_500_nodes() {
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
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 7296625);
    }

    #[test]
    fn test_solve_linetree_1000_nodes() {
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
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 58466345);
    }

    #[test]
    fn test_solve_linetree_2000_nodes() {
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
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 548823761);
    }

    #[test]
    fn test_solve_linetree_10000_nodes() {
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
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 52691143621);
    }

    #[test]
    fn test_solve_linetree_50000_nodes() {
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
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 571803609907);
    }

    #[test]
    #[allow(dead_code)]
    fn test_solve_linetree_100000_nodes() {
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
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 0);
    }

    #[test]
    #[allow(dead_code)]
    fn test_solve_linetree_200000_nodes() {
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
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 0);
    }

    #[test]
    fn test_solve_linetree_200_nodes() {
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
    fn test_solve_long_cheap_path() {
        let node_penalties = std::iter::once(2)
            .chain(std::iter::repeat_n(70, 40))
            .chain(std::iter::once(2))
            .collect::<Vec<WeightType>>();

        let edge_weights: Vec<BiEdge> = (0..41).map(|i| [i + 1, i + 2, 1].into()).collect();
        assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 6028)
    }

    #[test]
    fn test_solve_10k_star() {
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
    fn test_get_edge_hierarchy_kattis_testcase() {
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
        let path_counts = get_node_pathcounts(&edge_weights);
        let layers = get_edge_hierarchy(&edge_weights, &path_counts);
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

        let node_count: usize = 10000;
        let edgeweights: WeightType = 1000000;
        let node_costs: WeightType = 100000;
        let (node_penalties, edge_weights) = make_test_2tree(node_count, edgeweights, node_costs);

        let out = SELECTED_SOLVER(node_penalties, edge_weights);
        assert_eq!(out, 100000000000000);
    }

    #[test]
    fn test_solve_2tree_50000() {
        // For a binary tree we can trivially make one by saying that left is 2*N and right is 2*N+1
        // what the parent value is. That should be enough to calculate the edges.

        let node_count: usize = 50000;
        let edgeweights: WeightType = 1000000;
        let node_costs: WeightType = 100000;
        let (node_penalties, edge_weights) = make_test_2tree(node_count, edgeweights, node_costs);

        let out = SELECTED_SOLVER(node_penalties, edge_weights);
        assert_eq!(out, 500000000000000);
    }

    fn make_test_2tree(
        node_count: usize,
        edgeweights: WeightType,
        node_costs: WeightType,
    ) -> (Vec<WeightType>, Vec<BiEdge>) {
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
                        (j * j * j + i) as WeightType % edgeweights + 1,
                    ))
                }
                if (j + 1) <= node_count {
                    let j = j + 1;
                    out.push(BiEdge::new(
                        i as NodeType,
                        j as NodeType,
                        (j * j * j + i) as WeightType % edgeweights + 1,
                    ))
                }
                out
            })
            .collect();
        (node_penalties, edge_weights)
    }

    #[test]
    #[allow(dead_code)]
    fn test_solve_2tree_100000() {
        // For a binary tree we can trivially make one by saying that left is 2*N and right is 2*N+1
        // what the parent value is. That should be enough to calculate the edges.
        let node_count: usize = 100000;
        let edgeweights: WeightType = 1000000;
        let node_costs: WeightType = 100000;
        let (node_penalties, edge_weights) = make_test_2tree(node_count, edgeweights, node_costs);

        let out = SELECTED_SOLVER(node_penalties, edge_weights);
        assert_eq!(out, 1000000000000000);
    }

    #[test]
    #[allow(dead_code)]
    fn test_solve_2tree_200000() {
        // For a binary tree we can trivially make one by saying that left is 2*N and right is 2*N+1
        // what the parent value is. That should be enough to calculate the edges.
        let node_count: usize = 200000;
        let edgeweights: WeightType = 1000000;
        let node_costs: WeightType = 100000;
        let (node_penalties, edge_weights) = make_test_2tree(node_count, edgeweights, node_costs);
        let path_counts = get_node_pathcounts(&edge_weights);
        let _layers: Vec<Vec<BiEdge>> = get_edge_hierarchy(&edge_weights, &path_counts);

        let out = SELECTED_SOLVER(node_penalties, edge_weights);
        assert_eq!(out, 2000000000000000);
    }

    #[test]
    fn test_get_edge_hierarchy_2tree_4layer() {
        // For a binary tree we can trivially make one by saying that left is 2*N and right is 2*N+1
        //                        1
        //            2                      3
        //      4           5            6           7
        //  8     9     10    11     12    13    14    15
        //16 17 18 19  20 21 22 23 24 25  26 27 28 29 30 31
        let node_count: usize = 31;
        let edgeweights: WeightType = 2;
        let node_costs: WeightType = 2;
        let (_node_penalties, edge_weights) = make_test_2tree(node_count, edgeweights, node_costs);
        let path_counts = get_node_pathcounts(&edge_weights);
        let _node_hierarchies = get_nodes_in_hierarchy_order(&path_counts);
        let layers: Vec<Vec<BiEdge>> = get_edge_hierarchy(&edge_weights, &path_counts);

        assert_eq!(layers.len(), 4); // There should be 4 layers
        let mut layer = layers.iter();
        assert_eq!(
            *layer.next().unwrap(),
            edge_weights[16 - 2..=31 - 2].to_vec()
        );
        assert_eq!(
            *layer.next().unwrap(),
            edge_weights[8 - 2..=15 - 2].to_vec()
        );
        assert_eq!(*layer.next().unwrap(), edge_weights[4 - 2..=7 - 2].to_vec());
        assert_eq!(*layer.next().unwrap(), edge_weights[2 - 2..=3 - 2].to_vec());

        SELECTED_SOLVER(_node_penalties, edge_weights);
    }
}
