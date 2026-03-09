#[cfg(test)]
use pretty_assertions::assert_eq;

// Exercise 1 — Filter greater than
// Goal: Basic single-condition filtering.
pub fn greater_than(nums: &[i32], threshold: i32) -> Vec<i32> {
    nums.iter().copied().filter(|&n| n > threshold).collect()
}

#[test]
fn test_greater_than() {
    assert_eq!(greater_than(&[1, 5, 2, 8], 3), vec![5, 8]);
}

// Exercise 2 — Filter even AND greater than
// Goal: Combine multiple conditions.
pub fn even_and_greater_than(nums: &[i32], threshold: i32) -> Vec<i32> {
    nums.iter()
        .copied()
        .filter(|&n| n % 2 == 0 && n > threshold)
        .collect()
}

#[test]
fn test_even_and_greater_than() {
    assert_eq!(even_and_greater_than(&[1, 2, 3, 4, 6, 9], 3), vec![4, 6]);
}

// Exercise 3 — Filter within a range
// Goal: Use two boundary conditions.
pub fn between(nums: &[i32], min: i32, max: i32) -> Vec<i32> {
    nums.iter()
        .copied()
        .filter(|&n| n >= min && n <= max)
        .collect()
}

#[test]
fn test_between() {
    assert_eq!(between(&[1, 5, 10, 15, 20], 5, 15), vec![5, 10, 15]);
}

// Exercise 4 — Exclude values
// Goal: Negative filtering.
pub fn exclude(nums: &[i32], value: i32) -> Vec<i32> {
    nums.iter().copied().filter(|&n| n != value).collect()
}

#[test]
fn test_exclude() {
    assert_eq!(exclude(&[1, 2, 3, 2, 4], 2), vec![1, 3, 4]);
}

// Exercise 5 — Filter with predicate
// Goal: Accept a reusable predicate function.
pub fn filter_with<F>(nums: &[i32], predicate: F) -> Vec<i32>
where
    F: Fn(i32) -> bool,
{
    nums.iter().copied().filter(|&n| predicate(n)).collect()
}

#[test]
fn test_filter_with() {
    let result = filter_with(&[1, 2, 3, 4], |n| n % 2 == 0);
    assert_eq!(result, vec![2, 4]);
}

// Exercise 6 — Apply multiple predicates
// Goal: Keep values that satisfy ALL conditions.
pub fn filter_all(nums: &[i32], predicates: Vec<Box<dyn Fn(i32) -> bool>>) -> Vec<i32> {
    nums.iter()
        .copied()
        .filter(|&n| predicates.iter().all(|predicate| predicate(n)))
        .collect()
}

#[test]
fn test_filter_all() {
    let predicates: Vec<Box<dyn Fn(i32) -> bool>> =
        vec![Box::new(|n| n % 2 == 0), Box::new(|n| n > 3)];

    let result = filter_all(&[1, 2, 3, 4, 6], predicates);
    assert_eq!(result, vec![4, 6]);
}

// Exercise 7 — Any predicate match
// Goal: Keep values that satisfy AT LEAST ONE condition.
pub fn filter_any(nums: &[i32], predicates: Vec<Box<dyn Fn(i32) -> bool>>) -> Vec<i32> {
    nums.iter()
        .copied()
        .filter(|&n| predicates.iter().any(|predicate| predicate(n)))
        .collect()
}

#[test]
fn test_filter_any() {
    let predicates: Vec<Box<dyn Fn(i32) -> bool>> = vec![Box::new(|n| n < 2), Box::new(|n| n > 5)];

    let result = filter_any(&[1, 2, 3, 6], predicates);
    assert_eq!(result, vec![1, 6]);
}

// Exercise 8 — Chain two filters
// Goal: First filter evens, then filter greater than threshold.
pub fn even_then_greater(nums: &[i32], threshold: i32) -> Vec<i32> {
    nums.iter()
        .copied()
        .filter(|&n| n % 2 == 0 && n > threshold)
        .collect()
}

#[test]
fn test_even_then_greater() {
    assert_eq!(even_then_greater(&[1, 2, 3, 4, 6, 8], 4), vec![6, 8]);
}

// Exercise 9 — Filter and map
// Goal: Keep evens and return them doubled.
pub fn filter_and_double(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .copied()
        .filter_map(|n| (n % 2 == 0).then_some(n * 2))
        .collect()
}

#[test]
fn test_filter_and_double() {
    assert_eq!(filter_and_double(&[1, 2, 3, 4]), vec![4, 8]);
}

// Exercise 10 — Query builder
// Goal: Apply multiple filters AND a transformation.
pub fn query<F>(nums: &[i32], predicates: Vec<Box<dyn Fn(i32) -> bool>>, transform: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    nums.iter()
        .copied()
        .filter(|&n| predicates.iter().all(|predicate| predicate(n)))
        .map(transform)
        .collect()
}

#[test]
fn test_query() {
    let predicates: Vec<Box<dyn Fn(i32) -> bool>> =
        vec![Box::new(|n| n % 2 == 0), Box::new(|n| n > 2)];

    let result = query(&[1, 2, 3, 4, 6], predicates, |n| n * 10);
    assert_eq!(result, vec![40, 60]);
}
