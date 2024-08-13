use std::cell::Cell;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;
use lazy_static::lazy_static;
use rand::{rngs::StdRng, SeedableRng};

pub const DEFAULT_EQUALITY_THRESHOLD: f64 = 0.0001;

thread_local! {
    pub static EQUALITY_THRESHOLD: Cell<f64> = Cell::new(DEFAULT_EQUALITY_THRESHOLD);
}

pub fn set_equality_threshold(value: f64) {
    if value < 0.0 {
        panic!("The equality threshold ({}) must be greater than or equal to zero", value);
    }
    if value.is_nan() {
        panic!("The equality threshold cannot be NaN");
    }
    EQUALITY_THRESHOLD.with(|threshold| {
        threshold.set(value);
    });
}

pub fn get_equality_threshold() -> f64 {
    EQUALITY_THRESHOLD.get()
}

lazy_static! {
    pub static ref RANDOM: Mutex<StdRng> = Mutex::new(StdRng::from_entropy());
    pub static ref TO_STRING_MODE: Mutex<ToStringMode> = Mutex::new(ToStringMode::Default);
}

#[derive(Debug)]
pub enum ToStringMode {
    Simple,
    Default,
}

const PRIME: i32 = 31;

pub fn hash<T: Hash>(items: &[T]) -> i32 {
    items.iter().fold(1, |result, element| {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        element.hash(&mut hasher);
        let element_hash = hasher.finish() as i32;
        PRIME * result + element_hash
    })
}

#[cfg(test)]
mod tests {
    use expectest::prelude::*;
    use proptest::prelude::*;
    use super::*;

    #[test]
    fn test_default_equality_threshold() {
        expect!(get_equality_threshold()).to(be_equal_to(DEFAULT_EQUALITY_THRESHOLD));
    }

    proptest! {
        #[test]
        fn test_set_equality_threshold(value in 0.0..1000.0) {
            set_equality_threshold(value);
            expect!(get_equality_threshold()).to(be_equal_to(value));
        }
    }
}
