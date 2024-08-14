pub trait Verifiable {
    fn is_valid(&self) -> bool {
        true
    }
}
