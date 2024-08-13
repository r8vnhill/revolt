/// Represents a feature in an evolutionary algorithm.
///
/// A feature is an atomic component in an evolutionary algorithm. For instance, a gene is a feature
/// in a genetic algorithm, and a chromosome is a vector of such features. The `Feature` trait
/// provides a common structure for these components, with methods for duplicating features with new
/// values.
///
/// ## Usage:
/// This trait is intended to be implemented by types representing individual elements in
/// evolutionary algorithms, such as genes or other units.
///
/// ### Example 1: Implementing Feature
/// ```
/// use revolt::repr::feature::Feature;
///
/// struct MyGene {
///     value: i32,
/// }
///
/// impl Feature<i32, MyGene> for MyGene {
///     fn value(&self) -> &i32 {
///         &self.value
///     }
///
///     fn duplicate_with_value(&self, value: i32) -> MyGene {
///         MyGene { value }
///     }
/// }
/// ```
///
/// # Type Parameters
/// - `T`: The type of the value held by the feature.
/// - `F`: The type of the feature itself, which must implement `Feature`.
pub trait Feature<T, F>
where
    F: Feature<T, F>,
{
    /// Returns the value held by the feature.
    fn value(&self) -> &T;

    /// Creates a duplicate of the feature with the specified value.
    ///
    /// # Arguments
    /// - `value`: The value for the new feature.
    ///
    /// # Returns
    /// A new feature with the specified value.
    fn duplicate_with_value(&self, value: T) -> F;
}
