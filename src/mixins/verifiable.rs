pub trait Verifiable {
    fn verify(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use expectest::prelude::*;
    use super::*;

    #[test]
    fn test_verify_default() {
        let verifiable = MyVerifiable {};
        expect!(verifiable.verify()).to(be_true());
    }

    struct MyVerifiable;

    impl Verifiable for MyVerifiable {}
}
