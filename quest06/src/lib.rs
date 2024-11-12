use std::collections::hash_map::Entry;

use rustc_hash::FxHashMap as HashMap;

type Id = &'static str;
type Tree = HashMap<Id, Vec<Id>>;
pub type Path = Vec<Id>;

const FRUIT: &str = "@";

fn walk<const CHECK_CYCLES: bool>(tree: &Tree, path: &mut Path, paths: &mut HashMap<usize, Path>) {
    let &conductor = path.last().unwrap();
    if conductor == FRUIT {
        match paths.entry(path.len()) {
            Entry::Occupied(entry) => {
                entry.remove();
            }
            Entry::Vacant(entry) => {
                entry.insert(path.clone());
            }
        }
        return;
    }

    let Some(children) = tree.get(conductor) else {
        return;
    };

    children.iter().for_each(|&node| {
        if CHECK_CYCLES && path.contains(&node) {
            return;
        }
        path.push(node);
        walk::<CHECK_CYCLES>(tree, path, paths);
        path.pop();
    })
}


pub fn solve<const CHECK_CYCLES: bool>(input: &'static str) -> Vec<&str> {
    let mut nodes = HashMap::default();
    input.lines().for_each(|line| {
        let (node, children) = line.split_once(':').unwrap();
        let children = children.split(',').collect::<Vec<_>>();
        nodes.insert(node, children);
    });

    let mut paths = Default::default();
    walk::<CHECK_CYCLES>(&nodes, &mut vec!["RR"], &mut paths);
    debug_assert_eq!(paths.len(), 1);
    paths.into_values().next().unwrap()
}
