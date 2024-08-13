pub trait Foldable<A> {
    fn fold<B>(&self, initial: B, f: fn(B, A) -> B) -> B;
}
