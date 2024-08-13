use std::hash::Hash;
use std::ops::Deref;
use crate::domain::{ToStringMode, TO_STRING_MODE};
use crate::mixins::flat_mappable::FlatMappable;
use crate::mixins::foldable::Foldable;
use crate::mixins::verifiable::Verifiable;
use crate::repr::feature::Feature;
use crate::repr::representation::Representation;

#[derive(Clone)]
pub struct Individual<T, F, R>
where
    F: Feature<T, F>,
    R: Representation<T, F>,
{
    pub representation: R,
    pub fitness: f64,
}

impl<T, F, R> Individual<T, F, R>
where
    F: Feature<T, F>,
    R: Representation<T, F>,
{
    pub fn new(representation: R) -> Self {
        Individual {
            representation,
            fitness: f64::nan(),
        }
    }

    pub fn size(&self) -> usize {
        self.representation.size()
    }

    pub fn is_evaluated(&self) -> bool {
        !self.fitness.is_nan()
    }

    pub fn to_string(&self) -> String {
        match TO_STRING_MODE.lock().unwrap().deref() {
            ToStringMode::Simple =>
                format!("{} -> {}", self.representation.to_string(), self.fitness),
            ToStringMode::Default =>
                format!(
                    "Individual(representation={}, fitness={})",
                    self.representation.to_string(),
                    self.fitness
                )
        }
    }
}

impl<T, F, R> Verifiable for Individual<T, F, R>
where
    F: Feature<T, F>,
    R: Representation<T, F>,
{
    fn verify(&self) -> bool {
        self.representation.verify()
    }
}

impl<T, F, R> FlatMappable<T> for Individual<T, F, R>
where
    F: Feature<T, F>,
    R: Representation<T, F>,
{
    fn flatten(&self) -> T {
        self.representation.flatten()
    }
}

impl<T, F, R> Foldable<T> for Individual<T, F, R>
where
    F: Feature<T, F>,
    R: Representation<T, F>
{
    fn fold<R>(&self, initial: R, f: fn(R, T) -> R) -> R {
        self.representation.fold(initial, f)
    }
}

impl<T, F, R> PartialEq for Individual<T, F, R>
where
    F: Feature<T, F>,
    R: Representation<T, F>
{
    fn eq(&self, other: &Self) -> bool {
        self.representation.eq(&other.representation)
    }
}

impl<T, F, R> Eq for Individual<T, F, R>
where
    F: Feature<T, F>,
    R: Representation<T, F>
{
}

impl<T, F, R> Hash for Individual<T, F, R>
where
    F: Feature<T, F>,
    R: Representation<T, F>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        crate::domain::hash(&[&self.representation, &self.fitness]);
    }
}
