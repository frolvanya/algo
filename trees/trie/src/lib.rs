#[derive(Default)]
struct Trie {
    is_word: bool,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: &str) {
        let mut curr = self;

        for c in word.chars() {
            let index = c as usize - 'a' as usize;
            curr = curr.children[index].get_or_insert(Box::default());
        }

        curr.is_word = true;
    }

    fn search(&self, word: &str) -> bool {
        self.find(word).map_or(false, |node| node.is_word)
    }

    fn starts_with(&self, prefix: &str) -> bool {
        self.find(prefix).is_some()
    }

    fn find(&self, word: &str) -> Option<&Trie> {
        let mut curr = self;

        for c in word.chars() {
            let index = c as usize - 'a' as usize;
            curr = match curr.children[index].as_ref() {
                Some(node) => node,
                None => return None,
            };
        }

        Some(curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();

        trie.insert("apple");
        assert_eq!(trie.search("apple"), true);
        assert_eq!(trie.search("app"), false);
        assert_eq!(trie.starts_with("app"), true);
        trie.insert("app");
        assert_eq!(trie.search("app"), true);
    }
}
