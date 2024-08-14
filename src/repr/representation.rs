use crate::mixins::flat_mappable::FlatMappable;
use crate::mixins::foldable::Foldable;
use crate::mixins::verifiable::Verifiable;
use crate::repr::feature::Feature;

pub trait Representation<T, F>: Verifiable + FlatMappable<T> + Foldable<T>
where
    T: Clone,
    F: Feature<T, F>,
{
    fn size(&self) -> usize;
}
