use std::collections::hash_map::Entry;
use std::hash::Hash;

use rustc_hash::FxHashMap as HashMap;

type Tree<Id> = HashMap<Id, Vec<Id>>;

fn walk<Id: Hash + Eq + Copy, IsFruit: Fn(Id) -> bool>(
    tree: &Tree<Id>,
    path: &mut Vec<Id>,
    paths: &mut HashMap<usize, Vec<Id>>,
    is_fruit: &IsFruit,
) {
    let &conductor = path.last().unwrap();
    if is_fruit(conductor) {
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

    let Some(children) = tree.get(&conductor) else {
        return;
    };

    children.iter().for_each(|&node| {
        path.push(node);
        walk::<Id, IsFruit>(tree, path, paths, is_fruit);
        path.pop();
    })
}

fn solve<
    const CHECK_FOR_BUGS: bool,
    Id: Hash + Eq + Copy,
    ConvertId: Fn(&'static str) -> Id,
    IsFruit: Fn(Id) -> bool,
>(
    input: &'static str,
    convert_id: ConvertId,
    is_fruit: IsFruit,
) -> Vec<Id> {
    let mut nodes = HashMap::default();
    input.lines().for_each(|line| {
        let (node, children) = line.split_once(':').unwrap();
        if CHECK_FOR_BUGS && matches!(node, "ANT" | "BUG") {
            return;
        }
        let children = children
            .split(',')
            .filter(|&s| !(CHECK_FOR_BUGS && matches!(s, "ANT" | "BUG")))
            .map(&convert_id)
            .collect::<Vec<_>>();
        nodes.insert(convert_id(node), children);
    });

    let mut paths = Default::default();
    walk::<Id, IsFruit>(&nodes, &mut vec![convert_id("RR")], &mut paths, &is_fruit);
    debug_assert_eq!(paths.len(), 1);
    paths.into_values().next().unwrap()
}

pub fn solve_part1(input: &'static str) -> String {
    type Id = [u8; 2];
    const FRUIT: Id = [b'\0', b'@'];
    solve::<false, _, _, _>(
        input,
        |s| match s {
            "@" => FRUIT,
            _ => Id::try_from(s.as_bytes()).unwrap(),
        },
        |s| s == FRUIT,
    )
    .into_iter()
    .fold(String::new(), |mut acc, s| {
        acc.push_str(match s {
            FRUIT => "@",
            _ => std::str::from_utf8(&s[..]).unwrap(),
        });
        acc
    })
}

fn solve_part23<const CHECK_CYCLES: bool>(input: &'static str) -> String {
    type Id = [u8; 4];
    const FRUIT: Id = [b'\0', b'\0', b'\0', b'@'];
    const ROOT: Id = [b'\0', b'\0', b'R', b'R'];
    solve::<CHECK_CYCLES, _, _, _>(
        input,
        |s| match s {
            "@" => FRUIT,
            "RR" => ROOT,
            _ => Id::try_from(s.as_bytes()).unwrap(),
        },
        |s| s == FRUIT,
    )
    .into_iter()
    .fold(String::new(), |mut acc, s| {
        acc.push(match s {
            FRUIT => '@',
            ROOT => 'R',
            _ => s[0] as char,
        });
        acc
    })
}

pub fn solve_part2(input: &'static str) -> String {
    solve_part23::<false>(input)
}

pub fn solve_part3(input: &'static str) -> String {
    solve_part23::<true>(input)
}
