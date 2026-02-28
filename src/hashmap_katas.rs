use std::collections::HashMap;

#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

// Exercise 1 — Count word frequency
// Goal: Create a hashmap and increment counts.
pub fn word_count(words: &[&str]) -> HashMap<String, usize> {
    let mut map = HashMap::new();

    for word in words {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }

    map
}

#[test]
fn test_word_count() {
    let result = word_count(&["apple", "banana", "apple"]);
    assert_eq!(result.get("apple"), Some(&2));
    assert_eq!(result.get("banana"), Some(&1));
}

// Exercise 2 — Character frequency
// Goal: HashMap with char keys.
pub fn char_frequency(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    map
}

#[test]
fn test_char_frequency() {
    let freq = char_frequency("hello");
    assert_eq!(freq.get(&'l'), Some(&2));
    assert_eq!(freq.get(&'h'), Some(&1));
}

// Exercise 3 — Lookup with default value
// Goal: Read from a hashmap safely.
pub fn get_score(scores: &HashMap<String, i32>, name: &str) -> i32 {
    scores.get(name).copied().unwrap_or(0)
}

#[test]
fn test_get_score() {
    let mut scores = HashMap::new();
    scores.insert("Alice".to_string(), 10);

    assert_eq!(get_score(&scores, "Alice"), 10);
    assert_eq!(get_score(&scores, "Bob"), 0);
}

// Exercise 4 — Merge two hashmaps
// Goal: Combine values with same keys.
pub fn merge_counts(
    a: HashMap<String, usize>,
    b: HashMap<String, usize>,
) -> HashMap<String, usize> {
    let mut map = a;

    for (key, value) in b {
        *map.entry(key).or_insert(0) += value;
    }

    map
}

#[test]
fn test_merge_counts() {
    let mut a = HashMap::new();
    a.insert("apple".to_string(), 2);

    let mut b = HashMap::new();
    b.insert("apple".to_string(), 1);
    b.insert("banana".to_string(), 3);

    let merged = merge_counts(a, b);

    assert_eq!(merged.get("apple"), Some(&3));
    assert_eq!(merged.get("banana"), Some(&3));
}

// Exercise 5 — Find the most frequent word
// Goal: Iterate over hashmap entries.
pub fn most_frequent(map: &HashMap<String, usize>) -> Option<String> {
    map.iter()
        .max_by_key(|(_, &value)| value)
        .map(|(key, _)| key.to_string())
}

#[test]
fn test_most_frequent() {
    let mut map = HashMap::new();
    map.insert("apple".to_string(), 2);
    map.insert("banana".to_string(), 5);

    assert_eq!(most_frequent(&map), Some("banana".to_string()));
}

// Exercise 6 — Group values by key
// Goal: HashMap with Vec values.
pub fn group_by_first_letter(words: &[&str]) -> HashMap<char, Vec<String>> {
    let mut map = HashMap::new();

    for word in words {
        if let Some(first_char) = word.chars().next() {
            map.entry(first_char)
                .or_insert_with(Vec::new)
                .push(word.to_string());
        }
    }

    map
}

#[test]
fn test_group_by_first_letter() {
    let result = group_by_first_letter(&["apple", "ant", "banana"]);
    assert_eq!(result.get(&'a').unwrap().len(), 2);
    assert_eq!(result.get(&'b').unwrap().len(), 1);
}

// Exercise 7 — Invert a hashmap
// Goal: Keys become values and vice-versa.
pub fn invert_map(map: HashMap<String, i32>) -> HashMap<i32, String> {
    let mut inverted = HashMap::new();

    for (key, value) in map {
        inverted.insert(value, key);
    }

    inverted
}

#[test]
fn test_invert_map() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), 1);

    let inverted = invert_map(map);
    assert_eq!(inverted.get(&1), Some(&"a".to_string()));
}

// Exercise 8 — Filter hashmap entries
// Goal: Create a new hashmap from a predicate.
pub fn filter_above(map: &HashMap<String, i32>, threshold: i32) -> HashMap<String, i32> {
    let mut filtered = HashMap::new();

    for (key, value) in map {
        if *value > threshold {
            filtered.insert(key.to_string(), *value);
        }
    }

    filtered
}

#[test]
fn test_filter_above() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), 1);
    map.insert("b".to_string(), 10);

    let filtered = filter_above(&map, 5);
    assert!(filtered.contains_key("b"));
    assert!(!filtered.contains_key("a"));
}

// Exercise 9 — Count occurrences using ownership
// Goal: Practice ownership + into_iter.
pub fn count_numbers(nums: Vec<i32>) -> HashMap<i32, usize> {
    let mut count = HashMap::new();

    for n in nums {
        *count.entry(n).or_insert(0) += 1;
    }

    count
}

#[test]
fn test_count_numbers() {
    let counts = count_numbers(vec![1, 2, 1, 3, 2, 1]);
    assert_eq!(counts.get(&1), Some(&3));
}

// Exercise 10 — Two-level hashmap (nested)
// Goal: HashMap of hashmaps.
pub fn tally_scores(entries: Vec<(&str, &str, i32)>) -> HashMap<String, HashMap<String, i32>> {
    let mut tally = HashMap::new();

    for (player, game, score) in entries {
        *tally
            .entry(player.to_string())
            .or_insert_with(HashMap::new)
            .entry(game.to_string())
            .or_insert(0) += score;
    }

    tally
}

#[test]
fn test_tally_scores() {
    let data = vec![
        ("Alice", "Chess", 10),
        ("Alice", "Chess", 5),
        ("Bob", "Poker", 7),
    ];

    let scores = tally_scores(data);

    assert_eq!(scores["Alice"]["Chess"], 15);
    assert_eq!(scores["Bob"]["Poker"], 7);
}
