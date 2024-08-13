pub trait Foldable<T> {
    fn fold<R>(&self, initial: R, f: fn(R, T) -> R) -> R;
}
