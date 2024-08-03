#[allow(unused)]
use std::{cell::RefCell, rc::Weak};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct Node {
    value: char,
    is_word: RefCell<bool>,
    parent: RefCell<Weak<Node>>,
    children: RefCell<HashMap<char, Rc<Node>>>,
}

impl Node {
    pub fn new(char: char) -> Self {
        Self {
            value: char,
            is_word: RefCell::new(false),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(HashMap::new()),
        }
    }

    pub fn new_with_parent(char: char, parent: Weak<Node>) -> Self {
        let node = Node::new(char);
        *node.parent.borrow_mut() = parent;

        node
    }
}

#[derive(Debug)]
pub struct Trie {
    root: Rc<Node>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: Rc::new(Node::new('*')),
        }
    }

    pub fn from_words(words: &[&str]) -> Self {
        let trie = Trie::new();
        trie.insert_many(words);

        trie
    }

    pub fn insert_word(&self, word: &str) {
        let mut current = Rc::clone(&self.root);

        for char in word.chars() {
            if current.children.borrow().contains_key(&char) {
                current = {
                    let temp = Rc::clone(current.children.borrow().get(&char).unwrap());
                    temp
                };
                continue;
            }

            let new_node = Rc::new(Node::new_with_parent(char, Rc::downgrade(&current)));

            current
                .children
                .borrow_mut()
                .insert(char, Rc::clone(&new_node));

            current = new_node;
        }

        *current.is_word.borrow_mut() = true;
    }

    pub fn insert_many(&self, words: &[&str]) {
        for word in words {
            self.insert_word(word)
        }
    }

    pub fn search_words(&self, query: &str) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        search_words_from_node(Rc::clone(&self.root), make_query(query), &mut result);
        result
    }
}

/// Create a HashMap from the chars in the string as key and their presence
/// as count.
fn make_query(query: &str) -> HashMap<char, u8> {
    let mut map: HashMap<char, u8> = HashMap::new();

    for char in query.chars() {
        map.entry(char).and_modify(|v| *v += 1u8).or_insert(1u8);
    }

    map
}

/// Recursively searches a node with the given query. If it finds any complete
/// words, it pushes it to the resulting Vector.
fn search_words_from_node(node: Rc<Node>, query: HashMap<char, u8>, result: &mut Vec<String>) {
    for (char, _) in &query {
        if let Some(child) = node.children.borrow().get(&char) {
            if *child.is_word.borrow() {
                let mut word_chars: Vec<char> = Vec::new();
                collect_word(Rc::clone(child), &mut word_chars);

                word_chars.reverse();
                result.push(String::from_iter(word_chars));
            }

            search_words_from_node(
                Rc::clone(child),
                decrement_char_from_query(&query, &char),
                result,
            );
        }
    }
}

/// Traverse upwards from a node until it reaches the root node and keeps
/// pushing each nodes char into given resulting Vec<char>.
fn collect_word(node: Rc<Node>, chars: &mut Vec<char>) {
    if let Some(parent) = node.parent.borrow().upgrade() {
        chars.push(node.value);
        collect_word(Rc::clone(&parent), chars);
    }
}

/// Decrements the given char from the query map. If count reaches to zero it
/// removes the char from the map.
fn decrement_char_from_query(query: &HashMap<char, u8>, char: &char) -> HashMap<char, u8> {
    query
        .iter()
        .filter_map(|(current_char, count)| {
            let new_count = if current_char == char {
                count - 1u8
            } else {
                *count
            };

            if new_count == 0u8 {
                return None;
            }

            Some((*current_char, new_count))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decrement_char_from_query() {
        let query = HashMap::from([('a', 2), ('b', 0)]);

        assert_eq!(
            decrement_char_from_query(&query, &'a'),
            HashMap::from([('a', 1)])
        );
    }

    #[test]
    fn test_make_query() {
        assert_eq!(make_query("ab"), HashMap::from([('a', 1), ('b', 1)]));
        assert_eq!(make_query("aab"), HashMap::from([('a', 2), ('b', 1)]));
        assert_eq!(make_query("aabb"), HashMap::from([('a', 2), ('b', 2)]));
    }

    #[test]
    fn test_trie_works() {
        let trie = Trie::from_words(&["car", "cart", "cat", "cow", "cool"]);

        let test = |query: &str, expected: &[&str]| {
            let mut result = trie.search_words(query);
            result.sort();

            let mut expected: Vec<String> = expected.iter().map(|v| String::from(*v)).collect();

            expected.sort();

            assert_eq!(result, expected);
        };

        test("rtac", &["car", "cat", "cart"]);
        test("woolc", &["cow", "cool"]);
        test("abc", &[] as &[&str]);
    }
}
