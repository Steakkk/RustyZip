use std::{collections::HashMap, fs, rc::Rc};

#[derive(Debug)]
enum Tree {
    Leaf { ch: char },
    Node { left: Rc<Tree>, right: Rc<Tree> },
}

fn get_sorted_index(sorted_vector: &Vec<(Tree, usize)>, other: &usize) -> usize {
    let result = sorted_vector.binary_search_by(|(_, count)| count.cmp(other));
    match result {
        Ok(index) => index,
        Err(index) => index,
    }
}

fn parse_text(text: &String) -> HashMap<char, usize> {
    text.chars().fold(HashMap::new(), |mut acc, ch| {
        *acc.entry(ch).or_default() += 1;
        acc
    })
}

fn parse_map(map: HashMap<char, usize>) -> Vec<(Tree, usize)> {
    map.iter().fold(vec![], |mut acc, (ch, count)| {
        let index = get_sorted_index(&acc, count);
        acc.insert(index, (Tree::Leaf { ch: *ch }, *count));
        acc
    })
}

fn get_tree(mut tree_as_vector: Vec<(Tree, usize)>) -> Tree {
    if tree_as_vector.len() == 0 {
        return Tree::Leaf { ch: '\0' };
    }

    if tree_as_vector.len() == 1 {
        return match tree_as_vector[0] {
            (Tree::Leaf { ch }, _) => Tree::Leaf { ch },
            _ => panic!("This case should not be possible"),
        };
    }

    while tree_as_vector.len() > 2 {
        let (right, right_count) = tree_as_vector.remove(0);
        let (left, left_count) = tree_as_vector.remove(0);
        let count = left_count + right_count;
        let index = get_sorted_index(&tree_as_vector, &count);
        tree_as_vector.insert(
            index,
            (
                Tree::Node {
                    left: Rc::new(left),
                    right: Rc::new(right),
                },
                count,
            ),
        )
    }

    let (right, _) = tree_as_vector.remove(0);
    let (left, _) = tree_as_vector.remove(0);
    Tree::Node {
        left: Rc::new(left),
        right: Rc::new(right),
    }
}

fn dfs(tree: &Tree, result: &mut HashMap<char, u8>, mut code: u8, depth: u8) {
    match tree {
        Tree::Leaf { ch } => {
            result.insert(*ch, code);
        },
        Tree::Node { left, right } => {
            dfs(left, result, code, depth << 1);
            code ^= depth;
            dfs(right, result, code, depth << 1);
        }
    }
}

fn get_char_encoding(tree: Tree) -> HashMap<char, u8> {
    let mut result = HashMap::new();
    let code = 0b0000_0000;
    match tree {
        Tree::Leaf { ch } => {
            result.insert(ch, code);
        },
        _ => dfs(&tree, &mut result, code, 0b0000_0001)
    }
    result
}

fn encode_text(text: &String, char_encoding: HashMap<char, u8>) -> Vec<u8> {
    text.chars().fold(vec![], |mut acc, ch| {
        acc.push(*char_encoding.get(&ch).unwrap());
        acc
    })
}

pub fn encode(input_path: String, output_path: String) {
    let text = fs::read_to_string(input_path).expect("Wrong input file path!");
    let map = parse_text(&text);
    let tree_as_vector = parse_map(map);
    let tree = get_tree(tree_as_vector);
    let char_encoding = get_char_encoding(tree);
    let contents = encode_text(&text, char_encoding);
    fs::write(output_path, contents).expect("Wrong output file path!");
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;

    #[test]
    fn get_sorted_index_in_empty_vector() {
        let sorted_vector = vec![];
        let count = 0;
        let index = get_sorted_index(&sorted_vector, &count);
        assert_eq!(index, 0);
        let count = 10;
        let index = get_sorted_index(&sorted_vector, &count);
        assert_eq!(index, 0);
    }

    #[test]
    fn get_sorted_index_classic_test() {
        let mut sorted_vector = vec![];
        let element = Tree::Leaf { ch: '0' };
        let count = 1;
        sorted_vector.push((element, count));
        let element = Tree::Leaf { ch: '0' };
        let count = 3;
        sorted_vector.push((element, count));
        let count = 2;
        let index = get_sorted_index(&sorted_vector, &count);
        assert_eq!(index, 1);
        let count = 0;
        let index = get_sorted_index(&sorted_vector, &count);
        assert_eq!(index, 0);
        let count = 4;
        let index = get_sorted_index(&sorted_vector, &count);
        assert_eq!(index, 2);
    }

    #[test]
    fn parse_text_classic_test() {
        let map = parse_text(&String::from("aaababababa"));
        assert!(map.contains_key(&'a'));
        assert!(map.contains_key(&'b'));
        assert_eq!(map.keys().len(), 2);
        assert_eq!(*map.get(&'a').unwrap(), 7);
        assert_eq!(*map.get(&'b').unwrap(), 4);
    }

    #[test]
    fn parse_text_empty_test() {
        let map = parse_text(&String::from(""));
        assert!(map.is_empty());
    }

    #[test]
    fn parse_map_classic_test() {
        let map = parse_text(&String::from("aaababababa"));
        let sorted_vector = parse_map(map);
        assert_eq!(sorted_vector.len(), 2);
        assert_eq!(sorted_vector.get(0).unwrap().1, 4);
        assert_eq!(sorted_vector.get(1).unwrap().1, 7);
    }

    #[test]
    fn parse_map_empty_test() {
        let map = parse_text(&String::from(""));
        let sorted_vector = parse_map(map);
        assert_eq!(sorted_vector.len(), 0);
    }

    #[test]
    fn get_tree_classic_test() {
        let map = parse_text(&String::from("aaababababa"));
        let sorted_vector = parse_map(map);
        let tree = get_tree(sorted_vector);
        match tree {
            Tree::Node { left, right } => {
                match left.borrow() {
                    Tree::Leaf { ch } => assert_eq!(ch, &'a'),
                    Tree::Node { left: _, right: _ } => {
                        panic!("Left should be a Tree::Leaf with 'a' as value")
                    }
                }
                match right.borrow() {
                    Tree::Leaf { ch } => assert_eq!(ch, &'b'),
                    Tree::Node { left: _, right: _ } => {
                        panic!("Right should be a Tree::Leaf with 'b' as value")
                    }
                }
            }
            _ => panic!("Tree should be a Tree::Node!"),
        }
    }

    #[test]
    fn get_tree_three_chars_test() {
        let map = parse_text(&String::from("aaababababaccc"));
        let sorted_vector = parse_map(map);
        let tree = get_tree(sorted_vector);
        match tree {
            Tree::Node { left, right } => {
                match left.borrow() {
                    Tree::Leaf { ch } => assert_eq!(ch, &'a'),
                    Tree::Node { left: _, right: _ } => {
                        panic!("Left should be a Tree::Leaf with 'a' as value")
                    }
                }
                match right.borrow() {
                    Tree::Leaf { ch: _ } => panic!("Left should be a Tree::Node"),
                    Tree::Node { left, right } => {
                        match left.borrow() {
                            Tree::Leaf { ch } => assert_eq!(ch, &'b'),
                            Tree::Node { left: _, right: _ } => {
                                panic!("Left should be a Tree::Leaf with 'b' as value")
                            }
                        }
                        match right.borrow() {
                            Tree::Leaf { ch } => assert_eq!(ch, &'c'),
                            Tree::Node { left: _, right: _ } => {
                                panic!("Right should be a Tree::Leaf with 'c' as value")
                            }
                        }
                    }
                }
            }
            _ => panic!("Tree should be a Tree::Node!"),
        }
    }

    #[test]
    fn get_tree_empty_test() {
        let map = parse_text(&String::from(""));
        let sorted_vector = parse_map(map);
        let tree = get_tree(sorted_vector);
        match tree {
            Tree::Leaf { ch } => assert_eq!(ch, '\0'),
            _ => panic!("Tree should be a Tree::Leaf!"),
        }
    }

    #[test]
    fn get_char_encoding_classic_test() {
        let map = parse_text(&String::from("aaababababa"));
        let sorted_vector = parse_map(map);
        let tree = get_tree(sorted_vector);
        let char_encoding = get_char_encoding(tree);
        assert_eq!(char_encoding.get(&'a').unwrap(), &0b0000_0000);
        assert_eq!(char_encoding.get(&'b').unwrap(), &0b0000_0001);
    }

    #[test]
    fn get_char_encoding_five_chars_test() {
        let map = parse_text(&String::from("ababababacccdee"));
        let sorted_vector = parse_map(map);
        let tree = get_tree(sorted_vector);
        let char_encoding = get_char_encoding(tree);
        assert_eq!(char_encoding.get(&'a').unwrap(), &0b0000_0000);
        assert_eq!(char_encoding.get(&'b').unwrap(), &0b0000_0010);
        assert_eq!(char_encoding.get(&'c').unwrap(), &0b0000_0001);
        assert_eq!(char_encoding.get(&'d').unwrap(), &0b0000_0111);
        assert_eq!(char_encoding.get(&'e').unwrap(), &0b0000_0011);
    }

    #[test]
    fn get_char_encoding_empty_test() {
        let map = parse_text(&String::from(""));
        let sorted_vector = parse_map(map);
        let tree = get_tree(sorted_vector);
        let char_encoding = get_char_encoding(tree);
        assert_eq!(char_encoding.get(&'\0').unwrap(), &0b0000_0000);
    }
}
