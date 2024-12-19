use std::cmp::min;
use std::collections::HashMap;

pub fn count_possibilities(
    towels: &[String],
    pattern: &str,
    cache: &mut HashMap<String, i64>,
) -> i64 {
    // Use cache if possible
    if cache.contains_key(pattern) {
        return *cache.get(pattern).unwrap();
    }

    // Compute
    let min_len = towels.iter().map(|tow| tow.len()).min().unwrap();
    let max_len = min(
        pattern.len(),
        towels.iter().map(|tow| tow.len()).max().unwrap(),
    );

    // Recursion limit
    if pattern.is_empty() {
        cache.insert(pattern.to_string(), 1);
        return 1;
    } else if pattern.len() < min_len {
        cache.insert(pattern.to_string(), 0);
        return 0;
    }

    for i in min_len..=max_len {
        let pre_string = &pattern[0..i];
        if towels.contains(&pre_string.to_string()) {
            *cache.entry(pattern.to_string()).or_insert(0) +=
                count_possibilities(towels, &pattern[i..], cache);
        }
    }

    *cache.entry(pattern.to_string()).or_insert(0)
}
