#[cfg(test)]
pub(crate) mod test {
    use std::fmt::Debug;
    
    #[allow(dead_code)]
    pub(crate) fn assert_contains_in_any_order<T>(
        actual: impl IntoIterator<Item = T>,
        expected: impl IntoIterator<Item = T>,
    ) where
        T: Debug + Eq,
    {
        let actual_vec: Vec<T> = actual.into_iter().collect();
        let expected_vec: Vec<T> = expected.into_iter().collect();
        assert_eq!(
            actual_vec.len(),
            expected_vec.len(),
            "The actual length of the does not match the expected length"
        );

        for expected_value in expected_vec {
            assert!(
                actual_vec.contains(&expected_value),
                "{:?} was not found",
                expected_value,
            );
        }
    }
}
