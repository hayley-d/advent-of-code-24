use std::collections::HashMap;

// 1. Construct a trie for the substrings XMAS and SAMX
pub struct Vertex {
    children: HashMap<char, Vertex>,
    word_node: bool,
}

pub struct Trie {
    root: Box<Vertex>,
}

pub struct FailureLink {}

pub struct Dictionary {
    patterns: Vec<String>,
}

impl Trie {
    pub fn new() -> Self {
        return Trie {
            root: Box::new(Vertex::new()),
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
}

impl Vertex {
    pub fn new() -> Self {
        return Vertex {
            children: HashMap::new(),
            word_node: false,
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
}
