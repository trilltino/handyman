fn test_functionality() {
    assert_eq!(1 + 1, 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        test_functionality();
    }
}
