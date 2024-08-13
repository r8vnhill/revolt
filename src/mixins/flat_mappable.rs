pub trait FlatMappable<T> {
    fn flat_map<U, F>(&self, f: F) -> U
    where
        F: Fn(&T) -> U,
    {
        f(&self.flatten())
    }

    fn flatten(&self) -> T;
}

#[cfg(test)]
mod tests {
    use expectest::prelude::*;
    use proptest::prelude::*;
    use super::*;

    proptest! {
        #[test]
        fn test_flat_map_does_not_panic(i in 0..1000) {
            let flat_mappable = MyFlatMappable {};
            let result = flat_mappable.flat_map(|x| x + i);
            expect!(result).to(be_equal_to(i + 420));
        }
    }

    struct MyFlatMappable;

    impl FlatMappable<i32> for MyFlatMappable {
        fn flatten(&self) -> i32 {
            420
        }
    }
}
