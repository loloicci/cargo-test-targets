/// ```
/// assert_eq!(cargo_test_targets::add(2, 2), 5)
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit() {
        let result = add(2, 2);
        assert_eq!(result, 5);
    }
}
