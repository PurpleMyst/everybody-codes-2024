use rustc_hash::FxHashMap as HashMap;

type Tree = HashMap<&'static str, Vec<&'static str>>;
pub type Path = Vec<&'static str>;

const FRUIT: &str = "@";

fn walk<const CHECK_CYCLES: bool>(tree: &Tree, path: &mut Path, paths: &mut Vec<Path>) {
    let &conductor = path.last().unwrap();
    if conductor == FRUIT {
        match paths.binary_search_by_key(&path.len(), |path| path.len()) {
            Ok(len) => {
                paths.remove(len);
            }
            Err(len) => {
                paths.insert(len, path.clone());
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

    let mut paths = Vec::new();
    walk::<CHECK_CYCLES>(&nodes, &mut vec!["RR"], &mut paths);
    debug_assert_eq!(paths.len(), 1);
    paths.pop().unwrap()
}
