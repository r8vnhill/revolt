pub trait FlatMappable<T> {
    fn flatten(&self) -> Vec<T>;

    fn flat_map<R>(&self, f: impl Fn(T) -> Vec<R>) -> Vec<R> {
        self.flatten().into_iter().flat_map(f).collect()
    }
}
