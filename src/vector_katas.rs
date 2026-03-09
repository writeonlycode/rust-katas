use core::num;
use std::collections::HashSet;

#[cfg(test)]
use pretty_assertions::assert_eq;

// Exercise 1 — Sum elements
// Goal: Iterate and compute a total.
pub fn sum(nums: &[i32]) -> i32 {
    let mut sum = 0;

    for n in nums {
        sum += n;
    }

    sum
}

#[test]
fn test_sum() {
    assert_eq!(sum(&[1, 2, 3]), 6);
    assert_eq!(sum(&[]), 0);
}

// Exercise 2 — Filter even numbers
// Goal: Return a new vector with only even values.
pub fn evens(nums: &[i32]) -> Vec<i32> {
    let mut filtered = Vec::new();

    for &n in nums {
        if n % 2 == 0 {
            filtered.push(n);
        }
    }

    filtered
}

#[test]
fn test_evens() {
    assert_eq!(evens(&[1, 2, 3, 4]), vec![2, 4]);
    assert_eq!(evens(&[1, 3]), Vec::<i32>::new());
}

// Exercise 3 — Double in place
// Goal: Mutate a vector via mutable reference.
pub fn double(nums: &mut Vec<i32>) {
    for n in nums {
        *n *= 2;
    }
}

#[test]
fn test_double() {
    let mut v = vec![1, 2, 3];
    double(&mut v);
    assert_eq!(v, vec![2, 4, 6]);
}

// Exercise 4 — Find maximum
// Goal: Return None for empty input.
pub fn max_value(nums: &[i32]) -> Option<i32> {
    let mut max = None;

    for &n in nums {
        max = match max {
            None => Some(n),
            Some(m) => Some(if n > m { n } else { m }),
        }
    }

    max
}

#[test]
fn test_max_value() {
    assert_eq!(max_value(&[1, 5, 2]), Some(5));
    assert_eq!(max_value(&[]), None);
}

// Exercise 5 — Reverse vector
// Goal: Consume the vector and return reversed version.
pub fn reverse(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter().rev().collect()
}

#[test]
fn test_reverse() {
    assert_eq!(reverse(vec![1, 2, 3]), vec![3, 2, 1]);
}

// Exercise 6 — Remove duplicates
// Goal: Preserve order while removing duplicates.
pub fn unique(nums: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut seen = HashSet::new();

    for &n in nums {
        if seen.insert(n) {
            result.push(n);
        }
    }

    result
}

#[test]
fn test_unique() {
    assert_eq!(unique(&[1, 2, 2, 3, 1]), vec![1, 2, 3]);
}

// Exercise 7 — Chunk vector
// Goal: Split into groups of size N.
pub fn chunks(nums: &[i32], size: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut chunk = Vec::new();

    if size == 0 {
        return result;
    }

    for &n in nums {
        chunk.push(n);

        if chunk.len() == size {
            result.push(chunk);
            chunk = Vec::new();
        }
    }

    if !chunk.is_empty() {
        result.push(chunk);
    }

    result
}

#[test]
fn test_chunks() {
    assert_eq!(
        chunks(&[1, 2, 3, 4, 5], 2),
        vec![vec![1, 2], vec![3, 4], vec![5]]
    );
}

// Exercise 8 — Flatten nested vectors
// Goal: Consume nested vectors and flatten them.
pub fn flatten(nested: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = Vec::new();

    for v in nested {
        for n in v {
            result.push(n);
        }
    }
    result
}

#[test]
fn test_flatten() {
    assert_eq!(
        flatten(vec![vec![1, 2], vec![3], vec![4, 5]]),
        vec![1, 2, 3, 4, 5]
    );
}

// Exercise 9 — Take first N safely
// Goal: Avoid panics when n > length.
pub fn take(nums: &[i32], n: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let mut remaining = n;

    for &e in nums {
        if remaining == 0 {
            break;
        }

        result.push(e);
        remaining -= 1;
    }

    result
}

#[test]
fn test_take() {
    assert_eq!(take(&[1, 2, 3], 2), vec![1, 2]);
    assert_eq!(take(&[1, 2], 5), vec![1, 2]);
}

// Exercise 10 — Partition even and odd
// Goal: Return (even, odd) vectors.
pub fn partition(nums: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut even = Vec::new();
    let mut odd = Vec::new();

    for &n in nums {
        if n % 2 == 0 {
            even.push(n);
        } else {
            odd.push(n);
        }
    }

    (even, odd)
}

#[test]
fn test_partition() {
    let (even, odd) = partition(&[1, 2, 3, 4]);
    assert_eq!(even, vec![2, 4]);
    assert_eq!(odd, vec![1, 3]);
}
