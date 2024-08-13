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
    _t: std::marker::PhantomData<T>,
    _f: std::marker::PhantomData<F>,
}

impl<T, F, R> Individual<T, F, R>
where
    F: Feature<T, F>,
    R: Representation<T, F>,
{
    pub fn new(representation: R) -> Self {
        Individual {
            representation,
            fitness: f64::NAN,
            _t: Default::default(),
            _f: Default::default(),
        }
    }

    pub fn size(&self) -> usize {
        self.representation.size()
    }

    pub fn is_evaluated(&self) -> bool {
        !self.fitness.is_nan()
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
    fn fold<B>(&self, initial: B, f: fn(B, T) -> B) -> B {
        self.representation.fold(initial, f)
    }
}

impl<T, F, R> PartialEq for Individual<T, F, R>
where
    F: Feature<T, F>,
    R: Representation<T, F> + PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.representation == other.representation
            && self.fitness == other.fitness
    }
}

impl<T, F, R> Eq for Individual<T, F, R>
where
    F: Feature<T, F>,
    R: Representation<T, F> + Eq
{
}
