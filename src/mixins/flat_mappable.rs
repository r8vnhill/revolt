trait FlatMappable<T> {
    fn flatten(&self) -> Vec<&T>;
}
