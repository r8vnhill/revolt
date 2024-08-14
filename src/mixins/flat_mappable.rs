pub trait FlatMappable<T> where T: Clone {
    fn flatten(&self) -> Vec<T> where T: Clone;

    fn flat_map<R>(&self, f: impl Fn(T) -> Vec<R>) -> Vec<R> {
        self.flatten().into_iter().flat_map(f).collect()
    }
}
