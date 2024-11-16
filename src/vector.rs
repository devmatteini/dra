use std::collections::HashSet;
use std::hash::Hash;

pub fn unique<T: Eq + Hash + Clone>(value: Vec<T>) -> Vec<T> {
    let mut set = HashSet::new();
    value
        .into_iter()
        .filter(|item| set.insert(item.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::vector::unique;

    #[test]
    fn unique_consecutive_duplicated_elements() {
        let input = vec![1, 2, 2, 2, 3, 4];
        assert_eq!(vec![1, 2, 3, 4], unique(input));
    }

    #[test]
    fn unique_all_duplicated_elements() {
        let input = vec![1, 2, 3, 4, 1, 2, 3, 4];
        assert_eq!(vec![1, 2, 3, 4], unique(input));
    }

    #[test]
    fn unique_elements() {
        let input = vec![1, 2, 3, 4];
        assert_eq!(input.clone(), unique(input));
    }

    #[test]
    fn unique_respect_original_order() {
        let input = vec![5, 5, 3, 3, 3, 1, 10, 10];
        assert_eq!(vec![5, 3, 1, 10], unique(input));
    }
}
