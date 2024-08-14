use std::cell::Cell;
use std::sync::Mutex;
use lazy_static::lazy_static;
use rand::{rngs::StdRng, SeedableRng};

/// The default threshold value for equality comparisons.
///
/// This constant defines the default value used to compare floating-point values for equality. The
/// threshold is typically used in situations where strict equality may not be reliable due to the
/// precision issues inherent in floating-point arithmetic.
///
/// # Value
/// `DEFAULT_EQUALITY_THRESHOLD` is set to `0.0001`.
pub const DEFAULT_EQUALITY_THRESHOLD: f64 = 0.0001;

thread_local! {
    /// A thread-local storage for the equality threshold.
    ///
    /// This thread-local variable stores the current threshold value for equality comparisons. Each
    /// thread can have its own independent threshold value.
    ///
    /// # Initialization
    /// The threshold is initialized to `DEFAULT_EQUALITY_THRESHOLD` by default.
    pub static EQUALITY_THRESHOLD: Cell<f64> = Cell::new(DEFAULT_EQUALITY_THRESHOLD);
}

/// Sets the global equality threshold for comparisons.
///
/// This function updates the threshold used to compare floating-point values for equality. The
/// threshold value must be non-negative and cannot be NaN.
///
/// # Parameters
/// - `value`: The new threshold value. Must be greater than or equal to `0.0` and not NaN.
///
/// # Panics
/// This function will panic if:
/// - The `value` is less than `0.0`.
/// - The `value` is NaN.
///
/// # Example
/// ```
/// use revolt::domain::set_equality_threshold;
/// set_equality_threshold(0.001);
/// ```
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

/// Retrieves the current equality threshold.
///
/// This function returns the current value of the equality threshold used for floating-point
/// comparisons. The threshold is stored in thread-local storage, so each thread may have a different value.
///
/// # Returns
/// The current equality threshold as a `f64`.
///
/// # Example
/// ```
/// use revolt::domain::get_equality_threshold;
/// let threshold = get_equality_threshold();
/// println!("Current threshold: {}", threshold);
/// ```
pub fn get_equality_threshold() -> f64 {
    EQUALITY_THRESHOLD.get()
}

lazy_static! {
    /// A global, thread-safe random number generator.
    ///
    /// This RNG is initialized with system entropy and can be safely shared across threads. It is
    /// protected by a `Mutex` to ensure that only one thread can access it at a time.
    pub static ref RANDOM: Mutex<StdRng> = Mutex::new(StdRng::from_entropy());

    /// A global, thread-safe variable to control the string representation mode.
    ///
    /// This variable determines how certain objects are converted to strings for display or
    /// debugging purposes. The mode can be either `Simple` or `Default`.
    pub static ref TO_STRING_MODE: Mutex<ToStringMode> = Mutex::new(ToStringMode::Default);
}

/// An enumeration that defines different modes for converting objects to strings.
///
/// This enum controls how objects are represented as strings in different contexts. It can be used
/// to toggle between a simple or detailed string representation.
///
/// # Variants
/// - `Simple`: A simple, compact representation.
/// - `Default`: The standard, detailed representation.
#[derive(Debug)]
pub enum ToStringMode {
    Simple,
    Default,
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

        #[test]
        fn test_set_equality_threshold_panics(value in -1000.0..0.0) {
            expect!(std::panic::catch_unwind(|| set_equality_threshold(value))).to(be_err());
        }
    }

    #[test]
    fn test_set_equality_threshold_panics_nan() {
        expect!(std::panic::catch_unwind(|| set_equality_threshold(f64::NAN))).to(be_err());
    }
}
