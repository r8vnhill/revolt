pub trait Foldable<A> {
    fn fold<B>(&self, initial: B, f: impl Fn(B, &A) -> B) -> B;
}
