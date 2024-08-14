use std::vec::IntoIter;
use crate::genetics::genes::gene::Gene;
use crate::mixins::flat_mappable::FlatMappable;
use crate::mixins::foldable::Foldable;
use crate::repr::representation::Representation;

pub trait Chromosome<T, G>: Representation<T, G> + Sized
where
    T: Clone,
    G: Gene<T, G> + PartialEq,
{
    fn genes(&self) -> Vec<G>;

    fn size(&self) -> usize {
        self.genes().len()
    }

    fn is_empty(&self) -> bool {
        self.genes().is_empty()
    }

    fn contains_all(&self, other: &[G]) -> bool {
        other.iter().all(|gene| self.genes().contains(gene))
    }

    fn contains(&self, gene: &G) -> bool {
        self.genes().contains(gene)
    }

    fn flatten(&self) -> Vec<T> {
        self.genes().iter().flat_map(|gene| gene.flatten()).collect()
    }

    fn is_valid(&self) -> bool {
        self.genes().iter().all(|gene| gene.is_valid())
    }
}
