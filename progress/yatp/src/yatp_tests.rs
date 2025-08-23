use super::*;

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
    assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 208975396308);
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
    assert_eq!(SELECTED_SOLVER(node_penalties, edge_weights), 162272919846);
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
