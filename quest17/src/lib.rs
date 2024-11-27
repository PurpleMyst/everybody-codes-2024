use petgraph::data::FromElements;

type Point = (usize, usize);

fn manhattan_distance(p1: Point, p2: Point) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

pub fn solve_part12(input: &str) -> impl std::fmt::Display {
    let map = grid::Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );
    let stars = map
        .indexed_iter()
        .filter(|(_, &c)| c == b'*')
        .map(|(pos, _)| pos)
        .collect::<Vec<_>>();

    let mut graph = petgraph::Graph::new();
    let mut node_indices = Vec::new();
    for i in 0..stars.len() {
        node_indices.push(graph.add_node(i));
    }

    for (i, star1) in stars.iter().enumerate() {
        for (j, star2) in stars.iter().enumerate().skip(i + 1) {
            let distance = manhattan_distance(*star1, *star2);
            graph.add_edge(node_indices[i], node_indices[j], distance);
        }
    }

    let distances = petgraph::algo::min_spanning_tree(&graph)
        .map(|element| match element {
            petgraph::data::Element::Node { .. } => 0,
            petgraph::data::Element::Edge { weight, .. } => weight,
        })
        .sum::<usize>();

    stars.len() + distances
}

pub fn solve_part3(input: &str) -> impl std::fmt::Display {
    let map = grid::Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );
    let stars = map
        .indexed_iter()
        .filter(|(_, &c)| c == b'*')
        .map(|(pos, _)| pos)
        .collect::<Vec<_>>();

    let mut graph = petgraph::Graph::new();
    let mut node_indices = Vec::new();
    for i in 0..stars.len() {
        node_indices.push(graph.add_node(i));
    }

    for (i, star1) in stars.iter().enumerate() {
        for (j, star2) in stars.iter().enumerate().skip(i + 1) {
            let distance = manhattan_distance(*star1, *star2);
            if distance < 6 {
                graph.add_edge(node_indices[i], node_indices[j], distance);
            }
        }
    }

    let graph = petgraph::graph::UnGraph::<usize, usize>::from_elements(petgraph::algo::min_spanning_tree(&graph));

    let constellation_size = |nodes: Vec<petgraph::graph::NodeIndex>| {
        nodes.len()
            + nodes
                .into_iter()
                .map(|node| graph.edges(node).map(|edge| edge.weight()).sum::<usize>())
                .sum::<usize>()
                / 2
    };

    let mut components = petgraph::algo::kosaraju_scc(&graph)
        .into_iter()
        .map(|component| constellation_size(component))
        .collect::<Vec<_>>();
    components.sort_unstable();
    components.reverse();

    components.into_iter().take(3).product::<usize>()
}
