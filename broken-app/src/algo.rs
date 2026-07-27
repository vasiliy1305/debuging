/// Намеренно низкопроизводительная реализация.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = values.to_vec();
    out.sort_unstable();
    out.dedup();
    out
}


/// Классическая экспоненциальная реализация без мемоизации — будет медленной на больших n.
pub fn slow_fib(n: u64) -> u64 {
    let mut previous = 0;
    let mut current = 1;

    for _ in 0..n {
        let next = previous + current;
        previous = current;
        current = next;
    }
    previous
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dedup_returns_sorted_unique_values() {
        let input = [5, 2, 5, 1, 2, 3];

        assert_eq!(slow_dedup(&input), vec![1, 2, 3, 5]);
    }

    #[test]
    fn fibonacci_values_are_correct() {
        assert_eq!(slow_fib(0), 0);
        assert_eq!(slow_fib(1), 1);
        assert_eq!(slow_fib(2), 1);
        assert_eq!(slow_fib(10), 55);
        assert_eq!(slow_fib(32), 2_178_309);
    }
}