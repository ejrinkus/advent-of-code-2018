use std::collections::HashMap;

/// Represents a single node within a trie.
#[derive(Debug)]
pub struct TrieNode {
    /// The character value stored in this node.
    pub val: char,
    /// The child nodes below this node, keyed on the chars stored within them.
    pub children: HashMap<char, TrieNode>,
    /// If this node is not a leaf node (i.e. the end of a string), then `leaf`
    /// will be `None`. Otherwise, it will be `Some(string)` where string is the
    /// string ending in the character stored by this node.
    pub leaf: Option<String>,
}

/// Represents a trie structure. Each node (except the root) in the trie
/// represents a single character of one or more strings stored in the trie.
/// The depth of the node represents the index of that character within each
/// of the strings that use it.
#[derive(Debug)]
pub struct Trie {
    /// The root is a node that stores no character value, but whose children
    /// are the first characters of each of the strings in the trie.
    root: TrieNode,
}

impl Trie {
    /// Returns a new trie structure that stores no strings.
    ///
    /// # Example
    ///
    /// ```
    /// use trie::Trie;
    ///
    /// let mut trie = Trie::new();
    pub fn new() -> Trie {
        return Trie {
            root: TrieNode {
                val: 0 as char,
                children: HashMap::new(),
                leaf: None
            }
        }
    }

    /// Inserts a string into this trie.
    ///
    /// # Arguments
    ///
    /// * `val` - A string to be stored in this trie.
    ///
    /// # Example
    ///
    /// ```
    /// use trie::Trie;
    ///
    /// let mut trie = Trie::new();
    /// trie.insert("something".to_string());
    /// ```
    pub fn insert(&mut self, val: String) {
        let mut current = &mut self.root;
        for c in val.chars() {
            current = identity(current).children.entry(c).or_insert(TrieNode {
                    val: c,
                    children: HashMap::new(),
                    leaf: None
                });
        }
        current.leaf = Some(val);
    }

    /// Returns whether or not a string is stored in this trie.
    ///
    /// This function returns the TrieNode containing the final character of the
    /// string if the string. The node may or may not be a leaf, however. The
    /// `leaf` field of the returned node will match the input string if the input
    /// string itself was inserted into the trie. The field will be `None`, however,
    /// if the string is merely a prefix to other strings contained in the trie.
    ///
    /// # Arguments
    ///
    /// * `val` - A string to be searched for in this trie.
    ///
    /// # Returns
    ///
    /// * `Some(node)` where `node` is the node containing the final character
    ///    of `val`.
    /// * `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use trie::Trie;
    /// let mut trie = Trie::new();
    /// trie.insert( String::from("hello"));
    ///
    /// let search_str = String::from("hello");
    /// match trie.contains(&search_str) {
    ///     Some(node) => match node.leaf {
    ///         Some(ref leaf) => assert_eq!(leaf, &search_str),
    ///         None => assert!(false)
    ///     },
    ///     None => assert!(false)
    /// }
    /// ```
    pub fn contains(&self, val: &String) -> Option<&TrieNode> {
        let mut current = &self.root;
        for c in val.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return None
            }
        }
        return Some(current);
    }


    /// Searches for a string in the trie that differs from the input string by exactly
    /// one character.
    ///
    /// Returns a string consisting of only the matching characters, or None if there
    /// is no such string.
    ///
    /// For example, assume a trie where "abcdef" was inserted. Passing "abgdef" to
    /// this function will match since it only differs from "abcdef" by one character
    /// (the third character). The returned string ("abdef") omits that character.
    /// "hbgdef" will not match, on the other hand, because it is different in two
    /// places.
    ///
    /// None will also be returned in the case of an exact match.
    ///
    /// # Arguments
    ///
    /// * `val` - A string to be searched for in this trie.
    ///
    /// # Returns
    ///
    /// * `true` if there is a string in this trie that only differs from `val  
    ///   by one character.
    /// * `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use trie::Trie;
    ///
    /// let mut trie = Trie::new();
    /// trie.insert(String::from("abcdef"));
    ///
    /// let search_str1 = String::from("abgdef");
    /// match trie.match_off_by_one(&search_str1) {
    ///     Some(match_str) => assert_eq!(match_str, "abdef".to_string()),
    ///     None => assert!(false)
    /// }
    ///
    /// let search_str2 = String::from("hbgdef");
    /// match trie.match_off_by_one(&search_str2) {
    ///     Some(_match_str) => assert!(false),
    ///     None => assert!(true)
    /// }
    /// ```
    pub fn match_off_by_one(&self, val: &String) -> Option<String> {
        // First, find the largest prefix of val that is in this trie.
        let mut current = &self.root;
        let mut prefix = String::new();
        for c in val.chars() {
            match current.children.get(&c) {
                Some(node) => {
                    current = node;
                    prefix.push(c);
                },
                None => {
                    break;
                }
            }
        }
        // We now know the character at the index prefix.len() doesn't match the
        // character in val at that index. So we want to skip past it in val, and
        // check to see if the remaining suffix is in this trie, starting with
        // each of the children of the current node.
        for mut child in current.children.values() {
            let mut suffix = String::new();
            for c in val.chars().skip(prefix.len()+1) {
                match child.children.get(&c) {
                    Some(node) => {
                        child = node;
                        suffix.push(c);
                    },
                    None => {
                        break;
                    }
                }
            }
            // If prefix.len() + suffix.len() is equal to val.len() - 1, then we
            // know we found the full remaining suffix and can return. Otherwise
            // we need to keep checking subsequent children.
            if (prefix.len() + suffix.len()) == (val.len() - 1) {
                return Some([prefix, suffix].concat());
            }
        }
        return None;
    }
}

fn identity<T>(t: T) -> T { t }

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn can_find_after_insert() {
        let mut trie = Trie::new();
        trie.insert( String::from("hello"));

        let search_str = String::from("hello");
        match trie.contains(&search_str) {
            Some(node) => match node.leaf {
                Some(ref leaf) => assert_eq!(leaf, &search_str),
                None => assert!(false)
            },
            None => assert!(false)
        }
    }

    #[test]
    fn can_find_prefix() {
        let mut trie = Trie::new();
        trie.insert(String::from("hello world"));

        let search_str = String::from("hello");
        match trie.contains(&search_str) {
            Some(node) => match node.leaf {
                Some(ref _leaf) => assert!(false),
                None => assert!(true)
            },
            None => assert!(false)
        }
    }

    #[test]
    fn word_does_not_exist() {
        let mut trie = Trie::new();
        trie.insert(String::from("hello"));

        let search_str = String::from("world");
        match trie.contains(&search_str) {
            Some(_node) => assert!(false),
            None => assert!(true)
        }
    }

    #[test]
    fn test_off_by_one() {
        let mut trie = Trie::new();
        trie.insert(String::from("abcdef"));

        let search_str1 = String::from("abgdef");
        match trie.match_off_by_one(&search_str1) {
            Some(match_str) => assert_eq!(match_str, "abdef".to_string()),
            None => assert!(false)
        }

        let search_str2 = String::from("hbgdef");
        match trie.match_off_by_one(&search_str2) {
            Some(_match_str) => assert!(false),
            None => assert!(true)
        }
    }

    #[test]
    fn test_off_by_one_start() {
        let mut trie = Trie::new();
        trie.insert(String::from("abcdef"));

        let search_str = String::from("gbcdef");
        match trie.match_off_by_one(&search_str) {
            Some(match_str) => assert_eq!(match_str, String::from("bcdef")),
            None => assert!(false)
        }
    }

    #[test]
    fn test_off_by_one_end() {
        let mut trie = Trie::new();
        trie.insert(String::from("abcdef"));

        let search_str = String::from("abcdeg");
        match trie.match_off_by_one(&search_str) {
            Some(match_str) => assert_eq!(match_str, String::from("abcde")),
            None => assert!(false)
        }
    }
}
