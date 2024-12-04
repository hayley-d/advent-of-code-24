#[allow(dead_code)]
use std::collections::HashMap;
use std::rc::Rc;

// 1. Construct a trie for the substrings XMAS and SAMX
pub struct Vertex {
    children: HashMap<char, Vertex>,
    word_node: bool,
    failure_link: Option<FailureLink>,
}

pub struct Trie {
    root: Vertex,
}

pub struct FailureLink {
    start: Option<Rc<Vertex>>,
    end: Option<Rc<Vertex>>,
}

pub struct Dictionary {
    patterns: Vec<String>,
}

pub struct Automaton {
    trie: Trie,
}

impl Trie {
    pub fn new() -> Self {
        let mut root: Vertex = Vertex::new();

        return Trie {
            root: Vertex::new(),
        };
    }

    pub fn insert(&mut self, word: &str) {
        let chars: Vec<char> = word.chars().collect();
        let mut current: &mut Vertex = &mut self.root;

        for c in chars {
            // check if an edge esists for the current char
            let result: Option<_> = current.children.get(&c);

            match result {
                Some(_) => (),
                None => {
                    current.children.insert(c, Vertex::new());
                }
            }

            current = current.children.get_mut(&c).unwrap();
        }

        current.word_node = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut current: &Vertex = &self.root;

        for c in word.chars() {
            let result = current.children.get(&c);
            match result {
                Some(node) => {
                    current = node;
                }
                None => return false,
            }
        }
        return current.word_node;
    }
}

impl Vertex {
    pub fn new() -> Self {
        return Vertex {
            children: HashMap::new(),
            word_node: false,
            failure_link: None,
        };
    }
}

impl FailureLink {
    pub fn new(start: Rc<Vertex>, end: Rc<Vertex>) -> Self {
        return FailureLink {
            start: Some(start),
            end: Some(end),
        };
    }

    pub fn add_start(&mut self, start: Rc<Vertex>) {
        self.start = Some(start);
    }

    pub fn add_end(&mut self, end: Rc<Vertex>) {
        self.end = Some(end);
    }
}

impl Default for FailureLink {
    fn default() -> Self {
        return FailureLink {
            start: None,
            end: None,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_insert() {
        let word: String = String::from("sloth");

        let mut trie: Trie = Trie::new();

        trie.insert(&word);

        assert_eq!(trie.root.word_node, false);
        assert_eq!(
            match trie.root.children.get(&'s') {
                Some(_) => true,
                None => false,
            },
            true
        );
    }

    #[test]
    fn test_search() {
        let word: String = String::from("sloth");
        let word2: String = String::from("slith");

        let mut trie: Trie = Trie::new();

        trie.insert(&word);

        assert_eq!(trie.search(&word), true);
        assert_eq!(trie.search(&word2), false);
    }
}
